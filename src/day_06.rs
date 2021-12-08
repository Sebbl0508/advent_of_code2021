use crate::assets::ASSETS_FOLDER;

// New lanternfish every 7 days
// Fish is a number of days until it creates new fish
// First cycle is 2 more days

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day06.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes).to_string();

    part_01(&string);
    part_02(&string);
}

fn part_01(input: &String) {
    println!(
        "06/01: Number of Anglerfish after {} days: {}",
        80,
        run_sim(input, 80)
    );
}

// WARNING!: Very resource hungry
fn part_02(input: &String) {
    println!(
        "06/02: Number of Anglerfish after {} days: {}",
        256,
        run_sim(input, 256)
    );
}

// Stolen from github for performance reasons :(
fn run_sim(input: &String, num_days: usize) -> usize {
    let mut nums = input.split(",").fold([0; 9], |mut nums, n| {
        nums[n.parse::<usize>().unwrap()] += 1;
        nums
    });

    (1..num_days).for_each(|day| {
        nums[(day + 7) % 9] += nums[day % 9];
    });

    return nums.iter().sum();
}
