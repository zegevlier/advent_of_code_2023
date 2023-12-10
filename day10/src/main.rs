#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const CARDINALS: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    fn get_index_offsets(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    NorthEast,
    NorthSouth,
    NorthWest,
    EastSouth,
    EastWest,
    SouthWest,
    Empty,
    Start,
}

impl Tile {
    const NORTH_TILES: [Self; 3] = [Self::NorthEast, Self::NorthSouth, Self::NorthWest];
    const EAST_TILES: [Self; 3] = [Self::NorthEast, Self::EastSouth, Self::EastWest];
    const SOUTH_TILES: [Self; 3] = [Self::NorthSouth, Self::EastSouth, Self::SouthWest];
    const WEST_TILES: [Self; 3] = [Self::NorthWest, Self::EastWest, Self::SouthWest];

    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::EastSouth,
            '.' => Self::Empty,
            'S' => Self::Start,
            _ => panic!("Invalid character in board {}", c),
        }
    }

    fn get_allowed_incoming(dir: &Direction) -> [Self; 3] {
        match dir {
            Direction::North => Self::SOUTH_TILES,
            Direction::East => Self::WEST_TILES,
            Direction::West => Self::EAST_TILES,
            Direction::South => Self::NORTH_TILES,
        }
    }

    fn get_remaining_direction(&self, dir: &Direction) -> Direction {
        match self {
            Tile::NorthEast => {
                if dir == &Direction::North {
                    Direction::East
                } else {
                    Direction::North
                }
            }
            Tile::NorthSouth => {
                if dir == &Direction::North {
                    Direction::South
                } else {
                    Direction::North
                }
            }
            Tile::NorthWest => {
                if dir == &Direction::North {
                    Direction::West
                } else {
                    Direction::North
                }
            }
            Tile::EastSouth => {
                if dir == &Direction::East {
                    Direction::South
                } else {
                    Direction::East
                }
            }
            Tile::EastWest => {
                if dir == &Direction::East {
                    Direction::West
                } else {
                    Direction::East
                }
            }
            Tile::SouthWest => {
                if dir == &Direction::South {
                    Direction::West
                } else {
                    Direction::South
                }
            }
            _ => panic!("No remaining direction possible"),
        }
    }
}

struct Gameboard {
    start_coords: (i32, i32),
    board: Vec<Vec<Tile>>,
}

impl Gameboard {
    fn from_str(input: &str) -> Self {
        let board: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect();

        Gameboard {
            start_coords: board
                .iter()
                .enumerate()
                .find_map(|(i, v)| {
                    let j = v.iter().enumerate().find_map(|(j, v)| {
                        if *v == Tile::Start {
                            Some(j)
                        } else {
                            None
                        }
                    });
                    j.map(|j| (i as i32, j as i32))
                })
                .unwrap(),
            board,
        }
    }

    fn get(&self, pos: (i32, i32)) -> Option<Tile> {
        if pos.0 >= 0
            && pos.1 >= 0
            && pos.0 < self.board.len() as i32
            && pos.1 < self.board.first().unwrap().len() as i32
        {
            Some(self.board[pos.0 as usize][pos.1 as usize])
        } else {
            None
        }
    }

    fn set(&mut self, pos: (i32, i32), tile: Tile) {
        self.board[pos.0 as usize][pos.1 as usize] = tile;
    }
}

fn add_pos(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn part_one(input: &str) -> u32 {
    let board = Gameboard::from_str(input);

    let mut current_coord = (-1, -1);
    let mut prev_direction = Direction::North;
    for direction in Direction::CARDINALS {
        let checking_pos = add_pos(board.start_coords, direction.get_index_offsets());
        if let Some(tile) = board.get(checking_pos) {
            if Tile::get_allowed_incoming(&direction).contains(&tile) {
                current_coord = checking_pos;
                prev_direction = direction.opposite();
                break;
            }
        }
    }

    let mut path_length = 1;
    loop {
        if board.get(current_coord).unwrap() == Tile::Start {
            break;
        }

        let current_tile = board.get(current_coord).unwrap();
        let new_direction = current_tile.get_remaining_direction(&prev_direction);
        current_coord = add_pos(current_coord, new_direction.get_index_offsets());
        prev_direction = new_direction.opposite();

        path_length += 1;
    }

    path_length / 2
}

// Based on https://wrfranklin.org/Research/Short_Notes/pnpoly.html
fn is_in_polygon(polygon_x: &[i32], polygon_y: &[i32], point_x: i32, point_y: i32) -> bool {
    let mut j = polygon_x.len() - 1;
    let mut c = false;

    for i in 0..polygon_x.len() {
        if ((polygon_y[i] > point_y) != (polygon_y[j] > point_y))
            && (point_x
                < (polygon_x[j] - polygon_x[i]) * (point_y - polygon_y[i])
                    / (polygon_y[j] - polygon_y[i])
                    + polygon_x[i])
        {
            c = !c;
        }
        j = i;
    }

    c
}

fn part_two(input: &str) -> u32 {
    let mut board = Gameboard::from_str(input);

    let mut current_coord = (-1, -1);
    let mut prev_direction = Direction::North;

    for direction in Direction::CARDINALS {
        let checking_pos = add_pos(board.start_coords, direction.get_index_offsets());
        if let Some(tile) = board.get(checking_pos) {
            if Tile::get_allowed_incoming(&direction).contains(&tile) {
                current_coord = checking_pos;
                prev_direction = direction.opposite();
                break;
            }
        }
    }

    let mut polygon_x = vec![];
    let mut polygon_y = vec![];

    loop {
        if board.get(current_coord).unwrap() == Tile::Start {
            break;
        }

        let current_tile = board.get(current_coord).unwrap();
        match current_tile {
            Tile::NorthEast | Tile::SouthWest | Tile::NorthWest | Tile::EastSouth => {
                polygon_x.push(current_coord.0);
                polygon_y.push(current_coord.1);
            }
            Tile::EastWest | Tile::NorthSouth => {}
            Tile::Start | Tile::Empty => unreachable!(),
        }
        board.set(current_coord, Tile::Start);

        let new_direction = current_tile.get_remaining_direction(&prev_direction);
        current_coord = add_pos(current_coord, new_direction.get_index_offsets());
        prev_direction = new_direction.opposite();
    }

    board
        .board
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(j, s)| {
                    if s != &&Tile::Start {
                        is_in_polygon(&polygon_x, &polygon_y, i as i32, *j as i32)
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum::<usize>() as u32
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
        let input = include_str!("../test_files/part_one.txt");
        assert_eq!(part_one(input), 8)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_two.txt");
        assert_eq!(part_two(input), 10)
    }
}
