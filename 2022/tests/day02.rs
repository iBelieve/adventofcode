use std::str::FromStr;

use strum::EnumString;

const SAMPLE_INPUT: &str = "A Y
B X
C Z";

const INPUT: &str = include_str!("day02.txt");

#[derive(EnumString, Clone, Copy)]
enum Move {
    #[strum(serialize = "A", serialize = "X")]
    Rock,
    #[strum(serialize = "B", serialize = "Y")]
    Paper,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors,
}

#[derive(EnumString, Clone, Copy)]
enum DesiredResult {
    #[strum(serialize = "X")]
    Lose,
    #[strum(serialize = "Y")]
    Draw,
    #[strum(serialize = "Z")]
    Win,
}

impl Move {
    fn shape_score(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn parse_input_part1(input: &str) -> impl Iterator<Item = (Move, Move)> + '_ {
    input
        .trim_end()
        .split('\n')
        .map(|line| line.split_once(' ').expect("Unable to parse line"))
        .map(|(a, b)| {
            (
                Move::from_str(a).expect("Unable to parse opponent move"),
                Move::from_str(b).expect("Unable to parse your move"),
            )
        })
}

fn parse_input_part2(input: &str) -> impl Iterator<Item = (Move, DesiredResult)> + '_ {
    input
        .trim_end()
        .split('\n')
        .map(|line| line.split_once(' ').expect("Unable to parse line"))
        .map(|(a, b)| {
            (
                Move::from_str(a).expect("Unable to parse opponent move"),
                DesiredResult::from_str(b).expect("Unable to parse desired result"),
            )
        })
}

fn compute_score(opponent_move: Move, your_move: Move) -> u32 {
    const SCORE_LOST: u32 = 0;
    const SCORE_DRAW: u32 = 3;
    const SCORE_WON: u32 = 6;

    let round_score = match (your_move, opponent_move) {
        (Move::Rock, Move::Rock) => SCORE_DRAW,
        (Move::Rock, Move::Paper) => SCORE_LOST,
        (Move::Rock, Move::Scissors) => SCORE_WON,

        (Move::Paper, Move::Rock) => SCORE_WON,
        (Move::Paper, Move::Paper) => SCORE_DRAW,
        (Move::Paper, Move::Scissors) => SCORE_LOST,

        (Move::Scissors, Move::Rock) => SCORE_LOST,
        (Move::Scissors, Move::Paper) => SCORE_WON,
        (Move::Scissors, Move::Scissors) => SCORE_DRAW,
    };

    your_move.shape_score() + round_score
}

fn compute_your_move(opponent_move: Move, desired_result: DesiredResult) -> Move {
    match (opponent_move, desired_result) {
        (Move::Rock, DesiredResult::Lose) => Move::Scissors,
        (Move::Rock, DesiredResult::Draw) => Move::Rock,
        (Move::Rock, DesiredResult::Win) => Move::Paper,

        (Move::Paper, DesiredResult::Lose) => Move::Rock,
        (Move::Paper, DesiredResult::Draw) => Move::Paper,
        (Move::Paper, DesiredResult::Win) => Move::Scissors,

        (Move::Scissors, DesiredResult::Lose) => Move::Paper,
        (Move::Scissors, DesiredResult::Draw) => Move::Scissors,
        (Move::Scissors, DesiredResult::Win) => Move::Rock,
    }
}

fn part1(input: &str) -> u32 {
    parse_input_part1(input)
        .map(|(opponent_move, your_move)| compute_score(opponent_move, your_move))
        .sum()
}

fn part2(input: &str) -> u32 {
    parse_input_part2(input)
        .map(|(opponent_move, desired_move)| {
            (
                opponent_move,
                compute_your_move(opponent_move, desired_move),
            )
        })
        .map(|(opponent_move, your_move)| compute_score(opponent_move, your_move))
        .sum()
}

#[test]
fn day02_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 15);
}

#[test]
fn day02_part1() {
    assert_eq!(part1(INPUT), 14264);
}

#[test]
fn day02_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 12);
}

#[test]
fn day02_part2() {
    assert_eq!(part2(INPUT), 12382);
}
