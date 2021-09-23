use std::env;
use ruls::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = ".";

    if let Some(first_arg) = args.get(1) {
        if !first_arg.starts_with('-') {
            path = first_arg
        }
    }

    run(Config::new(path, (&args).into()))
}