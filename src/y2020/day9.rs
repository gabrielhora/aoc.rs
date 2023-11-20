fn part1(input: &str) -> usize {
    let nums = parse(input);
    let preemble_size = 25;
    for i in preemble_size..nums.len() {
        let num = nums[i];
        let prev = &nums[i - preemble_size..i];
        if !find_in_sums(prev, num) {
            return num;
        }
    }
    0
}

fn part2(input: &str) -> usize {
    let nums = parse(input);
    let lookup = 22406676; // result of part1(input)
    let sum_set = find_sum_set(&nums, lookup);
    let smallest = sum_set.iter().min().unwrap_or(&0);
    let largest = sum_set.iter().max().unwrap_or(&0);
    smallest + largest
}

fn find_sum_set(coll: &[usize], lookup: usize) -> Vec<usize> {
    for i in 0..coll.len() {
        let mut sum = coll[i];
        let mut set = Vec::from([sum]);

        for j in coll.iter().skip(i + 1) {
            sum += j;
            set.push(*j);
            if sum == lookup {
                return set;
            }
            if sum > lookup {
                break;
            }
        }
    }
    Vec::new()
}

fn find_in_sums(sums: &[usize], lookup: usize) -> bool {
    for i in 0..sums.len() {
        for j in 0..sums.len() {
            if sums[i] + sums[j] == lookup {
                return true;
            }
        }
    }
    false
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split('\n')
        .map(|l| l.parse::<usize>().expect("should be a number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn test_part1() {
        println!("part 1: {:?}", part1(&read_file("src/y2020/day9.txt")));
    }

    #[test]
    fn test_part2() {
        println!("part 2: {:?}", part2(&read_file("src/y2020/day9.txt")));
    }
}
