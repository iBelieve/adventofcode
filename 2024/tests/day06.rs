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

#[derive(Clone, Copy)]
enum CellState {
    NotVisited,
    Visited(Direction),
    Obstacle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

enum ExitStatus {
    LeftMap { distinct_locations: u32 },
    StuckInLoop,
}

type Grid = Vec<Vec<CellState>>;
type Location = (usize, usize);

fn evaluate(grid: &mut Grid, starting_location: Location) -> ExitStatus {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut location = starting_location;
    let mut direction = Direction::Up;
    let mut distinct_locations = 1;

    println!("Starting at: {:?}", location);

    loop {
        let Some(new_location) = direction.move_one(location, rows, cols) else {
            println!("Left map, all done!");
            break ExitStatus::LeftMap { distinct_locations };
        };

        let cell = &mut grid[new_location.0][new_location.1];

        match cell {
            CellState::NotVisited => {
                distinct_locations += 1;
                location = new_location;
                *cell = CellState::Visited(direction);
            }
            CellState::Visited(prev_direction) => {
                if *prev_direction == direction {
                    break ExitStatus::StuckInLoop;
                } else {
                    location = new_location;
                }
            }
            CellState::Obstacle => {
                direction = direction.turn_right();
            }
        }
    }
}

fn parse_input(input: &str) -> (Grid, Location) {
    let mut starting_location = None;

    let grid: Grid = input
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
                        '^' => CellState::Visited(Direction::Up),
                        '#' => CellState::Obstacle,
                        _ => panic!("unexpected character: {}", ch),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (grid, starting_location.unwrap())
}

fn part1(input: &str) -> u32 {
    let (mut grid, starting_location) = parse_input(input);

    if let ExitStatus::LeftMap { distinct_locations } = evaluate(&mut grid, starting_location) {
        distinct_locations
    } else {
        panic!("expected to leave the map");
    }
}

fn part2(input: &str) -> u32 {
    let (grid, starting_location) = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut distinct_obstructions = 0;

    for row in 0..rows {
        for col in 0..cols {
            if let CellState::Obstacle = grid[row][col] {
                continue;
            }

            let mut new_grid = grid.clone();
            new_grid[row][col] = CellState::Obstacle;

            if let ExitStatus::StuckInLoop = evaluate(&mut new_grid, starting_location) {
                distinct_obstructions += 1;
            }
        }
    }

    distinct_obstructions
}

#[test]
fn day06_part1_sample() {
    assert_eq!(part1(SAMPLE_INPUT), 41);
}

#[test]
fn day06_part1() {
    assert_eq!(part1(INPUT), 5312);
}

#[test]
fn day06_part2_sample() {
    assert_eq!(part2(SAMPLE_INPUT), 6);
}

#[test]
fn day06_part2() {
    assert_eq!(part2(INPUT), 1748);
}
