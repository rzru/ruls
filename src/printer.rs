use termion::{color, style};

pub trait Print {
    fn print_dir(&self, name: &str) -> ();

    fn print_exec(&self, name: &str) -> ();

    fn default_print(&self, name: &str) -> ();
}

pub struct Printer;

impl Printer {
    pub fn new() -> Self {
        Self
    }
}

impl Print for Printer {
    fn print_dir(&self, name: &str) -> () {
        print!("{}{}{}{} ", style::Bold, color::Fg(color::Blue), name, style::Reset)
    }

    fn print_exec(&self, name: &str) -> () {
        print!("{}{}{}{} ", style::Bold, color::Fg(color::Red), name, style::Reset)
    }

    fn default_print(&self, name: &str) -> () {
        print!("{} ", name)
    }
}