use std::io::{Seek, SeekFrom};
use tempfile::tempfile;
use std::io::Write;
use std::fs::File;

// Create temp file and seek it so you can read without closing it
// Like this it is not possible to read multiple times witout re-seeking
pub fn temp_file(content: &[u8]) -> File {
    let mut file = tempfile().expect("Test setup error.");
    file.write_all(content).expect("Test setup error.");
    file.seek(SeekFrom::Start(0)).expect("Test setup error.");
    file
}
