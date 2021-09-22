use reader::Reader;
use std::env;
mod reader;
mod arguments;
mod processor;
mod printer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from(".");
    let mut path = default_path;

    if let Some(first_arg) = args.get(1) {
        if !first_arg.starts_with('-') {
            path = first_arg.to_string()
        }
    }

    Reader::read(path, args.into());
}