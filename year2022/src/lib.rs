use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn text_file_to_vec(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Could not read file.");

    let lines = contents.split("\n");

    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        vec.push(line.to_string());
    }

    // Remove final, empty line
    vec.remove(vec.len() - 1);

    vec
}
