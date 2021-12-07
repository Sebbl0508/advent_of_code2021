use std::cmp::min;
use crate::assets::ASSETS_FOLDER;

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day07.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes).to_string();

    part_01(&string);
    part_02(&string);
}

fn part_01(input: &String) {
    let mut crab_positions: Vec<usize> = input.trim().split(",").map(|v| v.parse::<usize>().unwrap()).collect();
    crab_positions.sort();

    let mut lowest = crab_positions.get(0).unwrap();
    let mut lowest = get_fuel_consumption_p01(&crab_positions, *lowest);

    for i in crab_positions.iter().skip(1) {
        let f_consump = get_fuel_consumption_p01(&crab_positions, *i);

        if f_consump < lowest {
            lowest = f_consump;
        }
    }

    println!("07/01: The Lowest fuel consumption of all crabs: {}", lowest);
}

/// This function is not recommended to run in Debug mode :/
fn part_02(input: &String) {
    let mut crab_positions: Vec<usize> = input.trim().split(",").map(|v| v.parse::<usize>().unwrap()).collect();
    crab_positions.sort();

    let mut lowest = crab_positions.iter().fold(crab_positions[0], |acc, &v| {
        if v < acc {
            v
        } else {
            acc
        }
    });

    let mut highest = crab_positions.iter().fold(crab_positions[0], |acc, &v| {
        if v > acc {
            v
        } else {
            acc
        }
    });


    let mut lowest_fcons = get_fuel_consumption_p02(&crab_positions, lowest);


    for v in (lowest+1)..highest {
        let f_consump = get_fuel_consumption_p02(&crab_positions, v);

        if f_consump < lowest_fcons {
            lowest_fcons = f_consump;
        }
    }

    println!("07/02: The Lowest fuel consumption of all crabs: {}", lowest_fcons);
}

fn get_fuel_consumption_p01(positions: &Vec<usize>, target_pos: usize) -> usize {
    positions.iter().fold(0, |acc, &v| acc + (diff(v as isize, target_pos as isize)))
}

fn get_fuel_consumption_p02(positions: &Vec<usize>, target_pos: usize) -> usize {
    positions.iter().filter(|&&v| v != target_pos).fold(0, |acc, &v| acc + accumulate(diff(v as isize, target_pos as isize)))
}

fn accumulate(val: usize) -> usize {
    let mut a = 0;

    for i in 0..=val {
        a += i;
    }

    return a;
}

fn diff(val1: isize, val2: isize) -> usize {
    (val1 - val2).abs() as usize
}

fn get_nearest(values: &Vec<usize>, val: usize) -> usize {
    let mut min_diff = 0;
    let mut cur_idx = 0;

    let mut valiter = values.iter();

    min_diff = (*valiter.next().unwrap() as isize - val as isize).abs();

    for (i, v) in valiter.enumerate() {
        let diff = (*v as isize - val as isize).abs();
        if diff < min_diff {
            min_diff = diff;
            cur_idx = i;
        }
    }

    *values.get(cur_idx+1).unwrap()
}