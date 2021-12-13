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
    energy: usize,
}

/// 10 x 10 Grid
///
/// 0, 0  is at Left Top
///
/// y is down
#[derive(Debug, Clone)]
struct OctoMap {
    inner: [[Octopus; 10]; 10],
    flashed_this_step: Vec<(usize, usize)>,
}

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day11.input").unwrap().contents();
    let string = String::from_utf8(bytes.to_vec()).unwrap();

    part_01(&string);
    part_02(&string);
}

fn part_01(input: &String) {
    let mut map = OctoMap::from_input(input);

    let mut total_flashes = 0;

    for _ in 0..100 {
        total_flashes += map.step();
    }
    println!("11/01: Total flashes after 100 steps: {}", total_flashes);
}

fn part_02(input: &String) {
    let mut map = OctoMap::from_input(input);

    let mut counter: u64 = 1;
    loop {
        if map.step() == 100 {
            println!("11/02: All Octopi flash at step {}", counter);
            break;
        }

        counter += 1;
    }
}

impl OctoMap {
    /// Returns map[y][x]
    pub fn from_input(input: &String) -> Self {
        let mut map = [[Octopus::new(0); 10]; 10];

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map[y][x] = Octopus::new(c.to_digit(10).unwrap() as usize);
            }
        }

        Self {
            inner: map,
            flashed_this_step: Vec::new(),
        }
    }

    pub fn step(&mut self) -> usize {
        self.flashed_this_step.clear();

        self.increase_levels();

        let mut cnt = 0;
        loop {
            let num_flashed = self.process_flashing();
            cnt += num_flashed;
            if num_flashed == 0 {
                break;
            }
        }

        self.cleanup_after_step();

        return cnt;
    }

    pub fn cleanup_after_step(&mut self) {
        for y in 0..10 {
            for x in 0..10 {
                if self.flashed_this_step.contains(&(x, y)) {
                    self.inner[y][x].energy = 0;
                }
            }
        }
    }

    /// Returns number of fishes that flashed
    pub fn process_flashing(&mut self) -> usize {
        let mut counter = 0;

        for y in 0..10 {
            for x in 0..10 {
                if self.inner[y][x].flashing() {
                    if self.flashed_this_step.contains(&(x, y)) {
                        continue;
                    }

                    self.flashed_this_step.push((x, y));
                    counter += 1;

                    let adjacents = self.get_adjacent_coords(x, y);
                    for (ad_x, ad_y) in adjacents {
                        self.inner[ad_y][ad_x].energy += 1;
                    }
                }
            }
        }

        return counter;
    }

    pub fn increase_levels(&mut self) {
        for y in 0..10 {
            for x in 0..10 {
                self.inner[y][x].energy += 1;
            }
        }
    }

    pub fn get_adjacent_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacents = Vec::new();

        for dir in Direction::all() {
            let dir = dir.value();
            let new_pos = ((x as isize + dir.0) as usize, (y as isize + dir.1) as usize);

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
        for (y, yline) in self.inner.iter().enumerate() {
            for (x, xval) in yline.iter().enumerate() {
                if self.flashed_this_step.contains(&(x, y)) {
                    print!("{}", format!("{}", xval.energy).red());
                } else {
                    print!("{}", xval.energy);
                }
            }
            println!();
        }
        println!();
    }
}

impl Octopus {
    pub fn new(energy: usize) -> Self {
        Self { energy }
    }

    fn flashing(&self) -> bool {
        self.energy > 9
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
    fn value(&self) -> (isize, isize) {
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
