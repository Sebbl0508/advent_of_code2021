use crate::assets::ASSETS_FOLDER;
use itertools::Itertools;

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day10.input").unwrap().contents();
    let string = String::from_utf8(bytes.to_vec()).unwrap();

    part_01(&string);
    part_02(&string);
}

fn part_01(input: &String) {
    let mut total_syntax_error_score = 0;

    for line in input.lines() {
        let score = syntax_error_score_of_line(line);
        total_syntax_error_score += score;
    }

    println!(
        "10/01: Total syntax error score: {}",
        total_syntax_error_score
    );
}

fn part_02(input: &String) {
    let mut scores = Vec::new();

    for line in input.lines() {
        scores.push(complete_line_score(line));
    }

    let scores = scores
        .iter()
        .filter(|&&v| v > 0)
        .sorted()
        .collect::<Vec<&u64>>();
    println!(
        "10/02: Total score: {}",
        scores.get(scores.len() / 2).unwrap()
    );
}

fn complete_line_score(line: &str) -> u64 {
    let mut stack = Vec::new();
    for c in line.chars() {
        let stackfirst = *stack.last().unwrap_or(&'_');

        match (stackfirst, c) {
            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                let _ = stack.pop().unwrap();
            }
            (_, ')') | (_, ']') | (_, '}') | (_, '>') => {
                return 0;
            }
            (_, _) => {
                stack.push(c);
            }
        }
    }

    stack
        .iter()
        .map(|&v| match v {
            '(' => 1_u64,
            '[' => 2_u64,
            '{' => 3_u64,
            '<' => 4_u64,
            c => panic!("Invalid stack value! ({:?})", c),
        })
        .rev()
        .fold(0_u64, |acc, v| acc * 5 + v)
}

fn syntax_error_score_of_line(line: &str) -> u32 {
    let mut stack = Vec::new();

    for c in line.chars() {
        let stackfirst = *stack.last().unwrap_or(&'_');

        match (stackfirst, c) {
            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                let _ = stack.pop().unwrap();
            }
            (_, ')') => {
                return 3;
            }
            (_, ']') => {
                return 57;
            }
            (_, '}') => {
                return 1197;
            }
            (_, '>') => {
                return 25_137;
            }
            (_, _) => {
                stack.push(c);
            }
        }
    }

    0
}
