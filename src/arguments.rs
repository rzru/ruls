use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Argument {
    ShowAll
}

#[derive(Debug)]
pub struct Arguments {
    data: Vec<Argument>
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

impl From<Vec<String>> for Arguments {
    fn from(vec: Vec<String>) -> Self {
        let mut data: Vec<Argument> = vec![];
        let args_map = Arguments::prepare_arguments();
        for arg in vec {
            if arg.starts_with("--") {
                match args_map.get(&arg[2..].to_string()) {
                    Some(map_arg) => data.push(map_arg.clone()),
                    None => {}
                }
                continue;
            }
            if arg.starts_with('-') {
                for str in arg[1..].split("") {
                    match args_map.get(&str.to_string()) {
                        Some(map_arg) => data.push(map_arg.clone()),
                        None => {}
                    }
                }
            }
        }

        Arguments {
            data
        }
    }
}
