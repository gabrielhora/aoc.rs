use std::collections::HashMap;
use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn new(map: HashMap<&str, &str>) -> Self {
        Self {
            byr: map.get("byr").and_then(|v| v.parse().ok()),
            iyr: map.get("iyr").and_then(|v| v.parse().ok()),
            eyr: map.get("eyr").and_then(|v| v.parse().ok()),
            hgt: map.get("hgt").map(|v| v.to_string()),
            hcl: map.get("hcl").map(|v| v.to_string()),
            ecl: map.get("ecl").map(|v| v.to_string()),
            pid: map.get("pid").map(|v| v.to_string()),
        }
    }
}

fn part1() -> usize {
    parse().iter()
        .filter(|p| p.byr.is_some()
            && p.iyr.is_some()
            && p.eyr.is_some()
            && p.hgt.is_some()
            && p.hcl.is_some()
            && p.ecl.is_some()
            && p.pid.is_some())
        .count()
}

fn part2() -> usize {
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").expect("invalid regex");
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").expect("invalid regex");
    let ecl_re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").expect("invalid regex");
    let pid_re = Regex::new(r"^[0-9]{9}$").expect("invalid regex");

    parse().iter()
        .filter(|p|
            p.byr.is_some_and(|v| (1920..=2002).contains(&v))
                && p.iyr.is_some_and(|v| (2010..=2020).contains(&v))
                && p.eyr.is_some_and(|v| (2020..=2030).contains(&v))
                && p.hgt.as_ref().is_some_and(|v|
                match hgt_re.captures(&v) {
                    Some(caps) => {
                        let value: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                        let unit = caps.get(2).unwrap().as_str();
                        match unit {
                            "cm" => (150..=193).contains(&value),
                            "in" => (59..=76).contains(&value),
                            _ => false,
                        }
                    }
                    None => false,
                })
                && p.hcl.as_ref().is_some_and(|v| hcl_re.is_match(&v))
                && p.ecl.as_ref().is_some_and(|v| ecl_re.is_match(&v))
                && p.pid.as_ref().is_some_and(|v| pid_re.is_match(&v))
        )
        .count()
}

fn parse() -> Vec<Passport> {
    fs::read_to_string("src/y2020/day4.txt")
        .expect("failed to parse input")
        .replace("\n\n", "$$")
        .replace('\n', " ")
        .replace("$$", "\n")
        .lines()
        .map(parse_passport)
        .collect()
}

fn parse_passport(passport: &str) -> Passport {
    let map = {
        let fields: Vec<(&str, &str)> = passport
            .split_whitespace()
            .map(|f| {
                let parts: Vec<&str> = f.split(':').collect();
                assert_eq!(parts.len(), 2);
                (parts[0], parts[1])
            })
            .collect();

        let mut map = HashMap::new();
        for (name, value) in fields {
            map.insert(name, value);
        }
        map
    };
    Passport::new(map)
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