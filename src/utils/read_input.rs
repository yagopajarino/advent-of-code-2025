use std::fs;

pub fn read_input(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path);
    let lines = contents
        .expect("Error opening file")
        .split("\n")
        .map(|s| s.into())
        .filter(|line: &String| line.len() > 0)
        .collect::<Vec<String>>();

    lines
}
