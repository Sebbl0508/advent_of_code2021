use crate::assets::ASSETS_FOLDER;

/*
     01
     ||
0 -- 2199943210
1 -- 3987894921
     9856789892
     8767896789
   y 9899965678
     x

     heightmap[y][x]
*/

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct HeightMap {
    internal: Vec<Vec<usize>>,
}


pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day09.example").unwrap().contents();
    let string = String::from_utf8(bytes.to_vec()).unwrap();

    // part_01(&string);
    part_02(&string);
}

fn part_01(input: &String) {
    let heightmap = HeightMap::new(input);

    let (width, height) = heightmap.get_size();

    let mut risk_lvl_sum = 0;
    for y in 0..height {
        for x in 0..width {
            let current_height = heightmap.try_get_height_val(x, y).unwrap();
            if heightmap.get_all_adjecant_vals(x, y).iter().all(|&v| v > current_height) {
                risk_lvl_sum += current_height + 1;
            }
        }
    }

    println!("09/01 Sum of risk levels of all low points: {}", risk_lvl_sum);
}

fn part_02(input: &String) {
    todo!();
}


impl HeightMap {
    fn new(input: &String) -> Self {
        let (width, height) = (input.lines().next().unwrap().chars().count(), input.lines().count());
        let mut heightmap = vec![vec![0; width]; height];

        for (y, line) in input.lines().enumerate() {
            for (x, height_val) in line.chars().enumerate() {
                heightmap[y][x] = height_val.to_digit(10).unwrap() as usize;
            }
        }

        Self {
            internal: heightmap
        }
    }

    fn try_get_height_val(&self, x: usize, y: usize) -> Option<usize> {
        self.internal.get(y).map(|v| v.get(x)).flatten().map(|v| *v)
    }

    fn get_size(&self) -> (usize, usize) {
        (self.internal[0].len(), self.internal.len())
    }

    fn get_all_adjecant_vals(&self, x: usize, y: usize) -> Vec<usize> {
        let dirs = Direction::all();
        let mut adjecants = Vec::new();

        for dir in dirs {
            let dir_num = dir.get_num();

            match self.try_get_height_val((x as isize + dir_num.0) as usize, (y as isize + dir_num.1) as usize) {
                Some(v) => {
                    adjecants.push(v);
                }
                None => {}
            }
        }

        adjecants
    }

    fn debug_print(&self) {
        for y in &self.internal {
            for x in y {
                print!("{}", x);
            }
            println!();
        }
    }
}

impl Direction {
    /// Returns (x, y)
    fn get_num(&self) -> (isize, isize) {
        match self {
            Direction::Up => {
                (0, -1)
            }
            Direction::Down => {
                (0, 1)
            }
            Direction::Left => {
                (-1, 0)
            }
            Direction::Right => {
                (1, 0)
            }
        }
    }

    const fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}