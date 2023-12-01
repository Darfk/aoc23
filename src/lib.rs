use std::{path::PathBuf, fs};

pub fn read_input_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path.parse::<PathBuf>().unwrap())
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>()
}