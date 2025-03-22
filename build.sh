cargo build --release && mv target/release/DDPClassicalFix target/release/DDPClassicalFix_Linux_X86_64 &&
cargo build --release --target i686-unknown-linux-gnu && mv target/i686-unknown-linux-gnu/release/DDPClassicalFix target/i686-unknown-linux-gnu/release/DDPClassicalFix_Linux_i686 &&
cargo zigbuild --release --target aarch64-unknown-linux-gnu && mv target/aarch64-unknown-linux-gnu/release/DDPClassicalFix target/aarch64-unknown-linux-gnu/release/DDPClassicalFix_Linux_aarch64 &&
cargo zigbuild --release --target armv7-unknown-linux-gnueabihf && mv target/armv7-unknown-linux-gnueabihf/release/DDPClassicalFix target/armv7-unknown-linux-gnueabihf/release/DDPClassicalFix_Linux_armv7 &&
cargo build --release --target x86_64-pc-windows-gnu && mv target/x86_64-pc-windows-gnu/release/DDPClassicalFix.exe target/x86_64-pc-windows-gnu/release/DDPClassicalFix_x64.exe &&
cargo build --release --target i686-pc-windows-gnu
cargo zigbuild --release --target x86_64-apple-darwin && mv target/x86_64-apple-darwin/release/DDPClassicalFix target/x86_64-apple-darwin/release/DDPClassicalFix_Darwin_x86_64 &&
cargo zigbuild --release --target aarch64-apple-darwin && mv target/aarch64-apple-darwin/release/DDPClassicalFix target/aarch64-apple-darwin/release/DDPClassicalFix_Darwin_aarch64