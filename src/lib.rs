use std::fs;

mod y2020;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("failed to read file")
}