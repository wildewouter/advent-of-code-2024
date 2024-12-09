use itertools::{enumerate, Itertools};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    println!("Day noine");
    println!("Part one: {}", &part_one(input));
    println!("Part two: {}", &part_two(input));
}

fn part_one(input: &str) -> usize {
    let parsed = input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .enumerate()
        .fold(Vec::new(), |mut result: Vec<Option<usize>>, (index, a)| {
            match (index % 2) == 0 {
                true => {
                    (0..a).for_each(|pos| result.push(Some(index / 2)));
                }
                false => {
                    (0..a).for_each(|pos| result.push(None));
                }
            }

            result
        });

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
    let parsed = input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .enumerate()
        .map(|(index, size)| match index % 2 {
            0 => (size, Some(index / 2)),
            _ => (size, None),
        })
        .collect::<Vec<_>>();

    let mut result = parsed.clone();

    for (index_of_file_to_move, (size_to_find, maybe_file)) in parsed.iter().enumerate().rev() {
        match maybe_file {
            None => continue,
            Some(file_id) => {
                match result
                    .iter()
                    .enumerate()
                    .find(|(_, (size, file))| size >= size_to_find && file.is_none())
                {
                    None => continue,
                    Some((index, (free_space, _))) => {
                        if index_of_file_to_move <= index {
                            continue;
                        }

                        if free_space - size_to_find > 0 {
                            result.insert(index + 1, (free_space - size_to_find, None));
                        }
                        result[index] = (*size_to_find, Some(*file_id));
                        result[index_of_file_to_move+1] = (*size_to_find, None);
                    }
                }
            }
        };
    }

    let a = result
        .iter()
        .flat_map(|(size, o)| match o {
            None => (0..*size).map(|_| 0).collect::<Vec<_>>(),
            Some(id) => (0..*size).map(|_| *id).collect::<Vec<_>>(),
        })
        .inspect(|a| print!("{}", a))
        .collect::<Vec<_>>();

    a.iter()
        .enumerate()
        .map(|(i, v)| match i % 2 {
            0 => i / 2 * v,
            _ => 0,
        })
        .sum()
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
