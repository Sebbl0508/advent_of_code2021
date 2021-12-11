use crate::assets::ASSETS_FOLDER;
use std::collections::{HashMap, HashSet};

const UNIQ_DIGIT_ONE_LEN: usize = 2;
const UNIQ_DIGIT_FOUR_LEN: usize = 4;
const UNIQ_DIGIT_SEVEN_LEN: usize = 3;
const UNIQ_DIGIT_EIGHT_LEN: usize = 7;

const SEGMENTS: &'static str = "abcdefg";

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day08.example").unwrap().contents();
    let string = String::from_utf8(bytes.to_vec()).unwrap();

    // part_01(&string);
    part_02(&string);
}

fn part_01(input: &String) {
    let mut lines = input.lines();
    let mut counter = 0;

    for line in lines {
        let (_, outputs) = parse_input(line);
        for output in outputs {
            match output.len() {
                UNIQ_DIGIT_ONE_LEN | UNIQ_DIGIT_FOUR_LEN | UNIQ_DIGIT_SEVEN_LEN
                | UNIQ_DIGIT_EIGHT_LEN => {
                    counter += 1;
                }
                _ => {}
            }
        }
    }

    println!("08/01: Number of unique digits: {}", counter);
}

fn part_02(input: &String) {
    let mut lines = input.lines();

    for line in lines {
        let (inputs, outputs) = parse_input(line);
        build_connections(inputs, outputs);
    }
}

fn build_connections<'a>(inputs: Vec<&'a str>, outputs: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut digits_map = vec![vec![]; 10];

    // Build known digits: 1, 4, 7, 8
    for idk in outputs.iter().chain(inputs.iter()) {
        match idk.len() {
            UNIQ_DIGIT_ONE_LEN => {
                digits_map[1].push(*idk);
            }
            UNIQ_DIGIT_FOUR_LEN => {
                digits_map[4].push(*idk);
            }
            UNIQ_DIGIT_SEVEN_LEN => {
                digits_map[7].push(*idk);
            }
            UNIQ_DIGIT_EIGHT_LEN => {
                digits_map[8].push(*idk);
            }
            _ => {}
        }
    }
    println!(
        "inputs: {:?}\noutputs {:?}\nwire_map: {:?}\n",
        inputs, outputs, digits_map
    );

    digits_map
}

fn parse_input(line: &str) -> (Vec<&str>, Vec<&str>) {
    let parsed_line: Vec<&str> = line.split("|").collect();
    let inputs: Vec<&str> = parsed_line[0].trim().split(" ").collect();
    let outputs: Vec<&str> = parsed_line[1].trim().split(" ").collect();

    (inputs, outputs)
}

/*
fn segments_idx() -> [&'static [usize]; 10] {
    [
        &[0, 1, 2, 4, 5, 6],    // 0
        &[2, 5],                // 1
        &[0, 2, 3, 4, 6],       // 2
        &[0, 2, 3, 5, 6],       // 3
        &[1, 2, 3, 5],          // 4
        &[0, 1, 3, 5, 6],       // 5
        &[0, 1, 3, 4, 5, 6],    // 6
        &[0, 2, 5],             // 7
        &[0, 1, 2, 3, 4, 5, 6], // 8
        &[0, 1, 2, 3, 5, 6],    // 9
    ]
}
*/