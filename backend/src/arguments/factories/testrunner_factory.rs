use std::path::{Path, PathBuf};

use crate::arguments::factories::CommandFactory;
use crate::arguments::argument_parser::{Mode, RunMode};

pub struct TestRunnerFactory;

impl CommandFactory for TestRunnerFactory{
    fn try_parse(&self, args: &Vec<String>) -> Option<Mode> {
        if args.len() == 1 {
           return Some(Mode::RunTest(RunMode::new(parse_path(&args[0]))));
        } else if args.len() == 2 {
            let file_path = parse_path(&args[0]);
            let tag = args[1].clone();
            let runmode = RunMode::new(file_path).withtag(tag);

            return Some(Mode::RunTest(runmode));
        } else if args.len() == 3 {
            // let file_path = parse_path(&args[0]);
            // let tag = args[1].clone();
            // let runmode = RunMode::new(file_path).withtag(tag);

            // return Some(Mode::RunTest(runmode));
        }

        None
    }
}

fn parse_path(arg: &String) -> String{
     let path = Path::new(arg);

    let path = if path.exists() {
        std::fs::canonicalize(path).expect("Nem található a fájl (rossz útvonal?)")
    } else {
        std::path::absolute(path).expect("Nem található a fájl (rossz útvonal?)")
    };

    let path = if let Some(stripped) = path.to_str().and_then(|s| s.strip_prefix(r"\\?\")) {
        PathBuf::from(stripped)
    } else {
        path
    };

    path.into_os_string().into_string().unwrap()
}