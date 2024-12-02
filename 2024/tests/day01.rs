use counter::Counter;

const SAMPLE_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

const INPUT: &str = include_str!("day01.txt");

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.trim_end().split('\n') {
        let mut parts = line.split_whitespace();
        let l = parts
            .next()
            .unwrap()
            .parse::<u32>()
            .expect("Unable to parse left");
        let r = parts
            .next()
            .unwrap()
            .parse::<u32>()
            .expect("Unable to parse right");

        left.push(l);
        right.push(r);

        assert!(parts.next().is_none());
    }

    (left, right)
}

fn part1(input: &str) -> u32 {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part2(input: &str) -> u32 {
    let (left, right) = parse_input(input);

    let right_counts = right.into_iter().collect::<Counter<u32>>();

    left.into_iter()
        .map(|l| l * (right_counts[&l] as u32))
        .sum()
}

#[test]
fn day01_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 11);
}

#[test]
fn day01_part1() {
    assert_eq!(part1(INPUT), 2031679);
}

#[test]
fn day01_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 31);
}

#[test]
fn day01_part2() {
    assert_eq!(part2(INPUT), 19678534);
}
