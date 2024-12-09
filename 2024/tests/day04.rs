const SAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const INPUT: &str = include_str!("day04.txt");

fn part1(input: &str) -> u32 {
    let letters = input
        .trim_end()
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = letters.len();
    let cols = letters[0].len();

    let mut xmas_count = 0;

    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for row in 0..rows {
        for col in 0..cols {
            'outer: for (dx, dy) in directions.iter() {
                let mut x = col as i32;
                let mut y = row as i32;

                for letter in "XMAS".chars() {
                    if x < 0 || x >= cols as i32 || y < 0 || y >= rows as i32 {
                        continue 'outer;
                    }

                    if letters[y as usize][x as usize] != letter {
                        continue 'outer;
                    }

                    x += dx;
                    y += dy;
                }

                xmas_count += 1;
            }
        }
    }

    xmas_count
}

fn part2(input: &str) -> u32 {
    let letters = input
        .trim_end()
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = letters.len();
    let cols = letters[0].len();

    let mut xmas_count = 0;

    let shapes = ["MMASS", "MSASM", "SSAMM", "SMAMS"];
    let offsets = [(0, 0), (0, 2), (1, 1), (2, 2), (2, 0)];

    for row in 0..rows - 2 {
        for col in 0..cols - 2 {
            'outer: for shape in shapes {
                for (ch, offset) in shape.chars().zip(offsets.iter()) {
                    let x = col + offset.0;
                    let y = row + offset.1;

                    if letters[y][x] != ch {
                        continue 'outer;
                    }
                }

                xmas_count += 1;
            }
        }
    }

    xmas_count
}

#[test]
fn day04_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 18);
}

#[test]
fn day04_part1() {
    assert_eq!(part1(INPUT), 2390);
}

#[test]
fn day04_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 9);
}

#[test]
fn day04_part2() {
    assert_eq!(part2(INPUT), 1809);
}
