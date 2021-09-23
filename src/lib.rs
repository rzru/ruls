use std::fs::{DirEntry, symlink_metadata, read_dir};
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::{Write, stdout};
use is_executable::IsExecutable;
use termion::{color, style};
use termion::color::Color;

pub fn run(config: Config) -> () {
    match read_dir(config.path) {
        Ok(entries) => {
            let mut dir_entries: Vec<DirEntry> = vec![];
            for entry in entries {
                if let Ok(entry) = entry {
                    dir_entries.push(entry);
                }
            }

            dir_entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            process(&dir_entries, &config.arguments);
        },
        Err(e) => eprintln!("Error reading {}: {}", config.path, e),
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Argument {
    ShowAll
}

#[derive(Debug)]
pub struct Arguments {
    data: Vec<Argument>
}

pub struct Config<'a> {
    arguments: Arguments,
    path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(path: &'a str, arguments: Arguments) -> Self {
        Self {
            path,
            arguments,
        }
    }
}

impl Arguments {
    pub fn data(&self) -> &Vec<Argument> {
        &self.data
    }

    fn prepare_arguments() -> HashMap<String, Argument> {
        let mut arguments_mapping = HashMap::new();
        arguments_mapping.insert("a".to_string(), Argument::ShowAll);
        arguments_mapping.insert("all".to_string(), Argument::ShowAll);

        arguments_mapping
    }
}

impl From<&Vec<String>> for Arguments {
    fn from(vec: &Vec<String>) -> Self {
        let mut data: Vec<Argument> = vec![];
        let args_map = Arguments::prepare_arguments();
        for arg in vec {
            if arg.starts_with("--") {
                if let Some(map_arg) = args_map.get(&arg[2..].to_string()) {
                    data.push(map_arg.clone())
                }
                continue;
            }
            if arg.starts_with('-') {
                for str in arg[1..].split("") {
                    if let Some(map_arg) = args_map.get(&str.to_string()) {
                        data.push(map_arg.clone())
                    }
                }
            }
        }

        Arguments {
            data
        }
    }
}

pub trait ExtendPathInfo {
    fn is_symlink_local(&self) -> bool;
    fn is_in_array(&self, values: Vec<&str>) -> bool;
    fn is_image(&self) -> bool;
    fn is_archive(&self) -> bool;
}

impl ExtendPathInfo for PathBuf {
    fn is_symlink_local(&self) -> bool {
        if let Ok(metadata) = symlink_metadata(self) {
            let file_type = metadata.file_type();

            return file_type.is_symlink();
        }

        false
    }

    fn is_in_array(&self, values: Vec<&str>) -> bool {
        if let Some(extension) = self.extension() {
            if let Some(extension) = extension.to_str() {
                let extension = &extension.to_lowercase()[..];
                return values.contains(&extension)
            }
        }

        false
    }

    fn is_image(&self) -> bool {
        let image_extensions = vec!["tif", "tiff", "jpg", "bmp", "svg", "bmp", "jpeg", "gif", "eps", "webp", "psd", "raw", "heif", "indd", "ai"];
        self.is_in_array(image_extensions)
    }

    fn is_archive(&self) -> bool {
        let archive_extensions = vec!["7z", "zip", "rar", "bz2", "gz", "deb", "gzip"];
        self.is_in_array(archive_extensions)
    }
}

pub trait Print {
    fn print_dir(&self, name: &str) -> ();

    fn print_exec(&self, name: &str) -> ();

    fn print_symlink(&self, name: &str) -> ();

    fn print_image(&self, name: &str) -> ();

    fn print_archive(&self, name: &str) -> ();

    fn default_print(&self, name: &str) -> ();
}

pub struct Printer;

impl Printer {
    pub fn new() -> Self {
        Self
    }

    fn color_format(&self, name: &str, color: impl Color) -> String {
        format!("{}{}{}{} ", style::Bold, color::Fg(color), name, style::Reset)
    }

    fn color_print(&self, name: &str, color: impl Color) -> () {
        print!("{}", self.color_format(name, color))
    }
}

impl Print for Printer {
    fn print_dir(&self, name: &str) -> () {
        self.color_print(name, color::Blue)
    }

    fn print_exec(&self, name: &str) -> () {
        self.color_print(name, color::Green)
    }

    fn print_symlink(&self, name: &str) -> () {
        self.color_print(name, color::Cyan)
    }

    fn print_image(&self, name: &str) -> () {
        self.color_print(name, color::Magenta)
    }

    fn print_archive(&self, name: &str) -> () {
        self.color_print(name, color::Red)
    }

    fn default_print(&self, name: &str) -> () {
        print!("{} ", name)
    }
}