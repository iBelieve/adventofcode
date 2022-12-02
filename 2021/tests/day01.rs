use itertools::Itertools;

const SAMPLE_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

const INPUT: &str = include_str!("day01.txt");

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .trim_end()
        .split('\n')
        .map(|line| line.parse::<u32>().expect("Unable to parse line"))
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[test]
fn day01_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 7);
}

#[test]
fn day01_part1() {
    assert_eq!(part1(INPUT), 1711);
}

#[test]
fn day01_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 5);
}

#[test]
fn day01_part2() {
    assert_eq!(part2(INPUT), 1743);
}
