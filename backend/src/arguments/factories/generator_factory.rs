
use crate::arguments::{argument_parser::Mode, factories::CommandFactory};

pub struct GeneratorFactory;

impl CommandFactory for GeneratorFactory {
    fn try_parse(&self, args: &Vec<String>) -> Option<Mode> {
        if args.len() == 1 && args[0] == "generator".to_string() {
           return Some(Mode::Generate);
        }
        None
    }
}