#[derive(PartialEq, Eq)]
enum Character {
    Digit(u8),
    Dot,
    Symbol,
}

const ASCII_NUMBER_MASK: u8 = 0b00001111;

fn parse_input_part1(input: &str) -> Vec<Vec<Character>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c.is_numeric() {
                        Character::Digit(c as u8 & ASCII_NUMBER_MASK)
                    } else if c == '.' {
                        Character::Dot
                    } else {
                        Character::Symbol
                    }
                })
                .collect()
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    let input = parse_input_part1(input);
    let mut total = 0;
    let mut is_part_number = false;
    let mut current_number = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            match character {
                Character::Digit(num) => {
                    current_number = current_number * 10 + (*num as u32);
                    if is_part_number {
                        continue;
                    }
                    let above_is_symbol = i > 0 && input[i - 1][j] == Character::Symbol;
                    let left_is_symbol = j > 0 && input[i][j - 1] == Character::Symbol;
                    let below_is_symbol =
                        i < input.len() - 1 && input[i + 1][j] == Character::Symbol;

                    let (above_left_is_symbol, below_left_is_symbol) =
                        if current_number == (*num as u32) {
                            let above_left_is_symbol =
                                i > 0 && j > 0 && input[i - 1][j - 1] == Character::Symbol;
                            let below_left_is_symbol = i < input.len() - 1
                                && j > 0
                                && input[i + 1][j - 1] == Character::Symbol;
                            (above_left_is_symbol, below_left_is_symbol)
                        } else {
                            (false, false)
                        };

                    if above_is_symbol
                        || left_is_symbol
                        || below_is_symbol
                        || above_left_is_symbol
                        || below_left_is_symbol
                    {
                        is_part_number = true;
                        continue;
                    }
                }
                Character::Dot | Character::Symbol => {
                    if character == &Character::Symbol {
                        is_part_number = true;
                    } else if current_number != 0 {
                        let above_is_symbol = i > 0 && input[i - 1][j] == Character::Symbol;
                        let below_is_symbol =
                            i < input.len() - 1 && input[i + 1][j] == Character::Symbol;
                        if above_is_symbol || below_is_symbol {
                            is_part_number = true;
                        }
                    }
                    if is_part_number {
                        total += current_number;
                    }
                    is_part_number = false;
                    current_number = 0;
                }
            }
        }
    }
    total
}

#[derive(PartialEq, Eq, Debug)]
enum GearboxCharacter {
    Digit(u8),
    Star,
    Misc,
}

impl GearboxCharacter {
    fn is_digit(&self) -> bool {
        matches!(self, GearboxCharacter::Digit(_))
    }

    fn get_number(&self) -> Option<u8> {
        match self {
            GearboxCharacter::Digit(n) => Some(*n),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct PotentialGear {
    could_be_gear: bool,
    num1: Option<u32>,
    num2: Option<u32>,
}

impl PotentialGear {
    fn new() -> Self {
        PotentialGear {
            could_be_gear: true,
            num1: None,
            num2: None,
        }
    }

    fn ratio(&self) -> u32 {
        if self.could_be_gear && self.num1.is_some() && self.num2.is_some() {
            self.num1.unwrap() * self.num2.unwrap()
        } else {
            0
        }
    }

    fn add_adjacent_number(&mut self, number: u32) {
        if !self.could_be_gear {
            return;
        }
        if self.num1.is_none() {
            self.num1 = Some(number)
        } else if self.num2.is_none() {
            self.num2 = Some(number)
        } else {
            self.could_be_gear = false;
        }
    }
}

fn parse_input_part2(input: &str) -> Vec<Vec<GearboxCharacter>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c.is_numeric() {
                        GearboxCharacter::Digit(c as u8 & ASCII_NUMBER_MASK)
                    } else if c == '*' {
                        GearboxCharacter::Star
                    } else {
                        GearboxCharacter::Misc
                    }
                })
                .collect()
        })
        .collect()
}

fn find_complete_number(input: &Vec<GearboxCharacter>, i: usize) -> (u32, usize, usize) {
    let mut min_i = i;
    while min_i > 0 && input[min_i - 1].is_digit() {
        min_i -= 1;
    }
    let mut max_i = i;
    while max_i < input.len() - 1 && input[max_i + 1].is_digit() {
        max_i += 1;
    }

    let total_number = input
        .iter()
        .take(max_i + 1)
        .skip(min_i)
        .fold(0, |acc, c| acc * 10 + c.get_number().unwrap() as u32);
    (total_number, min_i, max_i)
}

fn part_two(input: &str) -> u32 {
    let input = parse_input_part2(input);
    let mut total = 0;

    for (i, line) in input.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            match character {
                GearboxCharacter::Star => {
                    let mut gear = PotentialGear::new();
                    let max_j = (j + 1).min(line.len() - 1);
                    for line in input.iter().take(i + 2).skip(i - 1) {
                        let mut lj = (j - 1).max(0);
                        while lj <= max_j {
                            if line[lj].is_digit() {
                                let (number, _min_j, new_max_j) = find_complete_number(line, lj);
                                lj = new_max_j;
                                gear.add_adjacent_number(number);
                            }
                            lj += 1;
                        }
                    }
                    total += gear.ratio()
                }
                GearboxCharacter::Digit(_) => {}
                GearboxCharacter::Misc => {}
            }
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
        assert_eq!(part_one(input), 4361)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 467835)
    }
}
