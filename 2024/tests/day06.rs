const SAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

const INPUT: &str = include_str!("day06.txt");

enum CellState {
    NotVisited,
    Visited,
    Obstacle,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_one(
        &self,
        location: (usize, usize),
        rows: usize,
        cols: usize,
    ) -> Option<(usize, usize)> {
        let (row, col) = location;

        let (new_row, new_col) = match self {
            Self::Up => (row.checked_sub(1)?, col),
            Self::Down => (row.checked_add(1)?, col),
            Self::Left => (row, col.checked_sub(1)?),
            Self::Right => (row, col.checked_add(1)?),
        };

        if new_row >= rows || new_col >= cols {
            None
        } else {
            Some((new_row, new_col))
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut starting_location = None;

    let mut grid = input
        .trim_end()
        .split('\n')
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| {
                    if ch == '^' {
                        starting_location = Some((row, col));
                    }

                    match ch {
                        '.' => CellState::NotVisited,
                        '^' => CellState::Visited,
                        '#' => CellState::Obstacle,
                        _ => panic!("unexpected character: {}", ch),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut location = starting_location.unwrap();
    let mut direction = Direction::Up;
    let mut distinct_locations = 1;

    println!("Starting at: {:?}", location);

    loop {
        let Some(new_location) = direction.move_one(location, rows, cols) else {
            println!("Left map, all done!");
            break;
        };

        let cell = &mut grid[new_location.0][new_location.1];

        match cell {
            CellState::NotVisited => {
                distinct_locations += 1;
                location = new_location;
                *cell = CellState::Visited;
                // println!(
                //     "Moving to new location: {:?} ({} distinct)",
                //     location, distinct_locations
                // );
            }
            CellState::Visited => {
                location = new_location;
                // println!(
                //     "Moving to visited location: {:?} ({} distinct)",
                //     location, distinct_locations
                // );
            }
            CellState::Obstacle => {
                direction = direction.turn_right();
                // println!("Encountered obstacle at: {:?}", new_location);
            }
        }

        // break;
    }

    distinct_locations
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[test]
fn day06_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 41);
}

#[test]
fn day06_part1() {
    assert_eq!(part1(INPUT), 5312);
}

// #[test]
// fn day06_part2_sample() {
//     assert_eq!(part2(SAMPLE_INPUT), 0);
// }

// #[test]
// fn day06_part2() {
//     assert_eq!(part2(INPUT), 0);
// }
