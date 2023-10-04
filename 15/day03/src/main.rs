use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn run(input: &str) {
    let mut cur = Coord { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(cur.clone());
    for c in input.chars() {
        cur = match c {
            '^' => Coord {
                x: cur.x,
                y: cur.y + 1,
            },
            '>' => Coord {
                x: cur.x + 1,
                y: cur.y,
            },
            '<' => Coord {
                x: cur.x - 1,
                y: cur.y,
            },
            'v' => Coord {
                x: cur.x,
                y: cur.y - 1,
            },
            _ => panic!("Got unexpected character {}", c),
        };
        visited.insert(cur.clone());
    }

    println!("{}", visited.len());
}

fn run2(input: &str) {
    let mut cur_santa = Coord { x: 0, y: 0 };
    let mut cur_robo = Coord { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(cur_santa.clone());

    for (i, c) in input.chars().enumerate() {
        let prev = if i % 2 == 0 { &cur_santa } else { &cur_robo };
        let cur = match c {
            '^' => Coord {
                x: prev.x,
                y: prev.y + 1,
            },
            '>' => Coord {
                x: prev.x + 1,
                y: prev.y,
            },
            '<' => Coord {
                x: prev.x - 1,
                y: prev.y,
            },
            'v' => Coord {
                x: prev.x,
                y: prev.y - 1,
            },
            _ => panic!("Got unexpected character {}", c),
        };
        visited.insert(cur.clone());
        if i % 2 == 0 {
            cur_santa = cur;
        } else {
            cur_robo = cur;
        }
    }

    println!("{}", visited.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map(|x| x.as_str()).unwrap_or("input.txt");

    let contents = match fs::read_to_string(filename) {
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            std::process::exit(65) // EX_DATAERR
        }
        Ok(f) => f,
    };

    run(contents.as_str());
    run2(contents.as_str());
}
