mod generated_assets;
mod resultutil;
mod test_config;

use headless_chrome::{Browser, FetcherOptions, LaunchOptionsBuilder, protocol::cdp};
use serde_json::Value;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::{
    generated_assets::{ID_CONFIG, get_api_files, get_testcases_map},
    resultutil::ResultUtil,
    test_config::TestConfig,
};
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

        let content_of_id = get_content_from_bytes(ID_CONFIG);
        let content_of_id = parse_config(content_of_id, path);
        let onlytype = if let Some(onlytype) = args.next() {
            onlytype
        } else {
            String::new()
        };

        let test_cases_map = get_testcases_map();
        let mut result_util = ResultUtil::new(test_cases_map.len());

        for (key, value) in get_testcases_map() {
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

fn parse_config(mut idconfig: String, html_path: String) -> String {
    let html_path = Path::new(&html_path);
    let dir = html_path.parent().expect("path has no parent directory");
    let config_path: PathBuf = dir.join("config.json");
    let config_json = fs::read_to_string(config_path).expect("cannot read config.json");
    let config: TestConfig = serde_json::from_str(&config_json).expect("Invalid JSON");

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
        &config
            .checkbox_id
            .clone()
            .unwrap_or_else(|| "undefined".to_string()),
    );

    idconfig = idconfig.replace(
        "CHECKBOXDEFAULT",
        if config.checkbox_default { "true" } else { "false" },
    );

    idconfig = idconfig.replace(
        "HASDROPDOWN",
        if config.has_dropdown { "true" } else { "false" },
    );

    idconfig = idconfig.replace(
        "DEFAULTSELECTED",
        &config
            .default_selected
            .clone()
            .unwrap_or_else(|| "undefined".to_string()),
    );

    idconfig = idconfig.replace(
        "DEFAULTEMPTY",
        if config.default_empty {
            "true"
        } else {
            "false"
        },
    );
    println!("{}", idconfig);
    return idconfig;
}
