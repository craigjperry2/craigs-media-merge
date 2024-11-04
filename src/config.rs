pub struct Config {
    file_exclude_patterns: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            file_exclude_patterns: vec![
                String::from("*.tmp"),
                String::from("*.bak"),
                String::from("*.swp"),
            ],
        }
    }

    pub fn file_exclude_patterns(&self) -> &Vec<String> {
        &self.file_exclude_patterns
    }
}
