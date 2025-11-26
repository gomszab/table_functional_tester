use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize)]
pub struct TestConfig{
   pub rowtablebody: String,
   pub coltablebody: String,
   pub colform: String,
   pub rowform: String,
   pub default_visible: Option<String>,
   pub has_checkbox: bool,
   pub checkbox_id: Option<String>,
   pub checkbox_default: bool,
   pub has_dropdown: bool,
   pub default_selected: Option<String>,
   pub default_empty: bool
}