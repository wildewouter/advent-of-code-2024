use rayon::prelude::*;

fn main() {
    let input = include_str!("../input");

    println!("Day se7en");
    println!("Part one: {}", &part_one(input));
    // println!("Part two: {}", &part_two(input));
}

fn part_one(input: &str) -> isize {
    input
        .par_lines()
        .filter_map(|l| {
            let split = l.split(": ").collect::<Vec<&str>>();

            let answer = split[0].parse::<isize>().unwrap();
            let calc_input = split[1]
                .split(" ")
                .filter_map(|v| v.parse::<isize>().ok())
                .collect::<Vec<isize>>();

            let length = calc_input.len();

            let limit = 2usize.pow(length as u32);
            let result = (0..limit).find(|i| {
                let permutation = format!("{:0bits$b}", i, bits = length)
                    .replace("0", "+")
                    .replace("1", "x");

                try_permutation(&calc_input, &permutation) == answer
            });

            result.map(|_| answer)
        })
        .sum()
}

fn try_permutation(calc: &[isize], permutation: &str) -> isize {
    calc.iter()
        .skip(1)
        .enumerate()
        .fold(*calc.first().unwrap(), |left, (index, right)| {
            let operator = permutation.chars().collect::<Vec<_>>();

            match operator.get(index) {
                Some('x') => left * right,
                Some('+') => left + right,
                _ => panic!("AAAAAAAAAAAAH"),
            }
        })
}

// fn part_two(_input: &str) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use crate::{part_one};

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn test_one() {
        assert_eq!(part_one(INPUT), 3749);
    }

    // #[test]
    // fn test_two() {
    //     assert_eq!(part_two(INPUT), 0);
    // }
}
