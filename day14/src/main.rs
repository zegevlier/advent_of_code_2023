#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    RoundRock,
    CubeRock,
    Empty,
}

fn part_one(input: &str) -> u32 {
    let board: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Cell::RoundRock,
                    '#' => Cell::CubeRock,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut total = 0;

    for col in 0..board[0].len() {
        let line: Vec<_> = (0..board.len()).map(|i| board[i][col]).collect();

        let mut seq_start_idx = 0;
        let mut round_rock_count = 0;
        for (i, cell) in line.iter().enumerate() {
            match cell {
                Cell::RoundRock => {
                    round_rock_count += 1;
                }
                Cell::CubeRock => {
                    total += (board.len() - seq_start_idx - round_rock_count + 1
                        ..=board.len() - seq_start_idx)
                        .sum::<usize>() as u32;
                    seq_start_idx = i + 1;
                    round_rock_count = 0;
                }
                Cell::Empty => {}
            }
        }
        total += (board.len() - seq_start_idx - round_rock_count + 1..=board.len() - seq_start_idx)
            .sum::<usize>() as u32;
    }

    total
}

fn cycle(board: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut out_board = vec![vec![Cell::Empty; board.first().unwrap().len()]; board.len()];

    // North
    for col in 0..board[0].len() {
        let line: Vec<_> = (0..board.len()).map(|i| board[i][col]).collect();

        let mut seq_start_idx = 0;
        let mut round_rock_count = 0;
        for (i, cell) in line.iter().enumerate() {
            match cell {
                Cell::RoundRock => {
                    round_rock_count += 1;
                }
                Cell::CubeRock => {
                    out_board[i][col] = Cell::CubeRock;
                    (seq_start_idx..seq_start_idx + round_rock_count).for_each(|j| {
                        out_board[j][col] = Cell::RoundRock;
                    });
                    seq_start_idx = i + 1;
                    round_rock_count = 0;
                }
                Cell::Empty => {}
            }
        }
        (seq_start_idx..seq_start_idx + round_rock_count).for_each(|j| {
            out_board[j][col] = Cell::RoundRock;
        });
    }

    let board = out_board;
    let mut out_board = vec![vec![Cell::Empty; board.first().unwrap().len()]; board.len()];

    // West
    for row in 0..board.len() {
        let line: Vec<_> = (0..board[0].len()).map(|i| board[row][i]).collect();

        let mut seq_start_idx = 0;
        let mut round_rock_count = 0;
        for (i, cell) in line.iter().enumerate() {
            match cell {
                Cell::RoundRock => {
                    round_rock_count += 1;
                }
                Cell::CubeRock => {
                    out_board[row][i] = Cell::CubeRock;
                    (seq_start_idx..seq_start_idx + round_rock_count).for_each(|j| {
                        out_board[row][j] = Cell::RoundRock;
                    });
                    seq_start_idx = i + 1;
                    round_rock_count = 0;
                }
                Cell::Empty => {}
            }
        }
        (seq_start_idx..seq_start_idx + round_rock_count).for_each(|j| {
            out_board[row][j] = Cell::RoundRock;
        });
    }

    let board = out_board;
    let mut out_board = vec![vec![Cell::Empty; board.first().unwrap().len()]; board.len()];

    // South
    for col in 0..board[0].len() {
        let line: Vec<_> = (0..board.len()).map(|i| board[i][col]).collect();

        let mut seq_start_idx = board.len();
        let mut round_rock_count = 0;
        for (i, cell) in line.iter().enumerate().rev() {
            match cell {
                Cell::RoundRock => {
                    round_rock_count += 1;
                }
                Cell::CubeRock => {
                    out_board[i][col] = Cell::CubeRock;
                    (seq_start_idx - round_rock_count..seq_start_idx).for_each(|j| {
                        out_board[j][col] = Cell::RoundRock;
                    });
                    seq_start_idx = i;
                    round_rock_count = 0;
                }
                Cell::Empty => {}
            }
        }
        (seq_start_idx - round_rock_count..seq_start_idx).for_each(|j| {
            out_board[j][col] = Cell::RoundRock;
        });
    }

    let board = out_board;
    let mut out_board = vec![vec![Cell::Empty; board.first().unwrap().len()]; board.len()];

    // West
    for row in 0..board.len() {
        let line: Vec<_> = (0..board[0].len()).map(|i| board[row][i]).collect();

        let mut seq_start_idx = board.len();
        let mut round_rock_count = 0;
        for (i, cell) in line.iter().enumerate().rev() {
            match cell {
                Cell::RoundRock => {
                    round_rock_count += 1;
                }
                Cell::CubeRock => {
                    out_board[row][i] = Cell::CubeRock;
                    (seq_start_idx - round_rock_count..seq_start_idx).for_each(|j| {
                        out_board[row][j] = Cell::RoundRock;
                    });
                    seq_start_idx = i;
                    round_rock_count = 0;
                }
                Cell::Empty => {}
            }
        }
        (seq_start_idx - round_rock_count..seq_start_idx).for_each(|j| {
            out_board[row][j] = Cell::RoundRock;
        });
    }

    // for line in out_board.iter() {
    //     for cell in line {
    //         match cell {
    //             Cell::RoundRock => print!("O"),
    //             Cell::CubeRock => print!("#"),
    //             Cell::Empty => print!("."),
    //         }
    //     }
    //     println!()
    // }

    out_board
}

fn calculate_board_load(board: Vec<Vec<Cell>>) -> u32 {
    board
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .filter_map(|c| match c {
                    Cell::CubeRock | Cell::Empty => None,
                    Cell::RoundRock => Some((board.len() - i) as u32),
                })
                .sum::<u32>()
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut board: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Cell::RoundRock,
                    '#' => Cell::CubeRock,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let cycle_count = 1000000000;
    let mut old_boards = vec![];
    let mut cycle_start_idx = 0;
    let mut cycle_end_idx = 0;
    for i in 0..cycle_count {
        old_boards.push(board.clone());
        board = cycle(board);
        if let Some(n) = old_boards.iter().position(|b| b.eq(&board)) {
            cycle_start_idx = n;
            cycle_end_idx = i + 1;
            break;
        }
    }

    let billionth_board_idx =
        cycle_start_idx + ((cycle_count - cycle_start_idx) % (cycle_end_idx - cycle_start_idx));

    calculate_board_load(old_boards[billionth_board_idx].to_owned())
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
        assert_eq!(part_one(input), 136)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 64)
    }
}
