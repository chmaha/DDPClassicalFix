use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use regex::Regex;
use md5;

fn get_file_size(filepath: &str) -> std::io::Result<u64> {
    Ok(fs::metadata(filepath)?.len())
}

fn fix_cdtext_bin(cdtext_path: &str) -> std::io::Result<Option<u64>> {
    let mut file = File::open(cdtext_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut positions = vec![];
    let mut i = 0;

    while i + 18 <= data.len() {
        if data[i] == 0x87 {
            positions.push(i);
        }
        i += 1;
    }

    println!("Found {} 0x87 packets at positions: {:?}", positions.len(), positions);

    if positions.len() > 1 {
        let first_packet_pos = positions[0];
        let second_packet_pos = positions[1];

        let first_packet_end = first_packet_pos + 18;
        if first_packet_end <= data.len() {
            let first_packet_content = &data[first_packet_pos..first_packet_end];
            let first_packet_string = String::from_utf8_lossy(first_packet_content);

            if first_packet_string.contains("Classical") {
                println!("First 0x87 packet contains 'Classical', removing second at position {}...", second_packet_pos);
                data.drain(second_packet_pos..second_packet_pos + 18);

                let mut file = File::create(cdtext_path)?;
                file.write_all(&data)?;
                println!("Packet successfully removed.");

                return get_file_size(cdtext_path).map(Some);
            } else {
                println!("First 0x87 packet does not contain 'Classical', skipping removal.");
            }
        }
    }

    Ok(None) // No changes made
}

fn update_cdtext_size_in_descriptor(ddp_folder: &str, new_size: u64) -> std::io::Result<()> {
    let ddp_path = Path::new(ddp_folder);
    let mut descriptor_path = None;

    for entry in fs::read_dir(ddp_path)? {
        let file_path = entry?.path();
        if file_path.is_file() {
            let content = fs::read_to_string(&file_path);
            if let Ok(content) = content {
                if content.contains("CDTEXT") {
                    descriptor_path = Some(file_path);
                    break;
                }
            }
        }
    }

    let descriptor_path = match descriptor_path {
        Some(path) => path,
        None => {
            println!("Descriptor file not found! Skipping update.");
            return Ok(());
        }
    };

    let content = fs::read_to_string(&descriptor_path)?;
    let re = Regex::new(r"(\d{6})(\s+CDTEXT)").unwrap();
    let modified_content = re.replace(&content, |caps: &regex::Captures| {
        format!("{:06}{}", new_size, &caps[2])
    });

    if content != modified_content {
        fs::write(&descriptor_path, modified_content.as_bytes())?;
        println!("Updated CDTEXT size in {:?}: {}", descriptor_path, new_size);
    } else {
        println!("CDTEXT size update failed! No match found.");
    }

    Ok(())
}

fn calculate_md5(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut hasher = md5::Context::new();
    let mut buffer = [0; 4096];

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        hasher.consume(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.compute()))
}

fn update_md5_checksum(ddp_folder: &str) -> std::io::Result<()> {
    let checksum_path = format!("{}/checksum.md5", ddp_folder);
    let checksum_path = Path::new(&checksum_path);

    if !checksum_path.exists() {
        println!("checksum.md5 not found! Skipping update.");
        return Ok(());
    }

    let mut file = File::open(checksum_path)?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    let mut checksums = std::collections::HashMap::new();
    for filename in ["CDTEXT.BIN", "DDPMS"] {
        let filepath = format!("{}/{}", ddp_folder, filename);
        if Path::new(&filepath).exists() {
            if let Ok(md5_hash) = calculate_md5(&filepath) {
                checksums.insert(filename, md5_hash);
            }
        }
    }

    let mut new_lines = String::new();
    for line in lines.lines() {
        let mut updated = false;
        for (filename, new_hash) in &checksums {
            if line.contains(&format!(" *{}", filename)) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(old_hash) = parts.get(0) {
                    println!("Updated MD5 for {}: {} → {}", filename, old_hash, new_hash);
                    new_lines.push_str(&format!("{} *{}\n", new_hash, filename));
                    updated = true;
                    break;
                }
            }
        }
        if !updated {
            new_lines.push_str(line);
            new_lines.push('\n');
        }
    }

    let mut file = File::create(checksum_path)?;
    file.write_all(new_lines.as_bytes())?;

    Ok(())
}

fn process_ddp_folder(ddp_folder: &str) -> std::io::Result<Option<()>> {
    let cdtext_path = format!("{}/CDTEXT.BIN", ddp_folder);

    if !Path::new(&cdtext_path).exists() {
        println!("CDTEXT.BIN not found! Skipping cleanup.");
        return Ok(None);
    }

    let new_cdtext_size = match fix_cdtext_bin(&cdtext_path)? {
        Some(size) => size,
        None => return Ok(None), // No changes → Exit early
    };

    update_cdtext_size_in_descriptor(ddp_folder, new_cdtext_size)?;
    update_md5_checksum(ddp_folder)?;

    Ok(Some(()))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: DDPClassicalFix <DDP_FOLDER>");
        std::process::exit(1);
    }

    let ddp_folder = &args[1];

    if !Path::new(ddp_folder).is_dir() {
        println!("Error: Folder '{}' not found.", ddp_folder);
        std::process::exit(1);
    }

    println!("Processing DDP folder: {}", ddp_folder);
    match process_ddp_folder(ddp_folder) {
        Ok(Some(())) => {
            println!("Changes were made.");
        }
        Ok(None) => {
            println!("No changes were necessary. Exiting.");
            std::process::exit(0); // Exit early
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    println!("Now checking checksums...");
}
