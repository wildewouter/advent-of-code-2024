fn main() {
    let input = parse(include_str!("../input"));

    println!("Day two");
    println!("Part one: {}", &part_one(&input));
    println!("Part two: {}", &part_two(&input));
}

fn part_one(input: &[Vec<isize>]) -> usize {
    input.iter().filter(|s| is_valid(s)).count()
}

fn part_two(input: &[Vec<isize>]) -> usize {
    input
        .iter()
        .filter(|s| {
            let report = *s;
            (0..report.len()).any(|index| {
                let (left, right) = report.split_at(index);
                let permutation = left.iter().chain(&right[1..]).copied().collect::<Vec<_>>();
                is_valid(&permutation)
            }) || is_valid(report)
        })
        .count()
}

fn is_valid(s: &[isize]) -> bool {
    let mut asc = true;
    let mut desc = true;

    s.windows(2).all(|w| {
        let diff = w[0] - w[1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        asc &= diff <= 0;
        desc &= diff >= 0;
        true
    }) && (asc || desc)
}

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|c| c.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_one, part_two};

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_one() {
        let input = parse(INPUT);

        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_two() {
        let input = parse(INPUT);

        assert_eq!(part_two(&input), 4);
    }
}
