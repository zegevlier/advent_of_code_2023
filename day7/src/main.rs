mod part_one;
mod part_two;

fn main() {
    let input = include_str!("../input.txt");

    println!("Part one: {}", part_one::part_one(input));
    println!("Part two: {}", part_two::part_two(input));
}
