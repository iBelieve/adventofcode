use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

const SAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

const INPUT: &str = include_str!("day08.txt");

fn part1(input: &str) -> usize {
    let heights_grid = input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();
    let grid_width = heights_grid[0].len();
    let grid_height = heights_grid.len();
    let mut visible_grid = vec![vec![false; grid_width]; grid_height];

    for (row_index, row) in heights_grid.iter().enumerate() {
        let mut max_height = -1;

        for (col_index, height) in row.iter().enumerate() {
            if *height > max_height {
                visible_grid[row_index][col_index] = true;
                max_height = *height;
            }
        }

        let mut max_height = -1;

        for (col_index, height) in row.iter().enumerate().rev() {
            if *height > max_height {
                visible_grid[row_index][col_index] = true;
                max_height = *height;
            }
        }
    }

    for col_index in 0..grid_width {
        let mut max_height = -1;

        for row_index in 0..grid_height {
            let height = heights_grid[row_index][col_index];

            if height > max_height {
                visible_grid[row_index][col_index] = true;
                max_height = height;
            }
        }

        let mut max_height = -1;

        for row_index in (0..grid_height).rev() {
            let height = heights_grid[row_index][col_index];

            if height > max_height {
                visible_grid[row_index][col_index] = true;
                max_height = height;
            }
        }
    }

    visible_grid
        .into_iter()
        .map(|row| row.iter().copied().filter(|visible| *visible).count())
        .sum()
}

fn part2(input: &str) -> usize {
    let heights_grid = input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();
    let grid_width = heights_grid[0].len();
    let grid_height = heights_grid.len();

    let mut distance_left_grid = vec![vec![0; grid_width]; grid_height];
    let mut distance_right_grid = vec![vec![0; grid_width]; grid_height];
    let mut distance_top_grid = vec![vec![0; grid_width]; grid_height];
    let mut distance_bottom_grid = vec![vec![0; grid_width]; grid_height];

    for (row_index, row) in heights_grid.iter().enumerate() {
        let mut heights = HashSet::new();
        let mut height_indices = HashMap::new();

        for (col_index, height) in row.iter().enumerate() {
            let same_or_taller_heights = heights.iter().copied().filter(|h| h >= height);

            distance_left_grid[row_index][col_index] = col_index;

            for same_or_taller_height in same_or_taller_heights {
                if let Some(prev_col_index) = height_indices.get(&same_or_taller_height) {
                    let distance = col_index - *prev_col_index;

                    distance_left_grid[row_index][col_index] =
                        distance_left_grid[row_index][col_index].min(distance);
                }
            }

            height_indices.insert(*height, col_index);
            heights.insert(*height);
        }

        let mut heights = HashSet::new();
        let mut height_indices = HashMap::new();

        for (col_index, height) in row.iter().enumerate().rev() {
            let same_or_taller_heights = heights.iter().copied().filter(|h| h >= height);

            distance_right_grid[row_index][col_index] = grid_width - col_index - 1;

            for same_or_taller_height in same_or_taller_heights {
                if let Some(prev_col_index) = height_indices.get(&same_or_taller_height) {
                    let distance = *prev_col_index - col_index;

                    distance_right_grid[row_index][col_index] =
                        distance_right_grid[row_index][col_index].min(distance);
                }
            }

            height_indices.insert(*height, col_index);
            heights.insert(*height);
        }
    }

    for col_index in 0..grid_width {
        let mut heights = HashSet::new();
        let mut height_indices = HashMap::new();

        for row_index in 0..grid_height {
            let height = heights_grid[row_index][col_index];
            let same_or_taller_heights = heights.iter().copied().filter(|h| *h >= height);

            distance_top_grid[row_index][col_index] = row_index;

            for same_or_taller_height in same_or_taller_heights {
                if let Some(prev_row_index) = height_indices.get(&same_or_taller_height) {
                    let distance = row_index - *prev_row_index;

                    distance_top_grid[row_index][col_index] =
                        distance_top_grid[row_index][col_index].min(distance);
                }
            }

            height_indices.insert(height, row_index);
            heights.insert(height);
        }

        let mut heights = HashSet::new();
        let mut height_indices = HashMap::new();

        for row_index in (0..grid_height).rev() {
            let height = heights_grid[row_index][col_index];
            let same_or_taller_heights = heights.iter().copied().filter(|h| *h >= height);

            distance_bottom_grid[row_index][col_index] = grid_height - row_index - 1;

            for same_or_taller_height in same_or_taller_heights {
                if let Some(prev_row_index) = height_indices.get(&same_or_taller_height) {
                    let distance = *prev_row_index - row_index;

                    distance_bottom_grid[row_index][col_index] =
                        distance_bottom_grid[row_index][col_index].min(distance);
                }
            }

            height_indices.insert(height, row_index);
            heights.insert(height);
        }
    }

    let mut scenic_grid = vec![vec![0; grid_width]; grid_height];

    for row_index in 0..grid_height {
        for col_index in 0..grid_width {
            scenic_grid[row_index][col_index] = distance_left_grid[row_index][col_index]
                * distance_right_grid[row_index][col_index]
                * distance_top_grid[row_index][col_index]
                * distance_bottom_grid[row_index][col_index];
        }
    }

    scenic_grid
        .into_iter()
        .map(|row| row.iter().copied().max().unwrap())
        .max()
        .unwrap()
}

#[test]
fn day08_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 21);
}

#[test]
fn day08_part1() {
    assert_eq!(part1(INPUT), 1705);
}

#[test]
fn day08_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 8);
}

#[test]
fn day08_part2() {
    assert_eq!(part2(INPUT), 371200);
}
