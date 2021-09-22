use std::fs;
use std::fs::{DirEntry};
use super::arguments::{Arguments};
use super::writer::Writer;

pub struct Reader;

impl Reader {
    pub fn read(path: String, args: Arguments) -> () {
        if let Ok(entries) = fs::read_dir(path) {
            let mut dir_entries: Vec<DirEntry> = vec![];
            for entry in entries {
                if let Ok(entry) = entry {
                    dir_entries.push(entry);
                }
            }

            dir_entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            Writer::write(&dir_entries, &args);
        }
    }
}
