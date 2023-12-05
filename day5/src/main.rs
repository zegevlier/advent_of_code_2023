use itertools::Itertools;
use std::str::Split;

struct Mapping {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

impl Mapping {
    fn from_string(input: &str) -> Self {
        let mut input = input.split_whitespace();
        Mapping {
            dest_start: input.next().unwrap().parse().unwrap(),
            source_start: input.next().unwrap().parse().unwrap(),
            length: input.next().unwrap().parse().unwrap(),
        }
    }

    fn map_if_in_range(&self, value: u64) -> Option<u64> {
        if value >= self.source_start && value < self.source_start + self.length {
            Some(self.dest_start + (value - self.source_start))
        } else {
            None
        }
    }
}

struct Mapper {
    mappings: Vec<Mapping>,
}

impl Mapper {
    fn from_string(input: &str) -> Self {
        Mapper {
            mappings: input.lines().skip(1).map(Mapping::from_string).collect(),
        }
    }

    fn map(&self, value: u64) -> u64 {
        self.mappings
            .iter()
            .find_map(|mapping| mapping.map_if_in_range(value))
            .unwrap_or(value)
    }
}

struct Almanac {
    mappers: Vec<Mapper>,
}

impl Almanac {
    fn from_string(input: Split<&str>) -> Self {
        Almanac {
            mappers: input.map(Mapper::from_string).collect(),
        }
    }

    fn map_seed_to_location(&self, value: u64) -> u64 {
        let mut value = value;
        for mapper in self.mappers.iter() {
            value = mapper.map(value);
        }
        value
    }
}

fn part_one(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    let seeds: Vec<u64> = input
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let almanac = Almanac::from_string(input);

    seeds
        .iter()
        .map(|v| (v, almanac.map_seed_to_location(*v)))
        .fold(u64::MAX, |acc, v| if v.1 < acc { v.1 } else { acc })
}

fn part_two(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    let seeds_ranges: Vec<(u64, u64)> = input
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut v| (v.next().unwrap(), v.next().unwrap()))
        .collect();

    let almanac = Almanac::from_string(input);

    seeds_ranges
        .iter()
        .flat_map(|(a, b)| *a..(a + b))
        .map(|v| (v, almanac.map_seed_to_location(v)))
        .fold(u64::MAX, |acc, v| if v.1 < acc { v.1 } else { acc })
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
        assert_eq!(part_one(input), 35)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 46)
    }
}
