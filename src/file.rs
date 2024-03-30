use std::fs;
use std::fs::read_to_string;

pub trait File {
    fn isfile(&self) -> bool;
}

impl File for String {
    fn isfile(&self) -> bool {
        if fs::metadata(self).unwrap().is_file() {
            true
        } else {
            false
        }
    }    
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}