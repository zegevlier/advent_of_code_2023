use std::collections::HashMap;

fn hash(input: &str) -> u32 {
    let mut current = 0;
    for c in input.chars() {
        current += c as u32;
        current *= 17;
        current %= 256;
    }
    current
}

fn part_one(input: &str) -> u32 {
    input.replace(['\n', '\r'], "").split(',').map(hash).sum()
}

fn part_two(input: &str) -> u32 {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
    for operation in input.replace(['\n', '\r'], "").split(',') {
        if operation.chars().nth(operation.len() - 1).unwrap() == '-' {
            let label = operation
                .chars()
                .take(operation.len() - 1)
                .collect::<String>();
            let hash = hash(&label);
            boxes
                .get_mut(hash as usize)
                .unwrap()
                .retain(|c| c.0 != label);
        } else {
            let label = operation
                .chars()
                .take(operation.len() - 2)
                .collect::<String>();
            let hash = hash(&label);
            let current_box = boxes.get_mut(hash as usize).unwrap();
            let focal_length = operation.chars().last().unwrap().to_digit(10).unwrap();
            if !current_box.iter_mut().any(|c| {
                if c.0 == label {
                    c.1 = focal_length;
                    true
                } else {
                    false
                }
            }) {
                current_box.push((label, focal_length));
            }
        }
    }

    let mut total = 0;

    for (box_num, current_box) in boxes.iter().enumerate() {
        for (slot, (_, focal_length)) in current_box.iter().enumerate() {
            total += (box_num as u32 + 1) * (slot as u32 + 1) * focal_length;
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
        assert_eq!(part_one(input), 1320)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 145)
    }
}
