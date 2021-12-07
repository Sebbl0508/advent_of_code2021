use std::cmp::min;
use crate::assets::ASSETS_FOLDER;

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day07.example").unwrap().contents();
    let string = String::from_utf8_lossy(bytes).to_string();

    part_01(&string);
    // part_02(&string);
}

// Binary Search?
// Begin at average of all values (sum / len_of_vec)
// Get the fuel consumption
// Get the average of both halfes again now (1/4 & 3/4)
// Get fuel consumption of both halfes
// Continue wherever the fuel consumption is lower than the first
fn part_01(input: &String) {
    let mut starting_positions: Vec<usize> = input.trim().split(",").map(|v| v.parse::<usize>().unwrap()).collect();
    starting_positions.sort();

    let avg = starting_positions.iter().sum::<usize>() / starting_positions.len();
    let avg = get_nearest(&starting_positions, avg);

    let mut smaller: Vec<usize> = starting_positions.clone().into_iter().filter(|&v| v <= avg).collect();
    let mut bigger: Vec<usize> = starting_positions.clone().into_iter().filter(|&v| v > avg).collect();

    println!("{:?}\navg: {}\nfuel consumption: {}", &starting_positions, avg, get_fuel_consumption(&starting_positions, avg));
    println!("\n\n{:?}\n{:?}\n{:?}", starting_positions, smaller, bigger);
}


fn part_02(input: &String) {
    todo!();
}

fn get_fuel_consumption(positions: &Vec<usize>, target_pos: usize) -> usize {
    positions.iter().fold(0, |acc, &v| acc + (diff(v as isize, target_pos as isize)))
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