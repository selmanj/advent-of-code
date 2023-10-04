use std::env;
use std::fs;

fn run(input: &str) {
    let mut floor = 0;
    let mut basement_position = None;

    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0, // ignore unknown for now
        };
        if floor == -1 && basement_position == None {
            basement_position = Some(i+1);
        }
    }

    println!("{}", floor);
    println!("{}", basement_position.expect("Should have a dang answer"));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .map(|x| x.as_str())
        .unwrap_or("input.txt");

    let contents = match fs::read_to_string(filename) {
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            std::process::exit(65) // EX_DATAERR
        }
        Ok(f) => f,
    };

    run(contents.as_str());
}
