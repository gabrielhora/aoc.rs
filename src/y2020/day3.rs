use std::fs;

fn part1() -> usize {
    let forest = parse();
    assert!(forest.len() > 0);
    count_trees(&forest, 3, 1)
}

fn part2() -> usize {
    let forest = parse();
    assert!(forest.len() > 0);
    count_trees(&forest, 1, 1)
        * count_trees(&forest, 3, 1)
        * count_trees(&forest, 5, 1)
        * count_trees(&forest, 7, 1)
        * count_trees(&forest, 1, 2)
}

fn is_tree(forest: &Vec<String>, x: usize, y: usize) -> bool {
    let width = forest[0].len();
    let x_mod = x % width;
    let char_at_coords = forest[y].chars().nth(x_mod).expect("a letter should be here");
    char_at_coords == '#'
}

fn count_trees(forest: &Vec<String>, right_moves: usize, down_moves: usize) -> usize {
    (0..forest.len())
        .step_by(down_moves)
        .enumerate()
        .filter(|(x, y)| is_tree(forest, x * right_moves, *y))
        .count()
}

fn parse() -> Vec<String> {
    fs::read_to_string("src/y2020/day3.txt")
        .expect("failed to read file")
        .lines()
        .map(String::from)
        .collect()
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