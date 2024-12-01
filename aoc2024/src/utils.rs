use std::fs;

pub fn read_input(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}
