use termion::{color, style};
use termion::color::Color;

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

    fn color_print(&self, name: &str, color: impl Color) -> () {
        print!("{}{}{}{} ", style::Bold, color::Fg(color), name, style::Reset)
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