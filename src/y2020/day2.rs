use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Line {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn part1() -> usize {
    parse()
        .filter(|l| {
            let count = l.password.chars().filter(|&c| c == l.letter).count();
            (l.min..=l.max).contains(&count)
        })
        .count()
}

fn part2() -> usize {
    parse()
        .filter(|l| {
            let letters: Vec<char> = l.password.chars().collect();
            let first = letters[l.min - 1];
            let second = letters[l.max - 1];
            (first == l.letter) != (second == l.letter)
        })
        .count()
}

fn parse() -> impl Iterator<Item=Line> {
    let line_regex = Regex::new(r"(\d+)-(\d+) (\w+): (.*)").expect("wrong regex");

    let content: Vec<String> = fs::read_to_string("src/y2020/day2.txt")
        .expect("failed to read file")
        .lines()
        .map(String::from)
        .collect();

    content
        .into_iter()
        .filter_map(move |l| {
            line_regex.captures(&l).map(|cap| Line {
                min: cap.get(1).map_or(0, |n| n.as_str().parse().unwrap_or(0)),
                max: cap.get(2).map_or(0, |n| n.as_str().parse().unwrap_or(0)),
                letter: cap.get(3).map_or(' ', |m| m.as_str().chars().next().unwrap_or(' ')),
                password: String::from(cap.get(4).map_or("", |m| m.as_str())),
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("part 1: {}", part1());
    }

    #[test]
    fn test_part2() {
        println!("part 2: {}", part2());
    }
}
