use itertools::Itertools;
use std::collections::HashSet;

const SAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

const INPUT: &str = include_str!("day03.txt");

fn priority(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        (c as u32) - ('a' as u32) + 1
    } else if c >= 'A' && c <= 'Z' {
        (c as u32) - ('A' as u32) + 27
    } else {
        panic!("Only a-z and A-Z were expected");
    }
}

fn unique_item(a: &str, b: &str) -> char {
    a.chars()
        .collect::<HashSet<char>>()
        .intersection(&b.chars().collect())
        .copied()
        .exactly_one()
        .expect("No unique char found")
}

fn unique_in_chunk<'a>(iter: impl Iterator<Item = &'a str>) -> char {
    iter.map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|acc, line| acc.intersection(&line).copied().collect())
        .expect("Nothing in chunk")
        .into_iter()
        .exactly_one()
        .expect("No unique char found")
}

fn part1(input: &str) -> u32 {
    input
        .trim_end()
        .split('\n')
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| unique_item(a, b))
        .map(|c| priority(c))
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .trim_end()
        .split('\n')
        .chunks(3)
        .into_iter()
        .map(|chunk| unique_in_chunk(chunk))
        .map(|c| priority(c))
        .sum()
}

#[test]
fn day03_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 157);
}

#[test]
fn day03_part1() {
    assert_eq!(part1(INPUT), 7795);
}

#[test]
fn day03_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 70);
}

#[test]
fn day03_part2() {
    assert_eq!(part2(INPUT), 2703);
}
