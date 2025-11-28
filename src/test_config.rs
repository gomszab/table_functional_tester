use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestConfig {
    pub rowtablebody: String,
    pub coltablebody: String,
    pub colform: String,
    pub rowform: String,
    pub default_visible: Option<String>,
    pub has_checkbox: bool,
    pub checkbox_id: Option<String>,
    pub checkbox_default: Option<bool>,
    pub has_dropdown: bool,
    pub dropdown_id: Option<String>,
    pub default_selected: Option<String>,
    pub default_empty: Option<bool>,
}
