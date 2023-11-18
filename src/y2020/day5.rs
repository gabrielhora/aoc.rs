use std::fs;

fn part1() -> usize {
    parse()
        .iter()
        .map(|l| seat_id(l))
        .max()
        .unwrap_or(0)
}

fn part2() -> usize {
    let seat_ids = {
        let mut seat_ids = parse()
            .iter()
            .map(|l| seat_id(l))
            .collect::<Vec<usize>>();
        seat_ids.sort();
        seat_ids
    };

    for window in seat_ids.windows(2) {
        if window[1] - window[0] > 1 {
            return window[0] + 1;
        }
    }

    panic!("should have found something")
}

fn seat_id(line: &str) -> usize {
    let binary = line
        .replace(['F', 'L'], "0")
        .replace(['B', 'R'], "1");
    usize::from_str_radix(&binary, 2).unwrap_or(0)
}

fn parse() -> Vec<String> {
    fs::read_to_string("src/y2020/day5.txt")
        .expect("failed to read file")
        .split('\n')
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("part 1: {:?}", part1())
    }

    #[test]
    fn test_part2() {
        println!("part 2: {:?}", part2())
    }
}