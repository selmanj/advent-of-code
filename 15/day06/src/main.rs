mod bitvec;
mod lightgrid;

use lazy_static::lazy_static;
use lightgrid::LightGrid;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn run(input: &str) {
    let mut grid = LightGrid::new(1000, 1000);
    lazy_static! {
        static ref TOGGLE_RE: Regex =
            Regex::new(r"^toggle (\d+),(\d+) through (\d+),(\d+)$").unwrap();
        static ref TURN_ON_RE: Regex =
            Regex::new(r"^turn on (\d+),(\d+) through (\d+),(\d+)$").unwrap();
        static ref TURN_OFF_RE: Regex =
            Regex::new(r"^turn off (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    }
    // utility closure for extracting results from REs
    let extract_coords = |captures: regex::Captures| {
        (
            captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        )
    };
    for line in input.lines() {
        if let Some(captures) = TOGGLE_RE.captures(line) {
            let coords = extract_coords(captures);

            grid.toggle(coords.0, coords.1, coords.2, coords.3);
        } else if let Some(captures) = TURN_ON_RE.captures(line) {
            let coords = extract_coords(captures);

            grid.set(coords.0, coords.1, coords.2, coords.3);
        } else if let Some(captures) = TURN_OFF_RE.captures(line) {
            let coords = extract_coords(captures);

            grid.unset(coords.0, coords.1, coords.2, coords.3);
        }
    }

    println!("{}", grid.on_count());

    // for step two, just use a hashmap
    let mut bright_map = HashMap::new();
    for line in input.lines() {
        if let Some(captures) = TOGGLE_RE.captures(line) {
            let coords = extract_coords(captures);

            for y in coords.1..coords.3 + 1 {
                for x in coords.0..coords.2 + 1 {
                    let entry = bright_map.entry((x, y)).or_insert(0);
                    *entry += 2;
                }
            }
        } else if let Some(captures) = TURN_ON_RE.captures(line) {
            let coords = extract_coords(captures);

            for y in coords.1..coords.3 + 1 {
                for x in coords.0..coords.2 + 1 {
                    let entry = bright_map.entry((x, y)).or_insert(0);
                    *entry += 1;
                }
            }
        } else if let Some(captures) = TURN_OFF_RE.captures(line) {
            let coords = extract_coords(captures);

            for y in coords.1..coords.3 + 1 {
                for x in coords.0..coords.2 + 1 {
                    let entry = bright_map.entry((x, y)).or_insert(0);
                    *entry -= 1;
                    if *entry < 0 {
                        *entry = 0;
                    }
                }
            }
        }
    }

    println!("{}", bright_map.values().sum::<i32>());
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
}
