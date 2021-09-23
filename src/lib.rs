use std::fs::{DirEntry, symlink_metadata, read_dir};
use is_executable::IsExecutable;
use crate::structs::{arguments::{Arguments, Argument}, path_buf::ExtendPathInfo, printer::Print};
use std::io::{Write, stdout};

mod structs;
pub use crate::structs::config::Config;

pub fn run(config: Config) -> () {
    match read_dir(config.path()) {
        Ok(entries) => {
            let mut dir_entries: Vec<DirEntry> = vec![];
            for entry in entries {
                if let Ok(entry) = entry {
                    dir_entries.push(entry);
                }
            }

            dir_entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            process(&dir_entries, config.arguments());
        }
        Err(e) => eprintln!("Error reading {}: {}", config.path(), e),
    }
}

pub fn process(dir_entries: &Vec<DirEntry>, args: &Arguments) -> () {
    for entry in dir_entries {
        if let Some(file_name) = entry.file_name().to_str() {
            if !file_name.starts_with('.') || args.data().contains(&Argument::ShowAll) {
                let path = entry.path();

                if path.is_dir() {
                    file_name.print_dir();
                } else if path.is_executable() {
                    file_name.print_exec();
                } else if path.is_symlink_local() {
                    file_name.print_symlink();
                } else if path.is_image() {
                    file_name.print_image();
                } else if path.is_archive() {
                } else {
                    file_name.default_print();
                }
            }
            stdout().flush().unwrap();
        }
    }
}




