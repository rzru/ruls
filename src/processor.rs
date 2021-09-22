use std::io::{Write, stdout};
use std::fs::{DirEntry};
use super::arguments::{Arguments, Argument};
use is_executable::IsExecutable;
use crate::printer::{Printer, Print};
use super::path_buf::ExtendPathInfo;

pub struct Processor;

impl Processor {
    pub fn process(dir_entries: &Vec<DirEntry>, args: &Arguments) -> () {
        let printer = Printer::new();

        for entry in dir_entries {
            if let Some(file_name) = entry.file_name().to_str() {
                if !file_name.starts_with('.') || args.data().contains(&Argument::ShowAll) {
                    let path = entry.path();

                    if path.is_dir() {
                        printer.print_dir(file_name);
                    } else if path.is_executable() {
                        printer.print_exec(file_name);
                    } else if path.is_symlink_local() {
                        printer.print_symlink(file_name);
                    } else if path.is_image() {
                        printer.print_image(file_name);
                    } else if path.is_archive() {
                        printer.print_archive(file_name)
                    } else {
                        printer.default_print(file_name);
                    }
                }
                stdout().flush().unwrap();
            }
        }
    }
}
