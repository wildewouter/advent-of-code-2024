use regex::Regex;

fn main() {
    let input = include_str!("../input");

    println!("Day three");
    println!("Part one: {}", &part_one(input));
    println!("Part two: {}", &part_two(input));
}

fn part_one(input: &str) -> isize {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    r.captures_iter(input)
        .map(|cap| cap[1].parse::<isize>().unwrap() * cap[2].parse::<isize>().unwrap())
        .sum()
}

fn part_two(input: &str) -> isize {
    let r = Regex::new(r"(do(n't)?\(\))|(mul\((\d+),(\d+)\))").unwrap();

    let mut include = true;
    let mut answer: isize = 0;

    for cap in r.captures_iter(input) {
        let include_instruction = cap.get(1).map(|m| m.as_str());

        include = match include_instruction {
            Some("don't()") => false,
            Some("do()") => true,
            _ => include,
        };

        if include_instruction.is_some() {
            continue
        }

        if !include {
            continue;
        }

        answer += cap[4].parse::<isize>().unwrap() * cap[5].parse::<isize>().unwrap()
    }

    answer
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    const INPUT2: &str = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test_one() {
        assert_eq!(part_one(INPUT), 161);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(INPUT2), 48);
    }
}
