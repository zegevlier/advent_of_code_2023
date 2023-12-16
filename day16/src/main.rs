#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CellType {
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
    RightAngledMirror,
    LeftAngledMirror,
}

impl CellType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            '/' => Self::RightAngledMirror,
            '\\' => Self::LeftAngledMirror,
            _ => panic!("Got invalid character in input"),
        }
    }
}

#[derive(Clone, Debug)]
struct Cell {
    cell_type: CellType,
    energized: bool,
    directions_taken: Vec<Direction>,
}

impl Cell {
    fn from_char(c: char) -> Self {
        Cell {
            cell_type: CellType::from_char(c),
            energized: false,
            directions_taken: vec![],
        }
    }
}

#[derive(Clone)]
struct Board {
    board: Vec<Vec<Cell>>,
}

impl Board {
    fn from_str(input: &str) -> Self {
        Board {
            board: input
                .lines()
                .map(|line| line.chars().map(Cell::from_char).collect())
                .collect(),
        }
    }

    fn take_step(&mut self, point: (i32, i32), direction: Direction) {
        if point.0 < 0 || point.1 < 0 {
            return;
        }

        let current_cell = match self.board.get_mut(point.0 as usize) {
            Some(line) => match line.get_mut(point.1 as usize) {
                Some(point) => point,
                None => return,
            },
            None => return,
        };

        current_cell.energized = true;

        let directions = match current_cell.cell_type {
            CellType::Empty => {
                vec![direction]
            }
            CellType::HorizontalSplitter => match direction {
                Direction::North | Direction::South => {
                    vec![Direction::East, Direction::West]
                }
                Direction::East | Direction::West => {
                    vec![direction]
                }
            },
            CellType::VerticalSplitter => match direction {
                Direction::East | Direction::West => {
                    vec![Direction::North, Direction::South]
                }
                Direction::North | Direction::South => {
                    vec![direction]
                }
            },
            CellType::RightAngledMirror => {
                vec![match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                }]
            }
            CellType::LeftAngledMirror => {
                vec![match direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                }]
            }
        };

        let directions: Vec<&Direction> = directions
            .iter()
            .filter(|dir| !current_cell.directions_taken.contains(dir))
            .collect();

        current_cell
            .directions_taken
            .extend(directions.iter().copied());

        for direction in directions {
            let index_offset = direction.get_index_offsets();
            self.take_step(
                (point.0 + index_offset.0, point.1 + index_offset.1),
                *direction,
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_index_offsets(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

fn part_one(input: &str) -> usize {
    let mut board = Board::from_str(input);

    board.take_step((0, 0), Direction::East);

    board.board.iter().flatten().filter(|c| c.energized).count()
}

fn part_two(input: &str) -> u32 {
    let board = Board::from_str(input);

    let mut current_max = 0;
    let board_height = board.board.len();
    let board_width = board.board[0].len();

    for i in 0..board_height {
        let mut left_column = board.clone();
        left_column.take_step((i as i32, 0), Direction::East);
        current_max = current_max.max(
            left_column
                .board
                .iter()
                .flatten()
                .filter(|c| c.energized)
                .count(),
        );
        let mut right_column = board.clone();
        right_column.take_step((i as i32, (board_width - 1) as i32), Direction::West);
        current_max = current_max.max(
            right_column
                .board
                .iter()
                .flatten()
                .filter(|c| c.energized)
                .count(),
        );
    }

    for i in 0..board_width {
        let mut top_column = board.clone();
        top_column.take_step((0, i as i32), Direction::South);
        current_max = current_max.max(
            top_column
                .board
                .iter()
                .flatten()
                .filter(|c| c.energized)
                .count(),
        );
        let mut bottom_column = board.clone();
        bottom_column.take_step(((board_height - 1) as i32, i as i32), Direction::North);
        current_max = current_max.max(
            bottom_column
                .board
                .iter()
                .flatten()
                .filter(|c| c.energized)
                .count(),
        );
    }

    current_max as u32
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_one(input), 46)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 51)
    }
}
