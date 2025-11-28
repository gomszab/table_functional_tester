use std::{
    env::Args,
    fs,
    path::{Path, PathBuf},
};

use crate::{resultutil::ResultUtil, test_config::TestConfig};
use generated::generated_assets::{ID_CONFIG, get_api_files, get_testcases_map};
use headless_chrome::{Browser, LaunchOptions, protocol::cdp};
use serde_json::Value;

pub fn run_tests(options: LaunchOptions<'_>, param: String, mut args: Args) {
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

    tab.navigate_to(&format!("file:///{param}")).unwrap();

    let content_of_id = get_content_from_bytes(ID_CONFIG);
    let content_of_id = parse_config(content_of_id, param);
    let onlytype = args.next().unwrap_or_default();

    let test_cases_map = get_testcases_map();
    let mut result_util = ResultUtil::new(test_cases_map.len());

    for (key, value) in test_cases_map {
        if !onlytype.is_empty() && onlytype != value.typ {
            result_util.increment_ignored();
            continue;
        }

        tab.evaluate(&content_of_id, false).unwrap();

        tab.wait_until_navigated().unwrap();

        let script = get_content_from_bytes(value.content);
        println!("[{}]{}", value.typ, value.details);
        let result = tab.evaluate(&script, true).unwrap();

        let test_passed = result
            .value.map(Testresult::from)
            .unwrap_or(Testresult::Failed(String::from("Ismeretlen hiba")));
        match test_passed {
            Testresult::Ignored => {
                println!("Teszt ignorált.");
                result_util.increment_ignored();
            }
            Testresult::Failed(msg) => {
                println!("{msg}");
                result_util.push_error(format!(
                    "{} azonosítójú teszt: [{}] {}\n{}",
                    key, value.typ, value.details, msg
                ));
            }
            Testresult::Success => {
                println!("Teszt sikeres.");
                result_util.increment_success();
            }
        };
        tab.reload(true, None).unwrap();
    }
    println!("{result_util}");
}

fn get_content_from_bytes(content: &'static [u8]) -> String {
    match str::from_utf8(content) {
        Ok(string_content) => String::from(string_content),
        Err(_) => panic!("Nem sikerült a string konverzió"),
    }
}

enum Testresult {
    Success,
    Ignored,
    Failed(String),
}

impl From<Value> for Testresult {
    fn from(value: Value) -> Self {
        if let Ok(value) = serde_json::from_str::<Value>(
            value.as_str().unwrap_or("{result: false, ignored: false}"),
        ) {
            let result = value["result"].as_bool().unwrap_or(false);
            let message = value["message"].as_str().unwrap_or("Ismeretlen hiba");
            let ignored = value["ignored"].as_bool().unwrap_or(false);
            if ignored {
                Self::Ignored
            } else if result {
                Self::Success
            } else {
                Self::Failed(String::from(message))
            }
        } else {
            Self::Failed(String::from("Ismeretlen hiba"))
        }
    }
}

fn parse_config(mut idconfig: String, html_path: String) -> String {
    let html_path = Path::new(&html_path);
    let dir = html_path.parent().expect("path has no parent directory");
    let config_path: PathBuf = dir.join("config.json");
    let config_json = fs::read_to_string(config_path)
        .expect("Nem lehet beolvasni a json file-t az index mellett");
    let config: TestConfig = serde_json::from_str(&config_json).expect("nem jó a json file");

    config.wrap_id(idconfig)

}
