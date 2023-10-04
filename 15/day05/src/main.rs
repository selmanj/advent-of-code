use std::env;
use std::fs;

fn has_three_vowels(input: &str) -> bool {
    input
        .chars()
        .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
        .take(3)
        .count()
        >= 3
}

fn has_twice_in_row(input: &str) -> bool {
    input
        .chars()
        .zip(input.chars().skip(1))
        .any(|(c1, c2)| c1 == c2)
}

fn no_banned_strings(input: &str) -> bool {
    !input.contains("ab") && !input.contains("cd") && !input.contains("pq") && !input.contains("xy")
}

fn is_nice(input: &str) -> bool {
    has_three_vowels(input) && has_twice_in_row(input) && no_banned_strings(input)
}

fn has_repeat(input: &str) -> bool {
    if input.len() < 4 {
        return false;
    }

    for idx in 1..input.len() - 2 {
        let left = &input[idx - 1..idx + 1];
        for i in idx + 1..input.len() - 1 {
            let right = &input[i..i + 2];
            if *left == *right {
                return true;
            }
        }
    }
    return false;
}

fn has_pair(input: &str) -> bool {
    if input.len() < 3 {
        return false;
    }
    for i in 0..input.len()-2 {
        if &input[i..i+1] == &input[i+2..i+3] {
            return true;
        }
    }
    return false;
}

fn is_nice2(input: &str) -> bool {
    has_repeat(input) && has_pair(input)
}

fn run(input: &str) {
    let mut nice1_count = 0;
    let mut nice2_count = 0;
    for line in input.lines() {
        if is_nice(line) {
            nice1_count += 1
        }
        if is_nice2(line) {
            nice2_count += 1
        }
    }

    println!("{}", nice1_count);
    println!("{}", nice2_count);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_three_vowels() {
        assert_eq!(has_three_vowels("aa"), false);
        assert_eq!(has_three_vowels("aei"), true);
        assert_eq!(has_three_vowels("abbae"), true);
    }

    #[test]
    fn test_has_twice_in_row() {
        assert_eq!(has_twice_in_row("xy"), false);
        assert_eq!(has_twice_in_row("zz"), true);
        assert_eq!(has_twice_in_row("abdbc"), false);
        assert_eq!(has_twice_in_row("abdbbc"), true);
    }

    #[test]
    fn test_is_nice() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_is_nice2() {
        assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice2("xxyxx"), true);
        assert_eq!(is_nice2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice2("ieodomkazucvgmuy"), false);
    }
}
