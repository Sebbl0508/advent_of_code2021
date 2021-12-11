use crate::assets::ASSETS_FOLDER;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const UNIQ_DIGIT_ONE_LEN: usize = 2;
const UNIQ_DIGIT_FOUR_LEN: usize = 4;
const UNIQ_DIGIT_SEVEN_LEN: usize = 3;
const UNIQ_DIGIT_EIGHT_LEN: usize = 7;

const SEGMENTS: &'static str = "abcdefg";

pub type Connection = (usize, char);

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day08.input").unwrap().contents();
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

    let mut total = 0;
    for line in lines {
        let (inputs, outputs) = parse_input(line);
        let input_map = build_digit_map(inputs);

        let mut counter = 0;
        for to_find in outputs {
            let sorted_output_string = to_find.chars().sorted().collect::<String>();

            for i in 0..10 {
                if input_map[i] == sorted_output_string {
                    counter = (counter * 10) + i;
                }
            }
        }
        total += counter;
    }

    println!("08/02 Some sum of somthing: {:?}", total);
}

fn build_digit_map(inputs: Vec<&str>) -> [String; 10] {
    let mut digits_map = [""; 10];

    // Build known digits: 1, 4, 7, 8
    for chars_str in inputs.iter() {
        match chars_str.len() {
            UNIQ_DIGIT_ONE_LEN => {
                digits_map[1] = *chars_str;
            }
            UNIQ_DIGIT_FOUR_LEN => {
                digits_map[4] = *chars_str;
            }
            UNIQ_DIGIT_SEVEN_LEN => {
                digits_map[7] = *chars_str;
            }
            UNIQ_DIGIT_EIGHT_LEN => {
                digits_map[8] = *chars_str;
            }
            _ => {}
        }
    }

    for chars_str in inputs.iter() {
        if chars_str.len() == 6 {
            if dumb_fns::is_nine(&digits_map, chars_str) {
                digits_map[9] = *chars_str;
            } else if dumb_fns::is_three_or_zero(&digits_map, chars_str) {
                digits_map[0] = *chars_str;
            } else {
                digits_map[6] = *chars_str;
            }
        }
    }

    for chars_str in inputs.iter() {
        if chars_str.len() == 5 {
            if dumb_fns::is_three_or_zero(&digits_map, chars_str) {
                digits_map[3] = *chars_str;
            } else if dumb_fns::is_five(&digits_map, chars_str) {
                digits_map[5] = *chars_str;
            } else {
                digits_map[2] = *chars_str;
            }
        }
    }

    let sorted_digit_map = digits_map.map(|v| v.chars().sorted().collect::<String>());

    sorted_digit_map
}

fn parse_input(line: &str) -> (Vec<&str>, Vec<&str>) {
    let parsed_line: Vec<&str> = line.split("|").collect();
    let inputs: Vec<&str> = parsed_line[0].trim().split(" ").collect();
    let outputs: Vec<&str> = parsed_line[1].trim().split(" ").collect();

    (inputs, outputs)
}

fn build_possible_connections() -> Vec<Connection> {
    let mut possible_combinations = Vec::new();

    for segment in [0, 1, 2, 3, 4, 5, 6] {
        for character in "abcdefg".chars() {
            possible_combinations.push((segment, character));
        }
    }

    possible_combinations
}

mod dumb_fns {
    type DigitMap<'a, 'b> = &'a [&'b str];

    pub fn is_nine(digits_map: DigitMap, inp_chars: &str) -> bool {
        for character in digits_map[4].chars() {
            if !inp_chars.contains(character) {
                return false;
            }
        }

        return true;
    }

    pub fn is_three_or_zero(digits_map: DigitMap, inp_chars: &str) -> bool {
        for character in digits_map[1].chars() {
            if !inp_chars.contains(character) {
                return false;
            }
        }

        return true;
    }

    pub fn is_five(digits_map: DigitMap, inp_chars: &str) -> bool {
        let mut missing = 0;

        for character in digits_map[6].chars() {
            if !inp_chars.contains(character) {
                missing += 1;
            }
        }

        return missing == 1;
    }
}
