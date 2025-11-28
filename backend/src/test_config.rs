use macros::TemplateConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, TemplateConfig)]
pub struct TestConfig {
    #[templateid(value = "ROWTABLEBODY")]
    pub rowtablebody: String,
    #[templateid(value = "COLTABLEBODY")]
    pub coltablebody: String,
    #[templateid(value = "COLFORM")]
    pub colform: String,
    #[templateid(value = "ROWFORM")]
    pub rowform: String,
    #[templateid(value = "DEFAULTVISIBLE")]
    pub default_visible: Option<String>,
    #[templateid(value = "HASCHECKBOX")]
    pub has_checkbox: bool,
    #[templateid(value = "CHECKBOXID")]
    pub checkbox_id: Option<String>,
    #[templateid(value = "CHECKBOXDEFAULT")]
    pub checkbox_default: Option<bool>,
    #[templateid(value = "HASDROPDOWN")]
    pub has_dropdown: bool,
    #[templateid(value = "DROPDOWN_ID")]
    pub dropdown_id: Option<String>,
    #[templateid(value = "DEFAULTSELECTED")]
    pub default_selected: Option<String>,
    #[templateid(value = "DEFAULTEMPTY")]
    pub default_empty: Option<bool>,
}
