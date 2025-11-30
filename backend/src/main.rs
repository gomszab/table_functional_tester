mod config_generator;
mod resultutil;
mod test_config;
mod test_runner;
mod arguments;

use headless_chrome::{FetcherOptions, LaunchOptionsBuilder};
use std::{
    env
};

use crate::{arguments::{ Mode, pars_args}, config_generator::run_generator, test_runner::run_tests};
fn main() -> Result<(), failure::Error> {
    let options = LaunchOptionsBuilder::default()
        .fetcher_options(
            FetcherOptions::default()
                .with_allow_download(true)
                .with_allow_standard_dirs(true)
                .with_install_dir(Some("..\\headless_chrome")),
        )
        .headless(true)
        .build()
        .unwrap();

    let  args: Vec<String> = env::args().skip(1).collect();
    let mode = pars_args(args);
    match mode {
        Ok(Mode::RunTest(parm)) => run_tests(options, parm),
        Ok(Mode::Generate) => run_generator(),
        _ => println!("Mi történt?")
    }
    // if let Some(param) = args.next() {
    //     if param == "generator" {
    //         run_generator();
    //     } else {
    //         let path = Path::new(&param);

    //         let path = if path.exists() {
    //             std::fs::canonicalize(path).expect("Nem található a fájl (rossz útvonal?)")
    //         } else {
    //             std::path::absolute(path).expect("Nem található a fájl (rossz útvonal?)")
    //         };

    //         let path = if let Some(stripped) = path.to_str().and_then(|s| s.strip_prefix(r"\\?\")) {
    //             PathBuf::from(stripped)
    //         } else {
    //             path
    //         };
    //         run_tests(options, path.into_os_string().into_string().unwrap(), args);
    //     }
    // }
    Ok(())
}
