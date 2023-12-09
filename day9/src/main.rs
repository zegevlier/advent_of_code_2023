fn estimate_next(input: Vec<i32>) -> i32 {
    let mut tree = vec![input];

    create_tree(&mut tree);

    tree.iter_mut().for_each(|v| v.push(0));

    for idx in (0..tree.len() - 1).rev() {
        let len = tree[idx].len();
        let new_value = tree[idx + 1][len - 2] + tree[idx][len - 2];
        tree[idx][len - 1] = new_value;
    }

    *tree.first().unwrap().iter().last().unwrap()
}

fn create_tree(tree: &mut Vec<Vec<i32>>) {
    let mut idx = 0;
    loop {
        tree.push(Vec::new());
        for i in 0..(tree[idx].len() - 1) {
            let new_value = tree[idx][i + 1] - tree[idx][i];
            tree[idx + 1].push(new_value)
        }

        idx += 1;

        if tree[idx].iter().all(|v| *v == 0) {
            break;
        }
    }
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .map(estimate_next)
        .sum()
}

fn estimate_prev(input: Vec<i32>) -> i32 {
    let mut tree = vec![input];

    create_tree(&mut tree);

    tree.iter_mut().for_each(|v| v.insert(0, 0));

    for idx in (0..tree.len() - 1).rev() {
        let new_value = tree[idx][1] - tree[idx + 1][0];
        tree[idx][0] = new_value;
    }

    *tree.first().unwrap().first().unwrap()
}

fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .map(estimate_prev)
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
        assert_eq!(part_one(input), 114)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 2)
    }
}
