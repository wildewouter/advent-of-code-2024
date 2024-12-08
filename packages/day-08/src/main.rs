use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::once;

fn main() {
    let input = include_str!("../input");

    println!("Day eight");
    println!("Part one: {}", &part_one(input));
    println!("Part two: {}", &part_two(input));
}

fn part_one(input: &str) -> usize {
    let grid = get_grid(input);

    let existing_coords = grid.values().flatten().collect::<HashSet<_>>();

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_, coords) in grid.iter().filter(|&(c, _)| *c != '.') {
        for ((ax, ay), (bx, by)) in coords.iter().tuple_combinations() {
            let x_diff = bx - ax;
            let y_diff = by - ay;

            [(ax - x_diff, ay - y_diff), (bx + x_diff, by + y_diff)]
                .into_iter()
                .for_each(|coord| {
                    if existing_coords.contains(&coord) {
                        antinodes.insert(coord);
                    }
                });
        }
    }

    antinodes.len()
}

fn part_two(input: &str) -> usize {
    let grid = get_grid(input);
    let existing_coords = grid.values().flatten().collect::<HashSet<_>>();

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_, coords) in grid.iter().filter(|&(c, _)| *c != '.') {
        for ((ax, ay), (bx, by)) in coords.iter().tuple_combinations() {
            let x_diff = bx - ax;
            let y_diff = by - ay;

            for i in 0.. {
                let right = (bx + (x_diff * i), by + (y_diff * i));

                if existing_coords.contains(&right) {
                    antinodes.insert(right);
                } else {
                    break;
                }
            }

            for i in 0.. {
                let left = (ax - (x_diff * i), ay - (y_diff * i));

                if existing_coords.contains(&left) {
                    antinodes.insert(left);
                } else {
                    break;
                }
            }
        }
    }

    antinodes.len()
}

fn get_grid(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    input.lines().enumerate().fold(
        HashMap::new(),
        |mut map: HashMap<char, Vec<(isize, isize)>>, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                map.insert(
                    c,
                    map.get(&c)
                        .map(|coords| {
                            coords
                                .iter()
                                .cloned()
                                .chain(once((x as isize, y as isize)))
                                .collect()
                        })
                        .unwrap_or(vec![(x as isize, y as isize)]),
                );
            });

            map
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_one() {
        assert_eq!(part_one(INPUT), 14);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(INPUT), 34);
    }
}
