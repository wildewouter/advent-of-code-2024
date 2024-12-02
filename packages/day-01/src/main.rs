use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let separated = separate_input(input);

    println!("Day one");
    println!("Part one: {}", part_one(&separated));
    println!("Part two: {}", part_two(&separated));
}

fn part_one((left, right): &(Vec<isize>, Vec<isize>)) -> isize {
    let mut mleft = left.clone();
    let mut mright = right.clone();

    mleft.sort();
    mright.sort();

    mleft
        .iter()
        .zip(mright.iter())
        .map(|(&l, &r)| (l - r).abs())
        .sum()
}

fn part_two((left, right): &(Vec<isize>, Vec<isize>)) -> isize {
    let occurrences: HashMap<_, _> = right
        .iter()
        .map(|s| (s, right.iter().filter(|v| *v == s).count()))
        .collect();

    left.iter()
        .map(|s| (*occurrences.get(&s).unwrap_or(&0) as isize) * s)
        .sum()
}

fn separate_input(input: &str) -> (Vec<isize>, Vec<isize>) {
    let (left, right): (Vec<_>, Vec<_>) = input
        .split_whitespace()
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    (
        left.iter()
            .map(|(_, s)| s.parse::<isize>().unwrap())
            .collect(),
        right
            .iter()
            .map(|(_, s)| s.parse::<isize>().unwrap())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two, separate_input};

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
    "#;

    #[test]
    fn test_one() {
        let input = separate_input(INPUT);
        assert_eq!(part_one(&input), 11);
    }

    #[test]
    fn test_two() {
        let input = separate_input(INPUT);
        assert_eq!(part_two(&input), 31);
    }
}
