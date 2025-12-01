use std::collections::HashMap;

use itertools::Itertools;

const SAMPLE_INPUT: &str = "2333133121414131402";

const INPUT: &str = include_str!("day09.txt");

fn part1(input: &str) -> usize {
    let mut chars = input.trim_end().chars();
    let mut next_file_id = 0;

    let mut blocks = Vec::new();

    loop {
        let Some(block_count) = chars.next() else {
            break;
        };

        let file_id = next_file_id;
        let block_count = block_count.to_digit(10).unwrap();

        for _ in 0..block_count {
            blocks.push(Some(file_id));
        }

        let Some(free_count) = chars.next() else {
            break;
        };

        let free_count = free_count.to_digit(10).unwrap();

        for _ in 0..free_count {
            blocks.push(None);
        }

        next_file_id += 1;
    }

    let mut l = 0;
    let mut r = blocks.len() - 1;

    while l < r {
        if blocks[l].is_some() {
            l += 1;
            continue;
        }

        if blocks[r].is_none() {
            r -= 1;
            continue;
        }

        blocks.swap(l, r);
        l += 1;
        r -= 1;
    }

    blocks
        .iter()
        .take_while(|block| block.is_some())
        .enumerate()
        .map(|(pos, file_id)| pos * file_id.unwrap())
        .sum()
}

enum Blocks {
    File { id: u32, size: u32 },
    Free { size: u32 },
}

fn part2(input: &str) -> usize {
    let mut chars = input.trim_end().chars();
    let mut next_file_id = 0;

    let mut blocks = Vec::new();

    loop {
        let Some(block_count) = chars.next() else {
            break;
        };

        let file_id = next_file_id;
        let block_count = block_count.to_digit(10).unwrap();

        for _ in 0..block_count {
            blocks.push(Blocks::File {
                id: file_id,
                size: block_count,
            });
        }

        let Some(free_count) = chars.next() else {
            break;
        };

        let free_count = free_count.to_digit(10).unwrap();

        for _ in 0..free_count {
            blocks.push(Blocks::Free { size: block_count });
        }

        next_file_id += 1;
    }

    0
}

#[test]
fn day09_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 1928);
}

#[test]
fn day09_part1() {
    assert_eq!(part1(INPUT), 6432869891895);
}

// #[test]
// fn day09_part2_sample() {
//     assert_eq!(part2(SAMPLE_INPUT), 0);
// }

// #[test]
// fn day09_part2() {
//     assert_eq!(part2(INPUT), 0);
// }
