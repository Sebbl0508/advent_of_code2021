use std::cmp::{max, min};
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
    let lines = input.lines();

    let mut vent_lines = Vec::new();

    for line in lines {
        vent_lines.push(VentLine::from_line(line));
    }

    println!("05/02: Number of vent lines overlapping: {}", find_overlapping_p02(&vent_lines));
}


fn find_overlapping_p01(vent_lines: &Vec<VentLine<i32>>) -> i32 {
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

    map.iter().filter(|x| *x.1 > 1).count() as i32
}

fn find_overlapping_p02(vent_lines: &Vec<VentLine<i32>>) -> u32 {
    let mut map = HashMap::new();

    for vent_line in vent_lines {
        let from = vent_line.0.clone();
        let to = vent_line.1.clone();

        if from.x == to.x {
            let start = min(from.y, to.y);
            let end = max(from.y, to.y);

            for x in start..=end {
                let pos = Vec2::new(x, from.y);
                if map.contains_key(&pos) {
                    map.get_mut(&pos).map(|v| *v += 1);
                } else {
                    map.insert(pos, 1);
                }
            }
        } else if from.y == to.y {
            let start = min(from.x, to.x);
            let end = max(from.x, to.x);

            for y in start..=end {
                let pos = Vec2::new(from.x, y);
                if map.contains_key(&pos) {
                    map.get_mut(&pos).map(|v| *v += 1);
                } else {
                    map.insert(pos, 1);
                }
            }
        } else {
            let x_delta = match from.x {
                x if x > to.x => -1,
                x if x < to.x => 1,
                _ => 0
            };
            let y_delta = match from.y {
                y if y > to.y => -1,
                y if y < to.y => 1,
                _ => 0
            };

            let mut pos = from.clone();

            while pos.x != to.x && pos.y != to.y {
                pos.x += x_delta;
                pos.y += y_delta;

                if map.contains_key(&pos) {
                    map.get_mut(&pos).map(|v| *v += 1);
                } else {
                    map.insert(pos.clone(), 1);
                }
            }
            println!("{:?}\n", &vent_line);
        }
    }
    let a = map.iter().filter(|v| *v.1 > 1);
    a.clone().for_each(|v| println!("{:?}: {}", v.0, v.1));

    a.count() as u32
    //map.iter().filter(|v| *v.1 > 1).for_each(|v| println!("{:?}: {}", v.0, v.1)).count() as u32
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
    pub fn from_line(line: &str) -> VentLine<i32> {
        let txt_splits: Vec<&str> = line.split(" -> ").collect();
        let begin_point: Vec<&str> = txt_splits.get(0).unwrap().split(",").collect();
        let begin_point: Vec2<i32> = Vec2::new(begin_point[0].parse().unwrap(), begin_point[1].parse().unwrap());

        let end_point: Vec<&str> = txt_splits.get(1).unwrap().split(",").collect();
        let end_point: Vec2<i32> = Vec2::new(end_point[0].parse().unwrap(), end_point[1].parse().unwrap());

        return VentLine(begin_point, end_point);
    }
}

impl Debug for VentLine<i32> {
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
