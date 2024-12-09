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
                    (0..a).for_each(|_| result.push(Some(index / 2)));
                }
                false => {
                    (0..a).for_each(|_| result.push(None));
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

    for (size_to_find, maybe_file) in parsed.iter().rev() {
        match maybe_file {
            None => continue,
            Some(file_id) => {
                match result
                    .iter()
                    .enumerate()
                    .find(|(_, (size, file))| size >= size_to_find && file.is_none())
                {
                    Some((index, (free_space, None))) => {
                        let (index_in_result, _) = result
                            .iter()
                            .enumerate()
                            .find(|(_, (_, opt))| match opt {
                                None => false,
                                Some(id) => id == file_id,
                            })
                            .unwrap();

                        if index_in_result <= index {
                            continue;
                        }

                        let remaining_free_space = free_space - size_to_find;

                        let &(free_space_to_add_to, _) = result.get(index_in_result - 1).unwrap();
                        let free_space_after = result.get(index_in_result + 1).map(|&(v, _)| v);

                        result[index_in_result - 1] = (
                            free_space_to_add_to + size_to_find + free_space_after.unwrap_or(0),
                            None,
                        );

                        if free_space_after.is_some() && (index + 1) != index_in_result {
                            result.remove(index_in_result + 1);
                        } else if free_space_after.is_some() && (index + 1) == index_in_result {
                            result[index_in_result+1] = (
                                size_to_find + free_space_after.unwrap_or(0),
                                None,
                            )
                        }

                        result.remove(index_in_result);

                        result[index] = (0, None);
                        result.insert(index + 1, (*size_to_find, Some(*file_id)));
                        result.insert(index + 2, (remaining_free_space, None));
                    }
                    _ => continue,
                }
            }
        };
    }

    result.iter()
        .fold((0,0),|(cursor, value): (usize, usize), (size, v)| {
            let x: usize = (cursor..cursor + size).map(|index| index * v.unwrap_or(0)).sum();
            (cursor+size, value + x)
        }).1
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

    #[test]
    fn test_three() {
        assert_eq!(part_two("1313165"), 169);
    }
}
