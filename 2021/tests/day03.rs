use itertools::Itertools;
use std::str::{Chars, FromStr};
use strum_macros::EnumString;

const SAMPLE_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

const INPUT: &str = include_str!("day03.txt");

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
enum Command {
    Forward,
    Down,
    Up,
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim_end()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| if c == '1' { 1 } else { 0 })
                .collect_vec()
        })
        .collect_vec()
}

fn get_bit_totals(lines: &Vec<Vec<u32>>) -> Vec<i32> {
    let bit_count = lines.first().expect("No input").len();

    let mut bit_totals = vec![0; bit_count];

    for bits in lines.iter() {
        for (i, bit) in bits.into_iter().enumerate() {
            bit_totals[i] += if *bit == 1 { 1 } else { -1 };
        }
    }

    bit_totals
}

fn get_bit_total(lines: &Vec<Vec<u32>>, index: usize) -> i32 {
    let mut bit_total = 0;

    for bits in lines.iter() {
        bit_total += if bits[index] == 1 { 1 } else { -1 };
    }

    bit_total
}

fn most_common_bits(bit_totals: &Vec<i32>) -> impl Iterator<Item = u32> + '_ {
    bit_totals
        .iter()
        .copied()
        .map(|bit_count| if bit_count >= 0 { 1 } else { 0 })
}

fn least_common_bits(bit_totals: &Vec<i32>) -> impl Iterator<Item = u32> + '_ {
    bit_totals
        .iter()
        .copied()
        .map(|bit_count| if bit_count >= 0 { 0 } else { 1 })
}

fn bits_to_number(bits: impl Iterator<Item = u32>) -> u32 {
    bits.reduce(|acc, value| (acc << 1) + value)
        .expect("No bits")
}

fn find_rating(mut lines: Vec<Vec<u32>>, most_common: bool) -> u32 {
    println!("Lines: {lines:?}");

    let bit_count = lines.first().expect("No input").len();

    for index in 0..bit_count {
        let bit_total = get_bit_total(&lines, index);
        let bit = if most_common {
            if bit_total >= 0 {
                1
            } else {
                0
            }
        } else {
            if bit_total >= 0 {
                0
            } else {
                1
            }
        };

        lines = lines
            .into_iter()
            .filter(|line| line[index] == bit)
            .collect_vec();

        if lines.len() == 1 {
            return bits_to_number(lines[0].iter().copied());
        }
    }

    panic!("Single matching line not found");
}

fn part1(input: &str) -> u32 {
    let lines = parse_input(input);
    let bit_totals = get_bit_totals(&lines);

    let gamma: u32 = most_common_bits(&bit_totals)
        .reduce(|acc, value| (acc << 1) + value)
        .expect("No input");

    let epsilon: u32 = least_common_bits(&bit_totals)
        .reduce(|acc, value| (acc << 1) + value)
        .expect("No input");

    gamma * epsilon
}

fn part2(input: &str) -> u32 {
    let lines = parse_input(input);

    let oxygen_generator_rating = find_rating(lines.clone(), true);
    let co2_scrubber_rating = find_rating(lines, false);

    oxygen_generator_rating * co2_scrubber_rating
}

#[test]
fn day03_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 198);
}

#[test]
fn day03_part1() {
    assert_eq!(part1(INPUT), 3549854);
}

#[test]
fn day03_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 230);
}

#[test]
fn day03_part2() {
    assert_eq!(part2(INPUT), 3765399);
}
