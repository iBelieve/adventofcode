use std::collections::{HashMap, HashSet};

const SAMPLE_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

const INPUT: &str = include_str!("day05.txt");

fn parse_input(
    input: &str,
) -> (
    HashMap<u32, HashSet<u32>>,
    impl Iterator<Item = Vec<u32>> + use<'_>,
) {
    let (rules_section, updates_section) = input.trim_end().split_once("\n\n").unwrap();

    let mut rules = HashMap::new();

    for line in rules_section.split('\n') {
        let (before, after) = line.split_once('|').unwrap();
        let before: u32 = before.parse().unwrap();
        let after: u32 = after.parse().unwrap();

        rules
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
    }

    let updates = updates_section
        .split('\n')
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect());

    (rules, updates)
}

fn is_valid(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut seen = HashSet::new();

    for page in update {
        if let Some(afters) = rules.get(page) {
            if afters.iter().any(|after| seen.contains(after)) {
                return false;
            }
        }

        seen.insert(page);
    }

    true
}

fn get_middle_page(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn part1(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;

    for update in updates {
        if is_valid(&update, &rules) {
            sum += get_middle_page(&update);
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;

    for mut update in updates {
        if !is_valid(&update, &rules) {
            update.sort_by(|a, b| {
                if rules.get(a).take_if(|afters| afters.contains(b)).is_some() {
                    std::cmp::Ordering::Less
                } else if rules.get(b).take_if(|afters| afters.contains(a)).is_some() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            sum += get_middle_page(&update);
        }
    }

    sum
}

#[test]
fn day05_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 143);
}

#[test]
fn day05_part1() {
    assert_eq!(part1(INPUT), 4135);
}

#[test]
fn day05_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 123);
}

#[test]
fn day05_part2() {
    assert_eq!(part2(INPUT), 5285);
}
