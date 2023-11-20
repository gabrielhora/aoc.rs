use regex::Regex;

#[derive(Debug, Clone)]
struct Command {
    op: String,
    num: isize,
}

#[derive(Debug)]
struct LoopingError {
    acc: isize,
}

fn part1(input: &str) -> isize {
    execute(0, &mut Vec::new(), 0, &parse(input))
        .expect_err("should exit with looping error")
        .acc
}

fn part2(input: &str) -> isize {
    fix_loop(&parse(input))
}

fn execute(
    acc: isize,
    visited: &mut Vec<usize>,
    index: usize,
    cmds: &[Command],
) -> Result<isize, LoopingError> {
    if index == cmds.len() {
        return Ok(acc);
    }

    for v in visited.iter() {
        if *v == index {
            return Err(LoopingError { acc });
        }
    }

    visited.push(index);

    let cmd = &cmds[index];
    match cmd.op.as_str() {
        "nop" => execute(acc, visited, index + 1, cmds),
        "acc" => execute(acc + cmd.num, visited, index + 1, cmds),
        "jmp" => execute(acc, visited, (index as isize + cmd.num) as usize, cmds),
        _ => panic!("invalid instruction"),
    }
}

fn fix_loop(cmds: &[Command]) -> isize {
    let (mut changed_cmds, mut i) = change_next_nop_or_jmp(cmds, 0);
    loop {
        match execute(0, &mut Vec::new(), 0, &changed_cmds) {
            Ok(acc) => return acc,
            Err(_) => (changed_cmds, i) = change_next_nop_or_jmp(cmds, i + 1),
        }
    }
}

fn change_next_nop_or_jmp(cmds: &[Command], index: usize) -> (Vec<Command>, usize) {
    let mut new_cmds = cmds.to_vec();
    for i in index..new_cmds.len() {
        let cmd = &mut new_cmds[i];
        if cmd.op == "nop" {
            cmd.op = "jmp".to_string();
            return (new_cmds, i);
        }
        if cmd.op == "jmp" {
            cmd.op = "nop".to_string();
            return (new_cmds, i);
        }
    }
    (new_cmds, 0)
}

fn parse(input: &str) -> Vec<Command> {
    let line_regex = Regex::new(r"(\w+)\s([+-]\d+)").expect("invalid regex");

    input
        .split('\n')
        .map(|l| {
            if let Some(caps) = line_regex.captures(l) {
                let count = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
                Command {
                    op: caps.get(1).unwrap().as_str().to_string(),
                    num: count,
                }
            } else {
                panic!("all lines should match regex");
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part1() {
        println!("part 1: {:?}", part1(&read_file("src/y2020/day8.txt")));
    }

    #[test]
    fn test_part2() {
        println!("part 2: {:?}", part2(&read_file("src/y2020/day8.txt")));
    }
}
