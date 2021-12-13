use crate::assets::ASSETS_FOLDER;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ChunkType {
    Round,  // ()
    Square, // []
    Curly,  // {}
    Pointy, // <>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ChunkPart {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Part {
    part: ChunkPart,
    typ: ChunkType,
}

pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day10.input").unwrap().contents();
    let string = String::from_utf8(bytes.to_vec()).unwrap();

    // part_01(&string);
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

impl Part {
    pub fn new(inp_char: char) -> Self {
        let chunk_type = ChunkType::from_char(inp_char);
        let chunk_part = match inp_char {
            '(' | '[' | '{' | '<' => ChunkPart::Opening,
            ')' | ']' | '}' | '>' => ChunkPart::Closing,
            invalid => panic!("Invalid char: {:?}", invalid),
        };

        Self {
            part: chunk_part,
            typ: chunk_type,
        }
    }
}

impl ChunkType {
    fn get_opening(&self) -> char {
        match self {
            ChunkType::Round => '(',
            ChunkType::Square => '[',
            ChunkType::Curly => '{',
            ChunkType::Pointy => '<',
        }
    }

    fn get_closing(&self) -> char {
        match self {
            ChunkType::Round => ')',
            ChunkType::Square => ']',
            ChunkType::Curly => '}',
            ChunkType::Pointy => '>',
        }
    }

    fn from_char(c: char) -> ChunkType {
        match c {
            '(' | ')' => ChunkType::Round,
            '[' | ']' => ChunkType::Square,
            '{' | '}' => ChunkType::Curly,
            '<' | '>' => ChunkType::Pointy,
            invalid => panic!("Invalid char: {:?}", invalid),
        }
    }
}
