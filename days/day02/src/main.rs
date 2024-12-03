fn main() {
    println!("day 2");
    let input = include_str!("./input/input.txt");
    let part1 = part_one(input);
    println!("part 1: {}", part1);
    let part2 = part_two(input);
    println!("part 2: {}", part2);
}

fn part_one(input: &str) -> usize {
    let mut total = 0;
    let reports = parse_input(input);
    for report in reports {
        total += report_is_safe(&report) as usize;
    }
    return total;
}

fn part_two(input: &str) -> usize {
    let mut total = 0;
    let reports = parse_input(input);
    for report in reports {
        if report_is_safe(&report) {
            total += 1;
            continue;
        }
        // There is almost certainly a more efficient way to do this but other methods required more
        // special case logic and this was quick and easy
        for skip in 0..report.len() {
            let dampened = report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip)
                .map(|(_, &n)| n)
                .collect();
            if report_is_safe(&dampened) {
                total += 1;
                break;
            }
        }
    }
    return total;
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn report_is_safe(report: &Vec<usize>) -> bool {
    let increasing = report[0] < report[1];
    for i in 0..(report.len() - 1) {
        match (
            report[i] < report[i + 1],
            increasing,
            report[i] == report[i + 1],
        ) {
            (_, _, true) | (true, false, _) | (false, true, _) => return false,
            (true, true, _) if report[i + 1] - report[i] > 3 => return false,
            (false, false, _) if report[i] - report[i + 1] > 3 => return false,
            _ => continue,
        }
    }
    true
}

#[test]
fn part_one_matches_example() {
    let input = include_str!("./input/example.txt");
    let p1 = part_one(input);
    assert_eq!(p1, 2);
}
#[test]
fn part_two_matches_example() {
    let input = include_str!("./input/example.txt");
    let p2 = part_two(input);
    assert_eq!(p2, 4);
}
