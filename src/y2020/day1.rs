use std::fs;

fn part1() -> i32 {
    let data = parse();
    for x in &data {
        for y in &data {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    0
}

fn part2() -> i32 {
    let data = parse();
    for x in &data {
        for y in &data {
            for z in &data {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    0
}

fn parse() -> Vec<i32> {
    fs::read_to_string("src/y2020/day1.txt")
        .expect("failed to read file")
        .lines()
        .map(|l| l.parse::<i32>().expect("failed to parse line to i32"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("part 1: {}", part1())
    }

    #[test]
    fn test_part2() {
        println!("part 2: {}", part2())
    }
}
