use std::str::FromStr;
use strum_macros::EnumString;

const SAMPLE_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

const INPUT: &str = include_str!("day02.txt");

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
enum Command {
    Forward,
    Down,
    Up,
}

fn parse_input(input: &str) -> impl Iterator<Item = (Command, u32)> + '_ {
    input
        .trim_end()
        .split('\n')
        .map(|line| line.split_once(' ').expect("Unable to parse line"))
        .map(|(command, value)| {
            (
                Command::from_str(command).expect("Unable to parse command"),
                value.parse::<u32>().expect("Unable to parse command value"),
            )
        })
}

fn part1(input: &str) -> u32 {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for (command, value) in parse_input(input) {
        match command {
            Command::Forward => {
                horizontal_position += value;
            }
            Command::Down => {
                depth += value;
            }
            Command::Up => {
                depth -= value;
            }
        }
    }

    horizontal_position * depth
}

fn part2(input: &str) -> u32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (command, value) in parse_input(input) {
        match command {
            Command::Forward => {
                horizontal_position += value;
                depth += aim * value;
            }
            Command::Down => {
                aim += value;
            }
            Command::Up => {
                aim -= value;
            }
        }
    }

    horizontal_position * depth
}

#[test]
fn day01_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 150);
}

#[test]
fn day01_part1() {
    assert_eq!(part1(INPUT), 1693300);
}

#[test]
fn day01_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 900);
}

#[test]
fn day01_part2() {
    assert_eq!(part2(INPUT), 1857958050);
}
