const SAMPLE_INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

const INPUT: &str = include_str!("day10.txt");

fn dfs(grid: &Vec<Vec<char>>, row: isize, col: isize, expected_digit: u8) -> usize {
    if row < 0 || row as usize >= grid.len() || col < 0 || col as usize >= grid[0].len() {
        return 0;
    }

    if grid[row as usize][col as usize].to_digit(10).unwrap() as u8 != expected_digit {
        return 0;
    }

    if expected_digit == 9 {
        return 1;
    }

    dfs(grid, row - 1, col, expected_digit + 1)
        + dfs(grid, row + 1, col, expected_digit + 1)
        + dfs(grid, row, col - 1, expected_digit + 1)
        + dfs(grid, row, col + 1, expected_digit + 1)
}

fn part1(input: &str) -> usize {
    let grid = input
        .trim_end()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut score = 0;

    for row in 0..rows {
        for col in 0..cols {
            let trail_score = dfs(&grid, row as isize, col as isize, 0);

            if trail_score > 0 {
                println!("Trail score: {}", trail_score);
                score += trail_score;
            }
        }
    }

    score
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[test]
fn day10_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 36);
}

// #[test]
// fn day10_part1() {
//     assert_eq!(part1(INPUT), 0);
// }

// #[test]
// fn day10_part2_sample() {
//     assert_eq!(part2(SAMPLE_INPUT), 0);
// }

// #[test]
// fn day10_part2() {
//     assert_eq!(part2(INPUT), 0);
// }
