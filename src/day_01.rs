use crate::assets::ASSETS_FOLDER;

pub fn run() {
    // part_01();
    part_02();
}

fn part_01() {
    let bytes = ASSETS_FOLDER.get_file("day01part01.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes).to_string();
    let mut lines = string.lines();

    let mut num_positive_depth_change = 0;

    let mut last_value = lines.next().unwrap().parse::<i32>().unwrap();

    for (i, e) in lines.enumerate() {
        let cur_value: i32 = e.parse().unwrap();

        if cur_value > last_value {
            num_positive_depth_change += 1;
        }

        last_value = cur_value;
    }

    println!("01/01: Number of depth increases: {}", num_positive_depth_change);
}

fn part_02() {
    let bytes = ASSETS_FOLDER.get_file("day01part02.input").unwrap().contents();
}