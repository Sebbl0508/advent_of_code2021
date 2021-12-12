use crate::assets::ASSETS_FOLDER;

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
    let bytes = ASSETS_FOLDER.get_file("day10.example").unwrap().contents();
    let string = String::from_utf8(bytes.to_vec()).unwrap();

    part_01(&string);
    // part_02(&string);
}

fn part_01(input: &String) {
    for line in input.lines() {
        let mut stack = Vec::new();

        for c in line.chars() {
            todo!();
        }
    }
}

fn part_02(input: &String) {
    todo!();
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
            '{' | '}' =>  ChunkType::Curly,
            '<' | '>' => ChunkType::Pointy,
            invalid => panic!("Invalid char: {:?}", invalid),
        }
    }
}