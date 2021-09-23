use termion::{color, style};
use termion::color::Color;

pub trait Print {
    fn print_dir(&self) -> ();

    fn print_exec(&self) -> ();

    fn print_symlink(&self) -> ();

    fn print_image(&self) -> ();

    fn print_archive(&self) -> ();

    fn default_print(&self) -> ();

    fn color_format(&self, color: impl Color) -> String;

    fn color_print(&self, color: impl Color) -> ();
}

impl Print for &str {
    fn print_dir(&self) -> () {
        self.color_print(color::Blue)
    }

    fn print_exec(&self) -> () {
        self.color_print(color::Green)
    }

    fn print_symlink(&self) -> () {
        self.color_print(color::Cyan)
    }

    fn print_image(&self) -> () {
        self.color_print(color::Magenta)
    }

    fn print_archive(&self) -> () {
        self.color_print(color::Red)
    }

    fn default_print(&self) -> () {
        print!("{} ", self)
    }

    fn color_format(&self, color: impl Color) -> String {
        format!("{}{}{}{} ", style::Bold, color::Fg(color), self, style::Reset)
    }

    fn color_print(&self, color: impl Color) -> () {
        print!("{}", self.color_format(color))
    }
}
