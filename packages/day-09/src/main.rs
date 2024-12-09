use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input");

    println!("Day noine");
    println!("Part one: {}", &part_one(input));
    println!("Part two: {}", &part_two(input));
}

fn part_one(input: &str) -> usize {
    let parsed = parse(input);

    let mut result = Vec::new();
    let mut cursor = 0;

    let files_only = parsed.iter().flatten().collect::<Vec<_>>();
    let files_length = files_only.len();

    for &file_bit in files_only.iter().rev() {
        if result.len() == files_length {
            break;
        }

        for og in parsed.iter().skip(cursor) {
            if result.len() == files_length {
                break;
            }
            match og {
                None => {
                    result.insert(cursor, Some(*file_bit));
                    cursor += 1;
                    break;
                }
                Some(actual) => {
                    result.insert(cursor, Some(*actual));
                    cursor += 1;
                }
            }
        }
    }

    result
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, v)| i * v)
        .sum()
}

fn part_two(input: &str) -> usize {
    let parsed = parse(input);



    let files_only = parsed.iter().flatten().collect::<Vec<_>>();
    let files_length = files_only.len();

    let mut empties: HashMap<usize, usize> = HashMap::new();
    let mut start_free = 0;
    let mut amount_free = 0;

    for (index, &file_or_free) in parsed.iter().enumerate() {
        match file_or_free {
            None => {
                if amount_free == 0 {
                    start_free = index;
                }

                amount_free += 1;
            }
            Some(_) => {
                empties.insert(start_free, amount_free);
                amount_free = 0;
            }
        }
    }


    let mut result = Vec::new();
    let mut cursor = 0;
    for (file_bit, group) in files_only.iter().rev().chunk_by(|a|a).into_iter() {
        if result.len() == files_length {
            break;
        }

        let size = group.into_iter().count();

        for og in parsed.iter().skip(cursor) {
            if result.len() == files_length {
                break;
            }
            match og {
                None => {
                    result.insert(cursor, Some(*file_bit));
                    cursor += 1;
                    break;
                }
                Some(actual) => {
                    result.insert(cursor, Some(*actual));
                    cursor += 1;
                }
            }
        }
    }

    result
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, v)| i * v)
        .sum()
}

fn parse(input: &str) -> Vec<Option<usize>> {
    input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .enumerate()
        .fold(Vec::new(), |mut result: Vec<Option<usize>>, (index, a)| {
            let left_over = index % 2;
            match left_over == 0 {
                true => {
                    (0..a).for_each(|pos| result.push(Some(index / 2 + left_over)));
                }
                false => {
                    (0..a).for_each(|pos| result.push(None));
                }
            }

            result
        })
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn test_one() {
        assert_eq!(part_one(INPUT), 1928);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(INPUT), 2858);
    }
}
