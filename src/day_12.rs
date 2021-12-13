use std::rc::Rc;
use itertools::Itertools;
use crate::assets::ASSETS_FOLDER;

#[derive(Debug, Clone, Copy)]
enum CaveKind {
    Big,
    Small,
}

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    kind: CaveKind,
    connections: Vec<String>,
}

#[derive(Debug, Clone)]
struct CavePath {
    path: Vec<String>,
}

#[derive(Debug, Clone)]
struct CaveSystem {
    caves: Vec<Cave>,
    paths: Vec<CavePath>,
}


pub fn run() {
    let bytes = ASSETS_FOLDER.get_file("day12.example").unwrap().contents();
    let string = String::from_utf8(bytes.to_vec()).unwrap();

    part_01(&string);
    // part_02(&string);
}

fn part_01(input: &String) {
    let mut cave_system = CaveSystem::from_input(input);
    cave_system.generate_connections(input);

    cave_system.generate_paths();

    dbg!(cave_system);
}

fn part_02(input: &String) {
    todo!()
}



impl CaveSystem {
    fn from_input(input: &String) -> Self {
        let mut caves = Vec::new();

        for line in input.lines() {
            let line = line.split("-").collect::<Vec<&str>>();

            for name in line {
                if !caves.iter().any(|v: &Cave| v.name == name.to_string()) {
                    caves.push(Cave::from_name(name.to_string()));
                }
            }
        }

        Self {
            caves,
            paths: vec![]
        }
    }

    fn generate_paths(&mut self) {
        todo!();
    }

    fn generate_connections(&mut self, input: &String) {
        for line in input.lines() {
            let line = line.split("-").collect::<Vec<&str>>();
            let from_str = line[0];
            let to_str = line[1];

            if let Some(from) = self.get_cave_by_name(from_str) {
                from.connections.push(to_str.to_string());
            }

            if let Some(to) = self.get_cave_by_name(to_str) {
                to.connections.push(from_str.to_string());
            }
        }
    }

    fn get_cave_by_name(&mut self, name: &str) -> Option<&mut Cave> {
        let name = String::from(name);

        for cave in self.caves.iter_mut() {
            if *cave.name == name {
                return Some(cave);
            }
        }

        return None;
    }
}

impl Cave {
    fn from_name(name: String) -> Self {
        Self {
            name: name.clone(),
            kind: match name.chars().all(|v| v.is_uppercase()) {
                true  => CaveKind::Big,
                false => CaveKind::Small,
            },
            connections: Vec::new(),
        }
    }
}