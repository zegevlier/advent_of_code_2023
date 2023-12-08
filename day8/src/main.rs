use std::collections::HashMap;

struct Node {
    left: String,
    right: String,
}

fn part_one(input: &str) -> u32 {
    let mut input = input.lines();
    let mut directions = input.next().unwrap().chars().cycle();
    input.next();

    let mut graph = HashMap::new();
    input.for_each(|line| {
        let source = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        graph.insert(source, Node { left, right });
    });

    let mut steps = 0;
    let mut current_id = "AAA".to_string();
    loop {
        if current_id == "ZZZ" {
            break;
        }
        let current_node = graph.get(&current_id).unwrap();
        current_id = match directions.next().unwrap() {
            'L' => current_node.left.clone(),
            'R' => current_node.right.clone(),
            _ => panic!("Invalid direction in input"),
        };
        steps += 1;
    }
    steps
}

#[derive(Clone)]
struct FastNode {
    left: u16,
    right: u16,
}

enum Direction {
    Left,
    Right,
}

fn id_to_u16(input: &str) -> u16 {
    let mut out = 0;
    for i in 0..3 {
        out |= ((input.as_bytes()[i as usize] & 0b00011111) as u16) << (i * 5);
    }
    out
}

fn part_two(input: &str) -> u32 {
    let mut input = input.lines();
    let directions: Vec<_> = input
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction in input"),
        })
        .collect();
    input.next();

    let mut graph = vec![FastNode { left: 0, right: 0 }; u16::MAX as usize];

    let mut current_nodes = Vec::new();
    input.for_each(|line| {
        let source_str = &line[0..3];
        let source = id_to_u16(source_str);

        if source_str.ends_with('A') {
            current_nodes.push(source);
        }

        let left = id_to_u16(&line[7..10]);
        let right = id_to_u16(&line[12..15]);
        graph[source as usize] = FastNode { left, right };
    });

    let mut steps = 0;
    let mut combined;
    let mut direction_idx = 0;
    loop {
        let direction = &directions[direction_idx % directions.len()];
        direction_idx += 1;
        combined = u16::MAX;

        #[allow(clippy::needless_range_loop)] // It's not actually needless here.
        for i in 0..current_nodes.len() {
            combined &= current_nodes[i];
            let current_node = &graph[current_nodes[i] as usize];
            current_nodes[i] = match direction {
                Direction::Left => current_node.left,
                Direction::Right => current_node.right,
            };
        }

        if combined & 0b0111110000000000 == 0b0110100000000000 {
            break;
        }

        steps += 1;
    }
    steps
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
    fn test_part_one_single() {
        let input = include_str!("../test_files/part_one_single.txt");
        assert_eq!(part_one(input), 2)
    }

    #[test]
    fn test_part_one_looping() {
        let input = include_str!("../test_files/part_one_looping.txt");
        assert_eq!(part_one(input), 6)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_two.txt");
        assert_eq!(part_two(input), 6)
    }
}
