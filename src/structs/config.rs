use super::arguments::Arguments;

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

    pub fn path(&self) -> &'a str {
        self.path
    }

    pub fn arguments(&self) -> &Arguments {
        &self.arguments
    }
}