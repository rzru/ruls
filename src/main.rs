use reader::Reader;
use std::env;
mod reader;
mod arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from(".");
    let path = args.get(1).unwrap_or(&default_path).to_string();

    Reader::read(path, args.into());
}