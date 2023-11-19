use std::borrow::Cow;
use std::collections::HashMap;
use std::time::Instant;
use regex::Regex;

#[derive(Debug)]
struct Bag {
    name: String,
    count: usize,
    contents: Vec<Bag>,
}

fn part1(input: &str) -> usize {
    let bags = parse_bags(input);
    bags
        .values()
        .fold(
            0,
            |acc, bag| if find_bag(&bags, &bag.name, "shiny gold") { acc + 1 } else { acc },
        )
}

fn part2(input: &str) -> usize {
    count_bags(&parse_bags(input), "shiny gold")
}

fn count_bags(bags: &HashMap<String, Bag>, lookup: &str) -> usize {
    let Some(bag) = bags.get(lookup) else { return 0; };
    bag.contents
        .iter()
        .fold(0, |acc, b| acc + b.count * (1 + count_bags(bags, &b.name)))
}

fn parse_bags(input: &str) -> HashMap<String, Bag> {
    let re = Regex::new(r"^\s*(?<name>\w+ \w+)|(?<qty>\d+)\s*(?<content>\w+ \w+)").expect("invalid regex");

    input
        .split('\n')
        .map(|line| {
            let mut bag = Bag { name: String::new(), count: 0, contents: vec![] };
            for cap in re.captures_iter(line) {
                if let Some(name) = cap.name("name") {
                    bag.name = name.as_str().to_string();
                }
                if let Some(qty) = cap.name("qty") {
                    bag.contents.push(Bag {
                        name: cap.name("content").unwrap().as_str().to_string(),
                        count: qty.as_str().parse::<usize>().expect("should be a number"),
                        contents: vec![],
                    })
                }
            }
            bag
        })
        .map(|b| (b.name.to_string(), b))
        .collect()
}

fn find_bag(bags: &HashMap<String, Bag>, start_bag: &str, lookup: &str) -> bool {
    if let Some(bag) = bags.get(start_bag) {
        for sub_bag in &bag.contents {
            if sub_bag.name == lookup || find_bag(bags, &sub_bag.name, lookup) {
                return true;
            }
        }
        false
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part1() {
        println!("part 1: {:?}", part1(&read_file("src/y2020/day7.txt")));
    }

    #[test]
    fn test_part2() {
        println!("part 2: {:?}", part2(&read_file("src/y2020/day7.txt")));
    }
}