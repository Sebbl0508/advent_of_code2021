use crate::assets::ASSETS_FOLDER;
use std::collections::VecDeque;

pub fn run() {
    part_01();
    part_02();
}

fn part_01() {
    let bytes = ASSETS_FOLDER
        .get_file("day01.input")
        .unwrap()
        .contents();
    let string = String::from_utf8_lossy(bytes).to_string();
    let mut lines = string.lines();

    let mut num_positive_depth_change = 0;

    let mut last_value = lines.next().unwrap().parse::<u32>().unwrap();

    for line in lines {
        let cur_value: u32 = line.parse().unwrap();

        if cur_value > last_value {
            num_positive_depth_change += 1;
        }

        last_value = cur_value;
    }

    println!(
        "01/01: Number of depth increases: {}",
        num_positive_depth_change
    );
}

fn part_02() {
    let bytes = ASSETS_FOLDER
        .get_file("day01.input")
        .unwrap()
        .contents();
    let string = String::from_utf8_lossy(bytes);
    let mut lines = string.lines();

    let mut num_positive_depth_change = 0;
    let mut last_sum = lines
        .clone()
        .take(3)
        .map(|i| i.parse::<u32>().unwrap())
        .sum();

    let mut values = VecDeque::new();

    for line in lines {
        values.push_back(line.parse::<u32>().unwrap());

        // If the Queu doesn't have 3 values yet, continue to next loop iteration
        if values.len() < 3 {
            continue;
        }

        let sum: u32 = values.iter().sum();

        // Discard popped value
        let _ = values.pop_front();

        if sum > last_sum {
            num_positive_depth_change += 1;
        }

        last_sum = sum;
    }

    println!(
        "01/02: Number of depth increases: {}",
        num_positive_depth_change
    );
}
