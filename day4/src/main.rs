use std::cmp::Ordering;

struct Game {
    winning_numbers: Vec<u8>,
    my_numbers: Vec<u8>,
}

impl Game {
    fn from_string(line: &str) -> Self {
        let mut line = line.split(':').nth(1).unwrap().split('|');
        let winning_numbers = line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let my_numbers = line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Game {
            winning_numbers,
            my_numbers,
        }
    }

    fn number_of_wins(&self) -> u32 {
        let mut my_number_index = 0;
        let mut winning_number_index = 0;
        let mut num_wins = 0;

        let mut my_numbers = self.my_numbers.clone();
        let mut winning_numbers = self.winning_numbers.clone();
        my_numbers.sort();
        winning_numbers.sort();

        while my_number_index < my_numbers.len() && winning_number_index < winning_numbers.len() {
            let first_my = my_numbers[my_number_index];
            let first_winning = winning_numbers[winning_number_index];

            match first_winning.cmp(&first_my) {
                Ordering::Equal => {
                    num_wins += 1;
                    my_number_index += 1;
                    winning_number_index += 1;
                }
                Ordering::Greater => {
                    my_number_index += 1;
                }
                Ordering::Less => {
                    winning_number_index += 1;
                }
            }
        }
        num_wins
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(Game::from_string)
        .map(|game| game.number_of_wins())
        .filter(|n| n > &0)
        .map(|n| 2_u32.pow(n - 1))
        .sum()
}

fn part_two(input: &str) -> u32 {
    let games: Vec<_> = input.lines().map(Game::from_string).collect();

    let mut card_counts = vec![1; games.len()];
    let mut total_cards = 0;
    let mut idx = 0;

    while idx < card_counts.len() {
        total_cards += card_counts[idx];
        let wins = games[idx].number_of_wins();
        for i in idx + 1..=(idx + wins as usize).min(card_counts.len() - 1) {
            card_counts[i] += card_counts[idx];
        }
        idx += 1;
    }

    total_cards
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
        assert_eq!(part_one(input), 13)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 30)
    }
}
