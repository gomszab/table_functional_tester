mod generated_assets;

use headless_chrome::{Browser, FetcherOptions, LaunchOptionsBuilder, protocol::cdp};
use serde_json::Value;
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

        let onlytype = if let Some(onlytype) = args.next() {
            onlytype
        } else {
            String::new()
        };

        let sumtestcount = get_testcases_map().len();
        let mut success_test_count = 0;
        let mut ignored_test_count = 0;
        let mut errors = Vec::new();

        for (key, value) in get_testcases_map() {
            if onlytype != value.typ {
                ignored_test_count += 1;
                continue;
            }
            let content_of_id = get_content_from_bytes(ID_CONFIG);
            tab.evaluate(&content_of_id, false).unwrap();

            tab.wait_until_navigated().unwrap();

            let script = get_content_from_bytes(&value.content);
            println!("[{}]{}", value.typ, value.details);
            let result = tab.evaluate(&script, true).unwrap();
            // println!("{:?}", result.value);

            let test_passed = result
                .value
                .and_then(|v| return Some(TESTRESULT::from(v)))
                .unwrap_or(TESTRESULT::FAILED(String::from("Ismeretlen hiba")));
            match test_passed {
                TESTRESULT::IGNORED => {
                    println!("Teszt ignorált.");
                    ignored_test_count += 1;
                }
                TESTRESULT::FAILED(msg) => {
                    println!("{}", msg);
                    errors.push(format!(
                        "{} azonosítójú teszt: [{}] {}\n{}",
                        key, value.typ, value.details, msg
                    ));
                }
                TESTRESULT::SUCCESS => {
                    println!("Teszt sikeres.");
                    success_test_count += 1;
                }
            };
            tab.reload(true, None).unwrap();
        }
        println!(
            "\n*********************************************************\n{} tesztből {} sikeres, {} sikertelen, {} kihagyott\n*********************************************************\n",
            sumtestcount,
            success_test_count,
            errors.len(),
            ignored_test_count
        );
        if errors.len() > 1 {
            println!("A hibák a következők:");
            for error in errors {
                println!("\n{}", error);
            }
        }
    }
    Ok(())
}

enum TESTRESULT {
    SUCCESS,
    IGNORED,
    FAILED(String),
}

impl From<Value> for TESTRESULT {
    fn from(value: Value) -> Self {
        if let Ok(value) = serde_json::from_str::<Value>(
            value.as_str().unwrap_or("{result: false, ignored: false}"),
        ) {
            let result = value["result"].as_bool().unwrap_or(false);
            let message = value["message"].as_str().unwrap_or("Ismeretlen hiba");
            let ignored = value["ignored"].as_bool().unwrap_or(false);

            if ignored {
                Self::IGNORED
            } else if result {
                Self::SUCCESS
            } else {
                Self::FAILED(String::from(message))
            }
        } else {
            Self::FAILED(String::from("Ismeretlen hiba"))
        }
    }
}

fn get_content_from_bytes(content: &'static [u8]) -> String {
    match str::from_utf8(content) {
        Ok(string_content) => String::from(string_content),
        Err(_) => panic!("Nem sikerült a string konverzió"),
    }
}
