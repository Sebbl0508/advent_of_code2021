use std::str::Lines;
use crate::assets::ASSETS_FOLDER;

#[derive(Debug, Clone)]
struct Board {
    num_board: Vec<Vec<u32>>,
}

struct BoardMgr {
    boards: Vec<Board>,
    drawn_nums: Vec<u32>,
}

pub fn run() {
    // part_01();
    part_02();
}

fn part_01() {
    let bytes = ASSETS_FOLDER.get_file("day04.input").unwrap().contents();
    let string = String::from_utf8_lossy(bytes).to_string();

    let mut lines = string.lines();
    let numbers: Vec<u32> = lines.next().unwrap().split(",").map(|x| x.parse::<u32>().unwrap()).collect();

    let mut board_mgr = BoardMgr::new(&mut lines);

    'top_loop: for i in numbers {
        board_mgr.add_drawn_nums(i);

        for board in &board_mgr.boards {
            if board.has_bingo(&board_mgr.drawn_nums) {
                println!("04/01: Sum of non-drawn numbers * current drawn number: {}", board.sum_of_unmarked(&board_mgr.drawn_nums) * i);
                break 'top_loop;
            }
        }
    }


}

fn part_02() {
    todo!();
}



impl Board {
    pub fn new(lines: Vec<&str>) -> Self {
        let mut intern = Vec::new();

        for line in lines {
            let numbers: Vec<u32> = line.split(" ").filter(|&x| x != "").map(|x| x.parse::<u32>().unwrap()).collect();
            intern.push(numbers);
        }


        Self {
            num_board: intern,
        }
    }

    pub fn sum_of_unmarked(&self, nums_coollected: &Vec<u32>) -> u32 {
        let mut sum = 0;
        for row in &self.num_board {
            for i in row {
                if !nums_coollected.contains(i) {
                    sum += *i;
                }
            }
        }

        return sum;
    }

    pub fn has_bingo(&self, nums_collected: &Vec<u32>) -> bool {
        let mut col = Vec::new();

        for row in self.num_board.iter() {
            let mut cnt = 0;

            col.push(row[0]);

            for i in row {
                if nums_collected.contains(i) {
                    cnt += 1;
                }
            }

            if cnt == row.len() {
                return true;
            }
        }

        assert_eq!(col.len(), 5);

        return col.iter().all(|x| nums_collected.contains(x));
    }

    /*
    pub fn check_num(&mut self, num: u32) -> bool {
        for i in self.num_board.iter() {
            for j in i {
                if *j == num {
                    return true;
                }
            }
        }

        return false;
    }
    */
}

impl BoardMgr {
    pub fn new(lines: &mut Lines) -> Self {
        let mut boards = Vec::new();

        while let Some(line) = lines.next() {
            // Blank lines are skipped by vodoo magic ( ͡° ͜ʖ ͡°)
            let board_lines: Vec<&str> = lines.take(5).collect();
            boards.push(Board::new(board_lines));
        }

        Self {
            boards,
            drawn_nums: Vec::new(),
        }
    }

    pub fn add_drawn_nums(&mut self, num: u32) {
        self.drawn_nums.push(num);
    }
}