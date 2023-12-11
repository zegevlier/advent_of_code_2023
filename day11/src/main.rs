use itertools::Itertools;

fn get_distances(input: &str, expansion_factor: usize) -> u128 {
    let mut galaxy_coords = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter_map(move |(j, c)| if c == '#' { Some((i, j)) } else { None })
        })
        .collect::<Vec<_>>();

    let x_size = input.lines().next().unwrap().len();
    let y_size = input.lines().count();

    let mut offset = 0;
    for x in 0..x_size {
        if !galaxy_coords.iter().any(|(_, j)| (x + offset) == *j) {
            galaxy_coords
                .iter_mut()
                .filter(|(_, j)| *j > (x + offset))
                .for_each(|g| g.1 += expansion_factor);
            offset += expansion_factor;
        }
    }

    let mut offset = 0;
    for y in 0..y_size {
        if !galaxy_coords.iter().any(|(i, _)| (y + offset) == *i) {
            galaxy_coords
                .iter_mut()
                .filter(|(i, _)| *i > (y + offset))
                .for_each(|g| g.0 += expansion_factor);
            offset += expansion_factor;
        }
    }

    galaxy_coords
        .iter()
        .combinations(2)
        .map(|c| {
            let a = c[0];
            let b = c[1];
            ((a.0 as i128 - b.0 as i128).abs() + (a.1 as i128 - b.1 as i128).abs()) as u128
        })
        .sum()
}

fn part_one(input: &str) -> u32 {
    get_distances(input, 1) as u32
}

fn part_two(input: &str) -> u128 {
    get_distances(input, 1_000_000 - 1)
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
        assert_eq!(part_one(input), 374)
    }

    #[test]
    fn test_part_two_1() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(get_distances(input, 10 - 1), 1030)
    }

    #[test]
    fn test_part_two_2() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(get_distances(input, 100 - 1), 8410)
    }
}
