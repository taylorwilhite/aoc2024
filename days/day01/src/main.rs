use itertools::Itertools;

fn main() {
    let input = include_str!("./input/input.txt");
    let part1 = part_one(input);
    println!("part 1: {}", part1);
    let part2 = part_two(input);
    println!("part 2: {}", part2);
}

fn part_one(input: &str) -> isize {
    let mut total = 0;
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();
    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }
    return total;
}

fn part_two(input: &str) -> usize {
    let mut total = 0;
    let (left, right) = parse_input(input);
    let freq = right.into_iter().counts();
    for n in left {
        if let Some(num) = freq.get(&n) {
            total += n as usize * num
        }
    }

    return total;
}

fn parse_input(input: &str) -> (Vec<isize>, Vec<isize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    // break input list into pairs of numbers that get parsed as ints and pushed to Vec
    // Inputs are checked when copying from site so we can safely assume some things will be correct
    for line in input.lines() {
        let nums: Vec<isize> = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        left.push(nums[0]);
        right.push(nums[1]);
    }
    (left, right)
}

#[test]
fn part_one_matches_example() {
    let input = include_str!("./input/example.txt");
    let p1 = part_one(input);
    assert_eq!(p1, 11);
}
#[test]
fn part_two_matches_example() {
    let input = include_str!("./input/example.txt");
    let p2 = part_two(input);
    assert_eq!(p2, 31);
}
