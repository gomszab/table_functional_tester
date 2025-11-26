use std::fmt::Display;

pub struct ResultUtil{
    sumtestcount: usize,
    success_test_count: i32,
    ignored_test_count: i32,
    errors: Vec<String>
}

impl ResultUtil {
    pub fn new(sumtestcount: usize) -> Self{
        Self { sumtestcount, success_test_count: 0, ignored_test_count: 0, errors: Vec::new() }
    }
}


impl ResultUtil {
    pub fn increment_success(&mut self){
        self.success_test_count += 1;
    }

    pub fn increment_ignored(&mut self){
        self.ignored_test_count += 1;
    }

    pub fn push_error(&mut self, value: String){
        self.errors.push(value);
    }
}

impl Display for ResultUtil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!( "\n*********************************************************\n{} tesztből {} sikeres, {} sikertelen, {} kihagyott\n*********************************************************\n",
            self.sumtestcount,
            self.success_test_count,
            self.errors.len(),
            self.ignored_test_count);
        if self.errors.len() > 1 {
            result.push_str("A hibák a következők:");
            for error in &self.errors {
                result.push_str(&format!("\n{}", error));
            }
        }
        f.write_fmt(format_args!("{}", result))
        
    }
}