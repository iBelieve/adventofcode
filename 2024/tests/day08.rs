use gcd;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const SAMPLE_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

const INPUT: &str = include_str!("day08.txt");

fn parse_input(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, usize, usize) {
    let mut antennas = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;

    for (row, line) in input.trim_end().split('\n').enumerate() {
        rows += 1;

        if cols == 0 {
            cols = line.len();
        }

        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push((row as i32, col as i32));
            }
        }
    }

    (antennas, rows, cols)
}

fn part1(input: &str) -> usize {
    let (antennas, rows, cols) = parse_input(input);
    let mut antinodes = HashSet::new();

    for (_freq, positions) in antennas {
        for pair in positions.iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);

            let anti_a = (a.0 - dx, a.1 - dy);
            let anti_b = (b.0 + dx, b.1 + dy);

            antinodes.insert(anti_a);
            antinodes.insert(anti_b);
        }
    }

    antinodes
        .iter()
        .filter(|(row, col)| *row >= 0 && *row < rows as i32 && *col >= 0 && *col < cols as i32)
        .count()
}

fn reduce(a: i32, b: i32) -> (i32, i32) {
    let gcd = gcd::binary_u32(a.abs() as u32, b.abs() as u32) as i32;

    (a / gcd, b / gcd)
}

fn part2(input: &str) -> usize {
    let (antennas, rows, cols) = parse_input(input);
    let mut antinodes = HashSet::new();

    let is_valid =
        |(row, col): (i32, i32)| row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32;

    for (_freq, positions) in antennas {
        for pair in positions.iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let (dx, dy) = reduce(b.0 - a.0, b.1 - a.1);
            let mut a = *a;
            let mut b = *b;

            while is_valid(a) {
                antinodes.insert(a);
                a = (a.0 - dx, a.1 - dy);
            }

            while is_valid(b) {
                antinodes.insert(b);
                b = (b.0 + dx, b.1 + dy);
            }
        }
    }

    antinodes.iter().count()
}

#[test]
fn day08_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 14);
}

#[test]
fn day08_part1() {
    assert_eq!(part1(INPUT), 396);
}

#[test]
fn day08_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 34);
}

#[test]
fn day08_part2() {
    assert_eq!(part2(INPUT), 1200);
}
