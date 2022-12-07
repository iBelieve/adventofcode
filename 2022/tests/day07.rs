use itertools::Itertools;
use path_clean::PathClean;
use regex::Regex;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

const SAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

const INPUT: &str = include_str!("day07.txt");

fn parse_input(input: &str) -> impl Iterator<Item = (&str, Vec<&str>)> + '_ {
    input
        .trim_end()
        .strip_prefix("$")
        .expect("Missing first command prompt prefix")
        .split("\n$")
        .map(|chunk| {
            let mut lines = chunk
                .strip_prefix(" ")
                .expect("Invalid command prompt prefix")
                .split('\n')
                .collect_vec();

            let cmd = lines.drain(0..1).next().expect("Missing command line");

            (cmd, lines)
        })
}

fn get_dir_sizes(input: &str) -> HashMap<PathBuf, u32> {
    let cd_regex = Regex::new(r"cd (.+)").unwrap();

    let mut dirs: HashMap<PathBuf, u32> = HashMap::new();
    let mut cwd = PathBuf::from("/");

    for (cmd, lines) in parse_input(input) {
        if let Some(caps) = cd_regex.captures(cmd) {
            cwd = cwd.join(&caps[1]).clean();
        } else if cmd == "ls" {
            let dir_size: u32 = lines
                .into_iter()
                .map(|line| {
                    let (left, _right) = line
                        .split_once(' ')
                        .unwrap_or_else(|| panic!("Invalid LS output line: {line}"));

                    if left == "dir" {
                        0
                    } else {
                        left.parse::<u32>().expect("Invalid file size")
                    }
                })
                .sum();

            let mut path = Some(cwd.as_path());

            while let Some(ref dir) = path {
                let entry = dirs.entry(dir.to_path_buf()).or_default();

                *entry += dir_size;

                path = dir.parent();
            }
        }
    }

    dirs
}

fn part1(input: &str) -> u32 {
    let dirs = get_dir_sizes(input);

    dirs.values()
        .copied()
        .filter(|dir_size| *dir_size <= 100000)
        .sum()
}

fn part2(input: &str) -> u32 {
    let dirs = get_dir_sizes(input);
    let total_space = 70000000;
    let needed_space = 30000000;
    let used_space = dirs[Path::new("/")];
    let free_space = total_space - used_space;
    let amount_to_free = needed_space - free_space;

    dirs.values()
        .copied()
        .sorted()
        .filter(|size| *size > amount_to_free)
        .next()
        .expect("No single directory can free up enough space")
}

#[test]
fn day07_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 95437);
}

#[test]
fn day07_part1() {
    assert_eq!(part1(INPUT), 1334506);
}

#[test]
fn day07_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 24933642);
}

#[test]
fn day07_part2() {
    assert_eq!(part2(INPUT), 7421137);
}
