use crate::arguments::factories::{CommandFactory, GeneratorFactory, TestRunnerFactory};



pub enum Mode {
    /// Run tests with different params
    RunTest(RunMode),
    /// Generate config.js for test running
    Generate,
    /// Saved config.js template with path and name
    TemplateList,
    /// create new template: ask path of configjs and name for template
    CreateTemplateBasedOnConfigPath,
    /// update templatelist
    UpdateCoreTemplateList,
    /// run latest command
    RunLatestCommand,
    /// print out help
    Help,
    /// start server
    FrontendMode,
}

pub struct RunMode{
    pub file_path: String,
    pub tag: Option<String>,
    pub template_name: Option<String>
}

impl RunMode {
   pub fn new(file_path: String) -> Self {
    Self { file_path, tag: None, template_name: None }
    }

   pub fn withtemplate(&self, template_name: String) -> Self {
    Self { file_path: self.file_path.clone(), tag: self.tag.clone(), template_name: Some(template_name) }
    }

    pub fn withtag(&self, tag: String) -> Self {
        Self { file_path: self.file_path.clone(), tag: Some(tag), template_name: self.template_name.clone() }
    }

}

pub fn pars_args(args: Vec<String>) -> Result<Mode, String>{
    let command_factories:Vec<Box<dyn CommandFactory>> = vec![
        Box::new(GeneratorFactory),
        Box::new(TestRunnerFactory),
    ];
    if args.len() == 0 {
        return Ok(Mode::Help)
    } else{
        for factory in command_factories{
            if let Some(mode) = factory.try_parse(&args){
               return Ok(mode)
            }
        }
    };
    Err("Ismeretlen parancs".to_string())
}