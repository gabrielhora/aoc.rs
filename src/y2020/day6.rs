use std::collections::HashSet;

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(count_unique)
        .sum::<usize>()
}

fn count_unique(group: &str) -> usize {
    group.replace('\n', "").chars().collect::<HashSet<char>>().len()
}

fn part2(input: &str) -> usize {
    let groups: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|g| g.split("\n").collect())
        .collect();

    groups.iter().map(|g| count_intersections(g)).sum()
}

fn count_intersections(group: &[&str]) -> usize {
    let sets: Vec<HashSet<char>> = group
        .iter()
        .map(|&g| HashSet::from_iter(g.chars()))
        .collect();

    let intersection: HashSet<char> = sets.iter()
        .fold(
            sets[0].clone(),
            |acc, set| acc.intersection(set).cloned().collect(),
        );

    intersection.len()
}

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|g| g.split('\n').map(String::from).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part1() {
        println!("part 1: {:?}", part1(&read_file("src/y2020/day6.txt")));
    }

    #[test]
    fn test_part2() {
        println!("part 2: {:?}", part2(&read_file("src/y2020/day6.txt")));
    }
}