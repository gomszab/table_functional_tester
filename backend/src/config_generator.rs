use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use crate::test_config::TestConfig;

pub fn run_generator() {
    let folder_path_str =
        prompt_string("Add meg a könyvtárat ahol az index.html van (id fogom menteni a configot):");
    let folder_path = Path::new(&folder_path_str);

    // collect config fields
    let config = TestConfig {
        rowtablebody: prompt_string("A rowspanos tbody id-ja?"),
        coltablebody: prompt_string("A colspanos tbody id-ja?"),
        colform: prompt_string("A colspanos táblázathoz tartozó form id-ja?"),
        rowform: prompt_string("A rowspanos táblázathoz tartozó form id-ja?"),
        default_visible: prompt_optional_string("Az alapértelmezetten megjelenő div id-ja?"),
        has_checkbox: prompt_bool("Van-e checkbox az oldalon?"),
        checkbox_id: prompt_optional_string("Mi a checkbox id-ja?"),
        checkbox_default: prompt_optional_bool("Mi a checkbox alapértelmezett értéke?"),
        has_dropdown: prompt_bool("Van-e select az oldalon?"),
        dropdown_id: prompt_optional_string("Mi a select id-ja?"),
        default_selected: prompt_optional_string("Mi az alapértelmezetten kiválasztott?"),
        default_empty: prompt_optional_bool("Alapértelmezetten üres-e a dropdownlist értéke?"),
    };

    if config.has_checkbox || config.has_dropdown {
        let json_str = serde_json::to_string_pretty(&config).unwrap();
        let config_path = folder_path.join("config.json");
        fs::write(config_path, json_str).unwrap();
        println!("config.json elmentve.");
    } else {
        println!(
            "Nem adtál meg sem checkboxot, sem pedig select-t, az egyiknek legalább kell lennie"
        );
    }
}

fn prompt_string(prompt: &str) -> String {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_bool(prompt: &str) -> bool {
    loop {
        let input = prompt_string(&format!("{prompt} (1 = true/ 0 = false)"));
        match input.to_lowercase().as_str() {
            "1" => return true,
            "0" => return false,
            _ => println!("0-t vagy 1-t lehet megadni."),
        }
    }
}

fn prompt_optional_string(prompt: &str) -> Option<String> {
    let input = prompt_string(&format!(
        "{prompt} (opcionális, nyomj entert, ha nincs rá szükséged)"
    ));
    if input.is_empty() { None } else { Some(input) }
}

fn prompt_optional_bool(prompt: &str) -> Option<bool> {
    loop {
        let input = prompt_string(&format!(
            "{prompt} (opcionális true = 1 /false = 0 vagy nyomj entert, ha nincs rá szükséged)"
        ));
        if input.is_empty() {
            return None;
        }
        match input.to_lowercase().as_str() {
            "1" => return Some(true),
            "0" => return Some(false),
            _ => println!("0-t vagy 1-t lehet megadni, de üresen is hagyható"),
        }
    }
}
