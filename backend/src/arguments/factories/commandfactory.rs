use crate::arguments::argument_parser::Mode;

pub trait CommandFactory{
    fn try_parse(&self, args: &Vec<String>) -> Option<Mode>;
}