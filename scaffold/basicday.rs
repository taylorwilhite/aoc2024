fn main() {
    println!("day x");
    let input = include_str!("./input/input.txt");
    let part1 = part_one(input);
    println!("part 1: {}", part1);
    let part2 = part_two(input);
    println!("part 2: {}", part2);
}

fn part_one(input: &str) -> usize {
    let mut total = 0;
    return total;
}

fn part_two(input: &str) -> usize {
    let mut total = 0;
    return total;
}

#[test]
fn part_one_matches_example() {
    let input = include_str!("./input/example.txt");
    let p1 = part_one(input);
    assert_eq!(p1, 142);
}
#[test]
fn part_two_matches_example() {
    let input = include_str!("./input/example.txt");
    let p2 = part_two(input);
    assert_eq!(p2, 281);
}
