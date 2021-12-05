use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::assets::ASSETS_FOLDER;

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day05.example").unwrap().contents();
    let string = String::from_utf8_lossy(bytes).to_string();

    // part_01(&string);
    part_02(&string);
}


fn part_01(input: &String) {
    let lines = input.lines();

    let mut vent_lines = Vec::new();

    for line in lines {
        vent_lines.push(VentLine::from_line(line));
    }

    println!("05/01: Number of vent lines overlapping: {}", find_overlapping_p01(&vent_lines));
}

fn part_02(input: &String) {

}


fn find_overlapping_p01(vent_lines: &Vec<VentLine<u32>>) -> u32 {
    let mut map = HashMap::new();

    for vent_line in vent_lines.iter() {
        if vent_line.0.x != vent_line.1.x && vent_line.0.y != vent_line.1.y {
            continue;
        }

        if vent_line.0.y == vent_line.1.y {

            let (start, end) = if vent_line.0.x > vent_line.1.x {
                (vent_line.1.x, vent_line.0.x)
            } else {
                (vent_line.0.x, vent_line.1.x)
            };

            for x in start..=end {
                let pos = Vec2::new(x, vent_line.0.y);
                if map.contains_key(&pos) {
                    map.get_mut(&pos).map(|v| *v += 1);
                } else {
                    map.insert(pos, 1);
                }
            }
        } else {
            let (start, end) = if vent_line.0.y > vent_line.1.y {
                (vent_line.1.y, vent_line.0.y)
            } else {
                (vent_line.0.y, vent_line.1.y)
            };


            for y in start..=end {
                let pos = Vec2::new(vent_line.0.x, y);
                if map.contains_key(&pos) {
                    map.get_mut(&pos).map(|v| *v += 1);
                } else {
                    map.insert(pos, 1);
                }
            }
        }
    }

    map.iter().filter(|x| *x.1 > 1).count() as u32
}


#[derive(Clone)]
struct VentLine<T>(Vec2<T>, Vec2<T>);

#[derive(Debug, Clone, Hash, Eq)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> PartialEq<Self> for Vec2<T>
where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


impl VentLine<u32> {
    pub fn from_line(line: &str) -> VentLine<u32> {
        let txt_splits: Vec<&str> = line.split(" -> ").collect();
        let begin_point: Vec<&str> = txt_splits.get(0).unwrap().split(",").collect();
        let begin_point: Vec2<u32> = Vec2::new(begin_point[0].parse().unwrap(), begin_point[1].parse().unwrap());

        let end_point: Vec<&str> = txt_splits.get(1).unwrap().split(",").collect();
        let end_point: Vec2<u32> = Vec2::new(end_point[0].parse().unwrap(), end_point[1].parse().unwrap());

        return VentLine(begin_point, end_point);
    }
}

impl Debug for VentLine<u32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VentLine([{}, {}] -> [{}, {}])", self.0.x, self.0.y, self.1.x, self.1.y)
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x, y
        }
    }
}
