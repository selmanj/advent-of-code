use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T {
    std::cmp::min(std::cmp::min(v1, v2), v3)
}

fn wrapping_paper(l: u32, w: u32, h: u32) -> u32 {
    (l * w * 2) + (l * h * 2) + (w * h * 2) + min3(l * w, w * h, h * l)
}

fn ribbon(l: u32, w: u32, h: u32) -> u32 {
    min3(2*l + 2*h, 2*h + 2*w, 2*w + 2*l) + (l*w*h)
}

fn run(input: &str) {
    lazy_static! {
        static ref PKG: Regex = Regex::new(r"^([\d]+)x([\d]+)x([\d]+)$").unwrap();
    }
    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in input.lines() {
        let captures = PKG.captures(line).expect("input should be valid");
        let l: u32 = captures.get(1).unwrap().as_str().parse::<u32>().unwrap(); // we expect input to be valid
        let w: u32 = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let h: u32 = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();

        total_paper += wrapping_paper(l, w, h);
        total_ribbon += ribbon(l, w, h);
    }

    println!("{}", total_paper);
    println!("{}", total_ribbon);
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
