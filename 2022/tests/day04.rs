use anyhow::{Context, Result};
use itertools::Itertools;
use std::{any, collections::HashSet, str::FromStr};

const SAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

const INPUT: &str = include_str!("day04.txt");

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }
    pub fn overlaps(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s
            .split_once('-')
            .context("Unable to split section assignment")?;

        let range = Range {
            start: start
                .parse()
                .context("Unable to parse start of first range")?,
            end: end
                .parse()
                .context("Unable to parse start of first range")?,
        };

        Ok(range)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Range, Range)> + '_ {
    input
        .trim_end()
        .split('\n')
        .map(|line| {
            line.split_once(',')
                .expect("Unable to split pair of assignments")
        })
        .map(|(a, b)| {
            (
                Range::from_str(a).expect("Unable to parse first section assignment"),
                Range::from_str(b).expect("Unable to parse second section assignment"),
            )
        })
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .filter(|(a, b)| a.overlaps(b) || b.overlaps(a))
        .count()
}

#[test]
fn day04_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 2);
}

#[test]
fn day04_part1() {
    assert_eq!(part1(INPUT), 526);
}

#[test]
fn day04_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 4);
}

#[test]
fn day04_part2() {
    assert_eq!(part2(INPUT), 0);
}
