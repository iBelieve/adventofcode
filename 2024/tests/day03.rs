use std::{iter::Peekable, str::Chars};

const SAMPLE_INPUT_1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

const SAMPLE_INPUT_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

const INPUT: &str = include_str!("day03.txt");

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let mut it = input.chars().peekable();

    while it.peek().is_some() {
        if match_str(&mut it, "mul(") {
            if let Some(res) = parse_mul(&mut it) {
                sum += res;
            }
        } else {
            it.next();
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut it = input.chars().peekable();
    let mut enabled = true;

    while it.peek().is_some() {
        if enabled && match_str(&mut it, "mul(") {
            if let Some(res) = parse_mul(&mut it) {
                sum += res;
            }
        } else if match_str(&mut it, "do") {
            if match_str(&mut it, "n't()") {
                println!("DON'T!");
                enabled = false;
            } else if match_str(&mut it, "()") {
                println!("DO!");
                enabled = true;
            }
        } else {
            it.next();
        }
    }

    sum
}

fn parse_mul(it: &mut Peekable<Chars>) -> Option<u32> {
    let a = take_num(it)?;

    it.next_if_eq(&',')?;

    let b = take_num(it)?;

    it.next_if_eq(&')')?;

    Some(a * b)
}

fn match_str(it: &mut Peekable<Chars>, s: &str) -> bool {
    for c in s.chars() {
        if it.next_if_eq(&c).is_none() {
            return false;
        }
    }

    true
}

fn take_num(it: &mut Peekable<Chars>) -> Option<u32> {
    let mut num = 0;
    let mut found = false;

    while let Some(c) = it.next_if(|c| c.is_digit(10)) {
        num = num * 10 + c.to_digit(10).unwrap();
        found = true;
    }

    if found {
        Some(num)
    } else {
        None
    }
}

#[test]
fn day03_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT_1), 161);
}

#[test]
fn day03_part1() {
    assert_eq!(part1(INPUT), 180233229);
}

#[test]
fn day03_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT_2), 48);
}

#[test]
fn day03_part2() {
    assert_eq!(part2(INPUT), 95411583);
}
