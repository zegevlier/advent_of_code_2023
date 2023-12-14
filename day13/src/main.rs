fn part_one(input: &str) -> usize {
    let mut total = 0;
    for board in input.split("\r\n\r\n") {
        let board: Vec<Vec<_>> = board.lines().map(|l| l.chars().collect()).collect();
        let rows = board.len();
        let cols = board.first().unwrap().len();

        let mut found = false;

        for row in 1..rows {
            let max_check = (2 * row - 1).min(rows - 1);
            let min_check = 0.max(row - (max_check - row) - 1);
            if (min_check..row).all(|i| {
                let offset = 2 * (row - i) - 1;
                (0..cols).all(|j| board[i][j] == board[i + offset][j])
            }) {
                total += row * 100;
                found = true;
                break;
            }
        }
        if found {
            continue;
        }

        for col in 1..cols {
            let max_check = (2 * col - 1).min(cols - 1);
            let min_check = 0.max(col - (max_check - col) - 1);
            if (min_check..col).all(|i| {
                let offset = 2 * (col - i) - 1;
                (0..rows).all(|j| board[j][i] == board[j][i + offset])
            }) {
                total += col;
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
    }
    total
}

fn part_two(input: &str) -> usize {
    let mut total = 0;
    for board in input.split("\r\n\r\n") {
        let board: Vec<Vec<_>> = board.lines().map(|l| l.chars().collect()).collect();
        let rows = board.len();
        let cols = board.first().unwrap().len();

        let mut found = false;

        for row in 1..rows {
            let max_check = (2 * row - 1).min(rows - 1);
            let min_check = 0.max(row - (max_check - row) - 1);
            if (min_check..row)
                .map(|i| {
                    let offset = 2 * (row - i) - 1;
                    (0..cols)
                        .filter(|j| board[i][*j] != board[i + offset][*j])
                        .count()
                })
                .sum::<usize>()
                == 1
            {
                total += row * 100;
                found = true;
                break;
            }
        }
        if found {
            continue;
        }

        for col in 1..cols {
            let max_check = (2 * col - 1).min(cols - 1);
            let min_check = 0.max(col - (max_check - col) - 1);
            if (min_check..col)
                .map(|i| {
                    let offset = 2 * (col - i) - 1;
                    (0..rows)
                        .filter(|j| board[*j][i] != board[*j][i + offset])
                        .count()
                })
                .sum::<usize>()
                == 1
            {
                total += col;
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
    }
    total
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
        assert_eq!(part_one(input), 405)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 400)
    }
}
