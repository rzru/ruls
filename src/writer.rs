use std::io::{Write, stdout};
use std::fs::{DirEntry};
use super::arguments::{Arguments, Argument};
use termion::{color, style};
use is_executable::IsExecutable;

pub struct Writer;

impl Writer {
    pub fn write(dir_entries: &Vec<DirEntry>, args: &Arguments) -> () {
        for entry in dir_entries {
            match entry.file_name().to_str() {
                Some(file_name) => {
                    if !file_name.starts_with('.') || args.data().contains(&Argument::ShowAll) {
                        let path = entry.path();

                        if path.is_dir() {
                            print!("{}{}{}{} ", style::Bold, color::Fg(color::Blue), file_name, style::Reset);
                        } else if path.is_executable() {
                            print!("{}{}{}{} ", style::Bold, color::Fg(color::Red), file_name, style::Reset);
                        } else {
                            print!("{} ", file_name);
                        }
                    }
                    stdout().flush().unwrap();
                }
                None => {}
            }
        }
    }
}
