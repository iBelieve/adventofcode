use itertools::Itertools;

const SAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

const INPUT: &str = include_str!("day01.txt");

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.trim_end().split("\n\n").map(|elf_lines| {
        elf_lines
            .split("\n")
            .map(|line| line.parse::<u32>().expect("Unable to parse line"))
            .sum()
    })
}

fn part1(input: &str) -> u32 {
    parse_input(input).max().expect("No input provided")
}

fn part2(input: &str) -> u32 {
    parse_input(input).k_largest(3).sum()
}

#[test]
fn day01_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 24000);
}

#[test]
fn day01_part1() {
    assert_eq!(part1(INPUT), 67027);
}

#[test]
fn day01_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 45000);
}

#[test]
fn day01_part2() {
    assert_eq!(part2(INPUT), 197291);
}
