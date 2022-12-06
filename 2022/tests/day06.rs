use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("day06.txt");

fn puzzle(input: &str, window_size: usize) -> usize {
    let chars = input.chars().collect_vec();

    chars
        .windows(window_size)
        .map(|window| window.into_iter().copied().collect::<HashSet<char>>())
        .take_while(|set| set.len() < window_size)
        .count()
        + window_size
}

fn part1(input: &str) -> usize {
    puzzle(input, 4)
}

fn part2(input: &str) -> usize {
    puzzle(input, 14)
}

#[test]
fn day06_part1_sample() {
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
}

#[test]
fn day06_part1() {
    assert_eq!(part1(INPUT), 1953);
}

#[test]
fn day06_part2_sample() {
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
}

#[test]
fn day06_part2() {
    assert_eq!(part2(INPUT), 2301);
}
