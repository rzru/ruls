use std::fs;
use std::io;
use std::io::Write;
use std::fs::DirEntry;
use super::arguments::{Arguments, Argument};

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

            for entry in dir_entries {
                match entry.file_name().to_str() {
                    Some(file_name) => {
                        if !file_name.starts_with('.') || args.data().contains(&Argument::ShowAll) {
                            print!("{} ", file_name);
                        }
                        io::stdout().flush().unwrap();
                    }
                    None => {}
                }
            }
        }
    }
}
