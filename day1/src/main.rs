fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numerics: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
            format!("{}{}", numerics.first().unwrap(), numerics.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

const REPLACEMENTS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut idx = 0;
            let first = 'outer: loop {
                let first_char = line.chars().nth(idx).unwrap();
                if first_char.is_numeric() {
                    break first_char;
                }
                for replacement in REPLACEMENTS {
                    if line.get(idx..).unwrap().starts_with(replacement.0) {
                        break 'outer replacement.1;
                    }
                }
                idx += 1;
            };
            idx = line.len() - 1;
            let last = 'outer: loop {
                let last_char = line.chars().nth(idx).unwrap();
                if last_char.is_numeric() {
                    break last_char;
                }
                for replacement in REPLACEMENTS {
                    if line.get(..idx + 1).unwrap().ends_with(replacement.0) {
                        break 'outer replacement.1;
                    }
                }
                idx -= 1;
            };
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
