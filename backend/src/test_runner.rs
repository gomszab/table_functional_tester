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

    tab.navigate_to(&format!("file:///{}", param)).unwrap();

    let content_of_id = get_content_from_bytes(ID_CONFIG);
    let content_of_id = parse_config(content_of_id, param);
    let onlytype = if let Some(onlytype) = args.next() {
        onlytype
    } else {
        String::new()
    };

    let test_cases_map = get_testcases_map();
    let mut result_util = ResultUtil::new(test_cases_map.len());

    for (key, value) in test_cases_map {
        if !onlytype.is_empty() && onlytype != value.typ {
            result_util.increment_ignored();
            continue;
        }

        tab.evaluate(&content_of_id, false).unwrap();

        tab.wait_until_navigated().unwrap();

        let script = get_content_from_bytes(&value.content);
        println!("[{}]{}", value.typ, value.details);
        let result = tab.evaluate(&script, true).unwrap();

        let test_passed = result
            .value
            .and_then(|v| return Some(TESTRESULT::from(v)))
            .unwrap_or(TESTRESULT::FAILED(String::from("Ismeretlen hiba")));
        match test_passed {
            TESTRESULT::IGNORED => {
                println!("Teszt ignorált.");
                result_util.increment_ignored();
            }
            TESTRESULT::FAILED(msg) => {
                println!("{}", msg);
                result_util.push_error(format!(
                    "{} azonosítójú teszt: [{}] {}\n{}",
                    key, value.typ, value.details, msg
                ));
            }
            TESTRESULT::SUCCESS => {
                println!("Teszt sikeres.");
                result_util.increment_success();
            }
        };
        tab.reload(true, None).unwrap();
    }
    println!("{}", result_util);
}

fn get_content_from_bytes(content: &'static [u8]) -> String {
    match str::from_utf8(content) {
        Ok(string_content) => String::from(string_content),
        Err(_) => panic!("Nem sikerült a string konverzió"),
    }
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

fn parse_config(mut idconfig: String, html_path: String) -> String {
    let html_path = Path::new(&html_path);
    let dir = html_path.parent().expect("path has no parent directory");
    let config_path: PathBuf = dir.join("config.json");
    let config_json = fs::read_to_string(config_path)
        .expect("Nem lehet beolvasni a json file-t az index mellett");
    let config: TestConfig = serde_json::from_str(&config_json).expect("nem jó a json file");

    idconfig = idconfig.replace("ROWTABLEBODY", &config.rowtablebody);
    idconfig = idconfig.replace("COLTABLEBODY", &config.coltablebody);
    idconfig = idconfig.replace("COLFORM", &config.colform);
    idconfig = idconfig.replace("ROWFORM", &config.rowform);

    idconfig = idconfig.replace(
        "DEFAULTVISIBLE",
        &config
            .default_visible
            .clone()
            .unwrap_or_else(|| "undefined".to_string()),
    );

    idconfig = idconfig.replace(
        "HASCHECKBOX",
        if config.has_checkbox { "true" } else { "false" },
    );

    idconfig = idconfig.replace(
        "CHECKBOXID",
        &config.checkbox_id.clone().unwrap_or_else(|| "".to_string()),
    );

    idconfig = idconfig.replace(
        "CHECKBOXDEFAULT",
        &config
            .checkbox_default
            .map(|v| v.to_string())
            .unwrap_or_else(|| "false".to_string()),
    );

    idconfig = idconfig.replace(
        "HASDROPDOWN",
        if config.has_dropdown { "true" } else { "false" },
    );

    idconfig = idconfig.replace(
        "DROPDOWN_ID",
        &config.dropdown_id.clone().unwrap_or_else(|| "".to_string()),
    );

    idconfig = idconfig.replace(
        "DEFAULTSELECTED",
        &config
            .default_selected
            .clone()
            .unwrap_or_else(|| "".to_string()),
    );

    idconfig = idconfig.replace(
        "DEFAULTEMPTY",
        &config
            .checkbox_default
            .map(|v| v.to_string())
            .unwrap_or_else(|| "undefined".to_string()),
    );
    println!("{}", idconfig);
    return idconfig;
}
