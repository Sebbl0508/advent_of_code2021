use crate::assets::ASSETS_FOLDER;

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day09.example").unwrap().contents();
    let string = String::from_utf8(bytes).unwrap();

    part_01(&string);
    part_02(&string);
}

fn part_01(input: &String) {
    todo!()
}

fn part_02(input: &String) {
    todo!();
}