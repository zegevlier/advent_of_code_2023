use std::collections::HashMap;

use rayon::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum Condition {
    Unknown,
    Broken,
    Working,
}

impl Condition {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Working,
            '#' => Self::Broken,
            '?' => Condition::Unknown,
            _ => panic!("Invalid character found in input"),
        }
    }
}

#[derive(Clone, Debug)]
struct Row {
    springs: Vec<Condition>,
    groups: Vec<u32>,
}

impl Row {
    fn from_string(input: &str) -> Self {
        let mut input = input.split_whitespace();
        Row {
            springs: input
                .next()
                .unwrap()
                .chars()
                .map(Condition::from_char)
                // .rev() // We reverse the list to make removing the "front" element easier.
                .collect(),
            groups: input
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                // .rev()
                .collect(),
        }
    }
}

fn get_number_of_arrangements(row: Row, is_reading_group: Option<bool>) -> u32 {
    let mut row = row;

    if row.groups.is_empty() && row.springs.is_empty() {
        return 1;
    }
    match row.springs.pop() {
        Some(c) => match c {
            Condition::Unknown => {
                let mut row_broken = row.clone();
                row_broken.springs.push(Condition::Broken);
                row.springs.push(Condition::Working);
                get_number_of_arrangements(row_broken, is_reading_group)
                    + get_number_of_arrangements(row, is_reading_group)
            }
            Condition::Broken => {
                match row.groups.last() {
                    None => return 0,
                    Some(v) => {
                        if v == &0 {
                            return 0;
                        }
                    }
                }
                *row.groups.last_mut().unwrap() -= 1;
                get_number_of_arrangements(row, Some(true))
            }
            Condition::Working => {
                // We must have a 0 at the start of the groups list, otherwise this path is invalid.
                if is_reading_group.is_none() {
                    get_number_of_arrangements(row, Some(false))
                } else if is_reading_group.unwrap() && row.groups.last().unwrap() == &0 {
                    row.groups.pop();
                    get_number_of_arrangements(row, Some(false))
                } else if !is_reading_group.unwrap() {
                    get_number_of_arrangements(row, Some(false))
                } else {
                    0
                }
            }
        },
        None => {
            if row.groups == vec![0] {
                1
            } else {
                0
            }
        }
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(Row::from_string)
        .map(|row| get_number_of_arrangements(row, None))
        .sum()
}

fn get_ways_to_fill_chunk(
    conditions: Vec<Condition>,
    groups: Vec<u32>,
    is_grouping: bool,
) -> Vec<Vec<u32>> {
    // dbg!(conditions, groups, is_grouping);
    let mut conditions = conditions;
    let mut groups = groups;
    let out = if conditions.is_empty() {
        if is_grouping {
            if groups.last().unwrap() == &0 {
                groups.pop();
                return vec![groups];
            } else {
                return vec![];
            }
        } else {
            return vec![groups];
        }
    } else {
        match conditions.last().unwrap() {
            Condition::Unknown => {
                if groups.is_empty() {
                    conditions.pop();
                    return get_ways_to_fill_chunk(conditions, groups, false);
                }
                if is_grouping {
                    if groups.last().unwrap() == &0 {
                        groups.pop();
                        conditions.pop();
                        get_ways_to_fill_chunk(conditions, groups, false)
                    } else {
                        conditions.pop();
                        conditions.push(Condition::Broken);
                        get_ways_to_fill_chunk(conditions, groups, false)
                    }
                } else {
                    conditions.pop();
                    let mut broken_conditions = conditions.clone();
                    broken_conditions.push(Condition::Broken);
                    let mut broken_groups =
                        get_ways_to_fill_chunk(broken_conditions, groups.clone(), false);
                    let working_groups = get_ways_to_fill_chunk(conditions, groups, false);
                    broken_groups.extend(working_groups);
                    broken_groups
                }
            }
            Condition::Broken => {
                if groups.is_empty() {
                    return vec![];
                }
                if groups.last().unwrap() == &0 {
                    return vec![];
                }
                *groups.last_mut().unwrap() -= 1;
                conditions.pop();
                get_ways_to_fill_chunk(conditions, groups, true)
            }
            Condition::Working => unreachable!(),
        }
    };

    // dbg!(out)
    out
}

fn get_number_of_arrangements_faster(row: Row) -> usize {
    let chunks: Vec<Vec<&Condition>> = row
        .springs
        .iter()
        .fold(vec![], |mut acc, c: &Condition| {
            if c == &Condition::Working || acc.is_empty() {
                acc.push(vec![])
            }
            if c != &Condition::Working {
                acc.last_mut().unwrap().push(c);
            }
            acc
        })
        .into_iter()
        .rev()
        .collect();
    // if chunks.len() == 1 {
    //     return get_number_of_arrangements(row, None) as usize;
    // }

    // dbg!(&row, &chunks);

    let mut prev_groups_left: Vec<(Vec<u32>, usize)> = vec![(row.groups, 1)];
    for chunk in chunks {
        // dbg!(&prev_groups_left);
        let mut groups_left = Vec::new();
        for group in prev_groups_left.iter() {
            if group.0.is_empty() {
                // This group is no longer valid.
                continue;
            }
            if chunk.len() as u32 == *group.0.last().unwrap() {
                // The head of the group is the same size as the current chunk, so we're good.
                let mut group = group.to_owned();
                group.0.pop();
                groups_left.push(group);
                continue;
            }
            let groups = get_ways_to_fill_chunk(
                chunk.iter().copied().copied().collect(),
                group.0.to_owned(),
                false,
            );
            for g in groups {
                groups_left.push((g, group.1));
            }
        }

        prev_groups_left.clear();
        let mut hashmap = HashMap::new();
        for group in groups_left {
            hashmap
                .entry(group.0)
                .and_modify(|e| *e += group.1)
                .or_insert(group.1);
        }
        prev_groups_left.append(&mut hashmap.into_iter().collect());
    }

    let mut combined = 1;
    for group in prev_groups_left {
        if group.0.is_empty() {
            combined *= group.1;
        }
    }

    combined
}

fn part_two(input: &str) -> usize {
    input
        .par_lines()
        .map(Row::from_string)
        .map(|row| Row {
            springs: std::iter::repeat(
                row.springs
                    .iter()
                    .chain(std::iter::once(&Condition::Unknown)),
            )
            .flatten()
            .take(5 * row.springs.len() + 4)
            .copied()
            .collect(),
            groups: std::iter::repeat(row.groups.iter())
                .take(5)
                .flatten()
                .copied()
                .collect(),
        })
        .map(get_number_of_arrangements_faster)
        // .map(|row| get_number_of_arrangements(row, None))
        .map(|c| dbg!(c))
        // .sum::<u32>() as usize
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
        assert_eq!(part_one(input), 21)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 525152)
    }
}
