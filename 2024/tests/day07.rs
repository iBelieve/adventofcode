const SAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

const INPUT: &str = include_str!("day07.txt");

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            Self::Concat => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

fn process(input: &str, operators: &[Operator]) -> u64 {
    input
        .trim_end()
        .split('\n')
        .map(|line| {
            let (first, second) = line.split_once(": ").unwrap();
            let target = first.parse::<u64>().unwrap();
            let parts = second
                .split(' ')
                .map(|p| p.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let mut stack = vec![(parts[0], 1)];

            while let Some((result, len)) = stack.pop() {
                let next_part = parts[len];
                let new_len = len + 1;

                for op in operators {
                    let new_result = op.apply(result, next_part);

                    if new_result == target && new_len == parts.len() {
                        return target;
                    } else if new_result <= target && new_len < parts.len() {
                        stack.push((new_result, len + 1));
                    }
                }
            }

            0
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    process(input, &[Operator::Add, Operator::Multiply])
}

fn part2(input: &str) -> u64 {
    process(
        input,
        &[Operator::Add, Operator::Multiply, Operator::Concat],
    )
}

#[test]
fn day07_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 3749);
}

#[test]
fn day07_part1() {
    assert_eq!(part1(INPUT), 850435817339);
}

#[test]
fn day07_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 11387);
}

#[test]
fn day07_part2() {
    assert_eq!(part2(INPUT), 104824810233437);
}
