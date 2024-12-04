use regex::Regex;

fn main() {
    println!("day 3");
    let input = include_str!("./input/input.txt");
    let part1 = part_one(input);
    println!("part 1: {}", part1);
    let part2 = part_two(input);
    println!("part 2: {}", part2);
}

fn part_one(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let (_, [l, r]) = caps.extract();
            l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let re = Regex::new(
        r"(?:mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)|(?<yes>do\(\))|(?<no>don't\(\)))",
    )
    .unwrap();
    let mut enabled = true;
    let mut total = 0;
    for cap in re.captures_iter(input) {
        if cap.name("yes").is_some() {
            enabled = true;
            continue;
        }
        if cap.name("no").is_some() {
            enabled = false;
            continue;
        }
        if enabled {
            let (l, r) = (&cap["left"], &cap["right"]);
            total += l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap();
        }
    }
    return total;
}

#[test]
fn part_one_matches_example() {
    let input = include_str!("./input/example.txt");
    let p1 = part_one(input);
    assert_eq!(p1, 161);
}
#[test]
fn part_two_matches_example() {
    let input = include_str!("./input/example2.txt");
    let p2 = part_two(input);
    assert_eq!(p2, 48);
}
