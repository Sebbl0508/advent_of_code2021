use crate::assets::ASSETS_FOLDER;

struct Submarine {
    depth: i32,
    horizontal_position: i32,
}


pub fn run() {
    part_01();
    // part_02();
}


fn part_01() {
    let bytes = ASSETS_FOLDER.get_file("day02part01.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes);
    let mut lines = string.lines();

    let mut my_sub = Submarine::new();

    for line in lines {
        my_sub.handle_command(line);
    }

    println!("02/01: Horizontal speed and depth multiplied: {}", my_sub.horizontal_position * my_sub.depth);
}

fn part_02() {

}


impl Submarine {
    pub fn new() -> Self {
        Submarine {
            depth: 0,
            horizontal_position: 0,
        }
    }

    pub fn handle_command(&mut self, cmd: &str) {
        let split_cmd: Vec<&str> = cmd.trim().split(" ").collect();
        let command = *split_cmd.get(0).unwrap();
        let num: i32 = split_cmd.get(1).unwrap().parse().unwrap();

        match command {
            "forward" => {
                self.horizontal_position += num;
            },
            "down" => {
                self.depth += num;
            },
            "up" => {
                self.depth -= num;
            },
            _ => panic!("IDK: {:?}", cmd),
        }
    }
}