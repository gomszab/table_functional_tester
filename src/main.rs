mod generated_assets;

use headless_chrome::{Browser, FetcherOptions, LaunchOptionsBuilder, protocol::cdp};
use std::env;

use crate::generated_assets::{ID_CONFIG, get_api_files, get_testcases_map};
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

    let mut args = env::args();
    args.next();
    if let Some(path) = args.next() {
        let browser = Browser::new(options).unwrap();
        println!("{:?}", path);
        let tab = browser.new_tab().unwrap();
        for value in get_api_files() {
            let content = get_content_from_bytes(value);
            tab.call_method(cdp::Page::AddScriptToEvaluateOnNewDocument {
                source: content,
                world_name: None,
                include_command_line_api: None,
                run_immediately: Some(true),
            })
            .unwrap();
        }

        tab.navigate_to(&format!("file:///{}", path)).unwrap();

        for (_key, value) in get_testcases_map() {
            if value.typ != "checkbox" {
                continue;
            }
            let content_of_id = get_content_from_bytes(ID_CONFIG);
            tab.evaluate(&content_of_id, false).unwrap();

            tab.wait_until_navigated().unwrap();

            let script = get_content_from_bytes(&value.content);
            println!("[{}]{}", value.typ, value.details);
            let result = tab.evaluate(&script, true).unwrap();
            // println!("{:?}", result);
            match result.subtype {
                Some(_error) => {
                    if let Some(description) = result.description {
                        let (err, _stack) = description
                            .split_once("\n")
                            .unwrap_or(("Ismeretlen hiba", ""));
                        println!("{}", err);
                    }
                }
                None => {
                    let test_passed = result.value.and_then(|v| v.as_bool()).unwrap_or(false);
                    println!("teszt passed: {}", test_passed);
                }
            };

            tab.reload(true, None).unwrap();
        }
    }
    Ok(())
}

fn get_content_from_bytes(content: &'static [u8]) -> String {
    match str::from_utf8(content) {
        Ok(string_content) => String::from(string_content),
        Err(_) => panic!("Nem sikerült a string konverzió"),
    }
}
