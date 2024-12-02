fn main() {
    let input = parse(include_str!("../input"));

    println!("Day two");
    println!("Part one: {}", &part_one(&input));
    println!("Part two: {}", &part_two(&input));
}

fn part_one(input: &[Vec<isize>]) -> usize {
    input
        .iter()
        .filter(|s| is_valid(s))
        .count()
}

fn part_two(input: &[Vec<isize>]) -> usize {
    input
        .iter()
        .filter(|s| {
            let mut permutations = vec![(*s).clone()];
            
            for i in 0..s.len() {
                let mut next = (*s).clone();
                next.remove(i);
                
                permutations.push(next);
            }
            
            permutations.iter().any(|v| is_valid(v))
        })
        .count()
}

fn is_valid(s: &[isize]) -> bool {
    let diffs: Vec<isize> = s.windows(2).map(|a| a[0] - a[1]).collect();

    let is_ascending = diffs.iter().filter(|v| **v > 0).count() == diffs.len();
    let is_descending = diffs.iter().filter(|v| **v < 0).count() == diffs.len();

    let differs_not_too_much = diffs
        .iter()
        .filter(|v| v.abs() >= 1 && v.abs() <= 3)
        .count()
        == diffs.len();

    (is_ascending || is_descending) && differs_not_too_much
}

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<isize>().unwrap())
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
