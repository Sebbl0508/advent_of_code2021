use crate::assets::ASSETS_FOLDER;
use colored::Colorize;

enum Direction {
    N,
    NO,
    O,
    SO,
    S,
    SW,
    W,
    NW,
}

#[derive(Debug, Copy, Clone)]
struct Octopus {
    energy: u8,
    flashing: bool,
}

/// 10 x 10 Grid
///
/// 0, 0  is at Left Top
///
/// y is down
#[derive(Debug, Copy, Clone)]
struct OctoMap {
    inner: [[Octopus; 10]; 10],
}

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day11.example").unwrap().contents();
    let string = String::from_utf8(bytes.to_vec()).unwrap();

    part_01(&string);
    // part_02(&string);
}

fn part_01(input: &String) {
    let mut map = OctoMap::from_input(input);
}

fn part_02(input: &String) {
    todo!()
}


impl OctoMap {
    /// Returns map[y][x]
    pub fn from_input(input: &String) -> Self {
        let mut map = [[Octopus::new(0); 10]; 10];

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map[y][x] = Octopus::new(c.to_digit(10).unwrap() as u8);
            }
        }

        Self {
            inner: map
        }
    }

    pub fn step(&mut self) {
        for y in 0..10 {
            for x in 0..10 {
                let cur_oct = &mut self.inner[y][x];

                if cur_oct.energy == 9 {
                    let adjacents = self.get_adjacent_coords(x as u8, y as u8);
                    continue;
                }

                cur_oct.energy += 1;
            }
        }
    }

    pub fn get_adjacent_coords(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
        let mut adjacents = Vec::new();

        for dir in Direction::all() {
            let dir = dir.value();
            let new_pos = ((x as i8 + dir.0) as u8, (y as i8 + dir.1) as u8);

            // Invalid
            if new_pos.0 > 9 || new_pos.1 > 9 {
                continue;
            } else {
                adjacents.push(new_pos);
            }
        }
        return adjacents;
    }


    pub fn print(&self) {
        for y in self.inner.iter() {
            for x in y {
                if x.flashing {
                    print!("{}", format!("{}", x.energy).red());
                } else {
                    print!("{}", x.energy);
                }
            }
            println!();
        }
    }
}

impl Octopus {
    pub fn new(energy: u8) -> Self {
        Self {
            energy,
            flashing: false,
        }
    }
}

impl Direction {
    const fn all() -> [Direction; 8] {
        [
            Direction::N,
            Direction::NO,
            Direction::O,
            Direction::SO,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
    }

    /// Returns (x, y)
    #[rustfmt::skip]
    fn value(&self) -> (i8, i8) {
        match self {
            Direction::N  => ( 0, -1),
            Direction::NO => ( 1, -1),
            Direction::O  => ( 1,  0),
            Direction::SO => ( 1,  1),
            Direction::S  => ( 0,  1),
            Direction::SW => (-1,  1),
            Direction::W  => (-1,  0),
            Direction::NW => (-1, -1),
        }
    }
}