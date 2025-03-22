# DDPClassicalFix

Automatically fixes a small issue where REAPER does not quite correctly add the "Classical" genre to a DDP file set.

Usage:

`DDPClassicalFix <DDP_FOLDER>`

It does the following automatically (and in a split second!):

1. Removes the extra `0x87` packet
2. Changes the file size of CDTEXT.BIN in DDPMS
3. Calculates new checksums for both CDTEXT.BIN and DDPMS
4. Adds checksums to checksums.md5

