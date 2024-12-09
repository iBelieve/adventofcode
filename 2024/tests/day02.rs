const SAMPLE_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

const INPUT: &str = include_str!("day02.txt");

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}

struct Entry {
    level: i32,
    direction: Option<Direction>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + use<'_> {
    input.trim_end().split('\n').map(|line| {
        line.split_whitespace()
            .map(|lvl| lvl.parse::<i32>().unwrap())
            .collect()
    })
}

fn is_valid_report(report: &[i32]) -> bool {
    let Some((first, rest)) = report.split_first() else {
        return false;
    };

    let mut prev = Entry {
        level: *first,
        direction: None,
    };

    for level in rest {
        let level = *level;

        let diff = level - prev.level;

        let direction = if diff >= 1 && diff <= 3 {
            Direction::Increasing
        } else if diff >= -3 && diff <= -1 {
            Direction::Decreasing
        } else {
            return false;
        };

        if prev.direction.is_none() || prev.direction == Some(direction) {
            prev = Entry {
                level,
                direction: Some(direction),
            };
        } else {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> u32 {
    parse_input(input)
        .filter(|report| is_valid_report(report))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    parse_input(input)
        .filter(|report| {
            for index in 0..report.len() {
                let mut report = report.to_vec();
                report.remove(index);

                if is_valid_report(&report) {
                    return true;
                }
            }

            false
        })
        .count() as u32
}

#[test]
fn day02_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 2);
}

#[test]
fn day02_part1() {
    assert_eq!(part1(INPUT), 287);
}

#[test]
fn day02_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 4);
}

#[test]
fn day02_part2() {
    assert_eq!(part2(INPUT), 354);
}
