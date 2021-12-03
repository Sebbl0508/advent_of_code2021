use crate::assets::ASSETS_FOLDER;

pub fn run() {
    part_01();
    part_02();
}

fn part_01() {
    let bytes = ASSETS_FOLDER.get_file("day03.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes).to_string();

    let linelen = string.lines().next().unwrap().len();
    let lines = string.lines();

    let mut gamma_rate_bitvec = Vec::new();
    let mut epsilon_rate_bitvec = Vec::new();

    for i in 0..linelen {
        let mut ones = 0;
        let mut zeros = 0;

        for line in lines.clone() {
            match line.get(i..=i).unwrap() {
                "1" => {
                    ones += 1;
                }
                "0" => {
                    zeros += 1;
                }
                _ => {
                    panic!("Invalid input: {:?}", line);
                }
            }
        }

        if ones > zeros {
            gamma_rate_bitvec.push("1");
            epsilon_rate_bitvec.push("0");
        } else {
            gamma_rate_bitvec.push("0");
            epsilon_rate_bitvec.push("1");
        }
    }

    let gamma_rate_num = isize::from_str_radix(&gamma_rate_bitvec.concat(), 2).unwrap();
    let epsilon_rate_num = isize::from_str_radix(&epsilon_rate_bitvec.concat(), 2).unwrap();

    println!(
        "03/01: Power consumption of the Submarine: {}",
        gamma_rate_num * epsilon_rate_num
    );
}

fn part_02() {
    let bytes = ASSETS_FOLDER.get_file("day03.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes).to_string();
    let lines = string.lines();

    let linelen = string.lines().next().unwrap().len();

    let oxygen_generator_rating = determine_rating(lines.clone().collect(), linelen, '1');
    let co2_scrubber_rating = determine_rating(lines.clone().collect(), linelen, '0');

    let final_num = isize::from_str_radix(oxygen_generator_rating, 2).unwrap()
        * isize::from_str_radix(co2_scrubber_rating, 2).unwrap();

    println!(
        "03/02: Oxygen Generator Rating * CO2 Scrubber Rating: {:?}",
        final_num
    );
}

fn determine_rating(lines: Vec<&str>, linelen: usize, preference: char) -> &str {
    let mut loc_lines = lines.clone();

    for i in 0..linelen {
        let mut ones = Vec::new();
        let mut zeros = Vec::new();

        for line in loc_lines.clone() {
            match line.get(i..=i).unwrap() {
                "1" => {
                    ones.push(line);
                }
                "0" => {
                    zeros.push(line);
                }
                _ => panic!("Invalid input: {:?}: {:?}", line.get(i..=i), line),
            }
        }

        loc_lines.clear();

        match preference {
            '1' => {
                if ones.len() > zeros.len() {
                    loc_lines.extend(ones);
                } else if ones.len() == zeros.len() {
                    loc_lines.extend(ones);
                } else {
                    loc_lines.extend(zeros);
                }
            }
            '0' => {
                if ones.len() > zeros.len() {
                    loc_lines.extend(zeros);
                } else if ones.len() == zeros.len() {
                    loc_lines.extend(zeros);
                } else {
                    loc_lines.extend(ones);
                }
            }
            _ => panic!("Invalid input preference: {:?}", preference),
        }

        if loc_lines.len() <= 1 {
            break;
        }
    }

    loc_lines[0]
}
