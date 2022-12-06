use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

const SAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

const INPUT: &str = include_str!("day05.txt");

struct Move {
    count: usize,
    from: char,
    to: char,
}

fn parse_input(
    input: &str,
) -> (
    Vec<char>,
    HashMap<char, Vec<char>>,
    impl Iterator<Item = Move> + '_,
) {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let (starting_stacks_string, procedure_lines) =
        input.split_once("\n\n").expect("Unable to parse input");

    let mut stack_rows = starting_stacks_string
        .split('\n')
        .map(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|mut chunk| chunk.nth(1).unwrap())
                .collect_vec()
        })
        .collect_vec();
    let stack_names = stack_rows
        .pop()
        .expect("Stack lines should have a least one line for stack names");
    let stacks: HashMap<char, Vec<char>> = stack_names
        .iter()
        .enumerate()
        .map(|(stack_index, stack_name)| {
            let mut stack = Vec::new();

            for stack_row in stack_rows.iter().rev() {
                let value = stack_row[stack_index];

                if value != ' ' {
                    stack.push(value);
                }
            }

            (*stack_name, stack)
        })
        .collect();

    let procedures = procedure_lines.trim_end().split('\n').map(move |line| {
        let cap = re
            .captures(line)
            .expect("Procedure line doesn't match regex");

        Move {
            count: cap[1].parse().expect("Unable to parse move count"),
            from: cap[2].parse().expect("Unable to parse move from"),
            to: cap[3].parse().expect("Unable to parse move to"),
        }
    });

    (stack_names, stacks, procedures)
}

fn part1(input: &str) -> String {
    let (stack_names, mut stacks, procedures) = parse_input(input);

    for procedure in procedures {
        for _ in 0..procedure.count {
            let from_stack = stacks
                .get_mut(&procedure.from)
                .expect("From stack not found");
            let value = from_stack.pop().expect("Stack is empty");
            let to_stack = stacks.get_mut(&procedure.to).expect("To stack not found");
            to_stack.push(value);
        }
    }

    stack_names
        .into_iter()
        .map(|stack_name| stacks[&stack_name].last().expect("Stack is empty"))
        .join("")
}

fn part2(input: &str) -> String {
    let (stack_names, mut stacks, procedures) = parse_input(input);

    for procedure in procedures {
        let from_stack = stacks
            .get_mut(&procedure.from)
            .expect("From stack not found");
        let values = from_stack.split_off(from_stack.len() - procedure.count);
        let to_stack = stacks.get_mut(&procedure.to).expect("To stack not found");
        to_stack.extend_from_slice(&values);
    }

    stack_names
        .into_iter()
        .map(|stack_name| stacks[&stack_name].last().expect("Stack is empty"))
        .join("")
}

#[test]
fn day05_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), "CMZ");
}

#[test]
fn day05_part1() {
    assert_eq!(part1(INPUT), "JRVNHHCSJ");
}

#[test]
fn day05_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), "MCD");
}

#[test]
fn day05_part2() {
    assert_eq!(part2(INPUT), "GNFBSBJLH");
}
