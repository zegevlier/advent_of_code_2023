fn part_one(input: &str) -> u64 {
    let mut input = input.lines();
    let times = input
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap());
    let distances = input
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap());

    times
        .zip(distances)
        .map(|(max_time, distance)| {
            (0..max_time)
                .filter(|time| (max_time - time) * time > distance)
                .count() as u64
        })
        .product()
}

fn part_two(input: &str) -> u128 {
    let mut input = input.lines();
    let max_time: u128 = input
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: u128 = input
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    (0..max_time)
        .filter(|time| (max_time - time) * time > distance)
        .count() as u128
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
        assert_eq!(part_one(input), 288)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 71503)
    }
}
