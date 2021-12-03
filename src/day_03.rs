use crate::assets::ASSETS_FOLDER;

pub fn run() {
    // part_01();
    part_02();
}

fn part_02() {
    let bytes = ASSETS_FOLDER.get_file("day03.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes);
    let lines = string.lines();

    let linelen = string.lines().next().unwrap();

    // let mut nums = Vec::new();
}

fn part_01() {
    let bytes = ASSETS_FOLDER.get_file("day03.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes);

    let linelen = string.lines().next().unwrap().len();
    let mut lines = string.lines();

    let mut gamma_rate_bitvec = Vec::new();
    let mut epsilon_rate_bitvec = Vec::new();

    for i in 0..linelen {
        let mut ones = 0;
        let mut zeros = 0;

        for line in lines.clone() {
            match line.get(i..=i).unwrap() {
                "1" => {
                    ones += 1;
                },
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

//    let gamma_rate_bitstr = gamma_rate_bitvec.concat();
    let gamma_rate_num = isize::from_str_radix(&gamma_rate_bitvec.concat(), 2).unwrap();
    let epsilon_rate_num = isize::from_str_radix(&epsilon_rate_bitvec.concat(), 2).unwrap();



    dbg!(gamma_rate_num, epsilon_rate_num, gamma_rate_num * epsilon_rate_num);
}
