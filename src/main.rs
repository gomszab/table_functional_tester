use headless_chrome::{Browser, FetcherOptions, LaunchOptionsBuilder};
// use std::env
use std::{fs, path::Path};
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

    // let mut args = env::args();

    // if let Some(path) = args.next(){

    let browser = Browser::new(options).unwrap();

    let tab = browser.new_tab().unwrap();

    tab.navigate_to(&format!(
        "file:///{}",
        "dummy"
    ))
    .unwrap();

    // tab.navigate_to(&format!("file:///{}", path)).unwrap();

    tab.wait_until_navigated().unwrap();

    let testcases_dir = Path::new("./testcases/html");
    let api_dir = Path::new("./testapi");   

    for entry in std::fs::read_dir(testcases_dir)? {
        for jsapi in std::fs::read_dir(api_dir)? {
            let path = jsapi?.path();

            let script = fs::read_to_string(&path)?;

            tab.evaluate(&script, false).unwrap();
        }

        let path = entry?.path();

        if path.extension().map(|ext| ext == "js").unwrap_or(false) {
            println!("Running test from {:?}", path);
            let script = fs::read_to_string(&path)?;
            let result = tab.evaluate(&script, true).unwrap();
            // println!("{:?}", result);
            let test_passed = result.value.and_then(|v| v.as_bool()).unwrap_or(false);
            println!("Test passed? {:?}", test_passed);
            tab.reload(true, None).unwrap();
            tab.wait_until_navigated().unwrap();
        }
    }
    // }

    Ok(())
}
