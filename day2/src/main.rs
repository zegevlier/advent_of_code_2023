use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl CubeColor {
    fn from_string(input: &str) -> Self {
        match input {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("Invalid color input"),
        }
    }
}

#[derive(Debug)]
struct CubeSet {
    set: Vec<(CubeColor, u32)>, // Cube color and amount
}

impl CubeSet {
    fn from_string(input: &str) -> Self {
        Self {
            set: input
                .split(',')
                .map(|input| {
                    let mut input = input.split_whitespace();
                    let amount = input.next().unwrap().parse().unwrap();
                    let color = CubeColor::from_string(input.next().unwrap());
                    (color, amount)
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn from_string(input: &str) -> Self {
        let mut input = input.strip_prefix("Game ").unwrap().split(':');

        Game {
            id: input.next().unwrap().parse().unwrap(),
            sets: input
                .next()
                .unwrap()
                .split(';')
                .map(CubeSet::from_string)
                .collect(),
        }
    }
}

const MAX_ALLOWED: [(CubeColor, u32); 3] = [
    (CubeColor::Red, 12),
    (CubeColor::Green, 13),
    (CubeColor::Blue, 14),
];

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(Game::from_string)
        .filter(|game| {
            !game.sets.iter().any(|set| {
                set.set.iter().any(|(color, amount)| {
                    for (max_color, max_amount) in MAX_ALLOWED {
                        if color == &max_color && amount > &max_amount {
                            return true;
                        }
                    }
                    false
                })
            })
        })
        .map(|game| game.id)
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(Game::from_string)
        .map(|game| {
            game.sets.iter().fold(
                {
                    let mut map = HashMap::new();
                    map.insert(CubeColor::Red, 0);
                    map.insert(CubeColor::Green, 0);
                    map.insert(CubeColor::Blue, 0);
                    map
                },
                |min_required_set, set| {
                    let mut min_required_set = min_required_set;
                    set.set.iter().for_each(|(color, amount)| {
                        if min_required_set.get(color).unwrap() < amount {
                            min_required_set.insert(*color, *amount);
                        }
                    });
                    min_required_set
                },
            )
        })
        .map(|set| set.iter().map(|i| i.1).product::<u32>())
        .sum()
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
        assert_eq!(part_one(input), 8)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 2286)
    }
}
