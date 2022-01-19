use std::fs;
use std::str;

pub struct Lines {
    data: String,
}

impl Lines {
    pub fn new(file: &str) -> Lines {
        let filename = format!("./inputs/{}", file);
        let contents = match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(err) => panic!("Could not read file {}: {:?}", file, err),
        };
        Lines { data: contents }
    }

    pub fn lines(&self) -> std::str::Lines {
        self.data.lines()
    }
}
