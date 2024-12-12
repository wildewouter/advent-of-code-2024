use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let input = include_str!("../input");
    let start = Instant::now();
    println!("Day ten");
    println!("Part one: {}", &part_one(input));
    println!("Part two: {}", &part_two(input));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration.as_micros());
}

fn part_one(input: &str) -> usize {
    let mut grid: HashMap<(isize, isize), usize> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            grid.insert(
                (x as isize, y as isize),
                height.to_digit(10).unwrap() as usize,
            );
        }
    }

    let start = grid
        .iter()
        .filter_map(|(c, v)| Some(*c).filter(|_| *v == 0).map(|v| (v, v)))
        .collect::<Vec<_>>();

    walk(&start, &1, &grid)
        .into_iter()
        .collect::<HashSet<((isize, isize), (isize, isize))>>()
        .len()
}

fn part_two(input: &str) -> usize {
    let mut grid: HashMap<(isize, isize), usize> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            grid.insert(
                (x as isize, y as isize),
                height.to_digit(10).unwrap() as usize,
            );
        }
    }

    let start = grid
        .iter()
        .filter_map(|(c, v)| Some(*c).filter(|_| *v == 0))
        .collect::<Vec<_>>();

    walk_two(&start, &1, &grid)
}

fn walk(
    continue_pos: &[((isize, isize), (isize, isize))],
    next_value: &usize,
    grid: &HashMap<(isize, isize), usize>,
) -> Vec<((isize, isize), (isize, isize))> {
    if next_value == &10 {
        return continue_pos.to_vec();
    }

    walk(
        &continue_pos
            .iter()
            .flat_map(|&(start, (x, y))| {
                [
                    ((x - 1, y), grid.get(&(x - 1, y))),
                    ((x + 1, y), grid.get(&(x + 1, y))),
                    ((x, y + 1), grid.get(&(x, y + 1))),
                    ((x, y - 1), grid.get(&(x, y - 1))),
                ]
                .iter()
                .filter_map(|(coord, opt)| {
                    opt.filter(|&v| v == next_value).map(|_| (start, *coord))
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        &(next_value + 1),
        grid,
    )
}

fn walk_two(
    continue_pos: &[(isize, isize)],
    next_value: &usize,
    grid: &HashMap<(isize, isize), usize>,
) -> usize {
    if next_value == &10 {
        return continue_pos.len();
    }

    walk_two(&continue_pos
        .iter()
        .flat_map(|&(x, y)| {
            [
                ((x - 1, y), grid.get(&(x - 1, y))),
                ((x + 1, y), grid.get(&(x + 1, y))),
                ((x, y + 1), grid.get(&(x, y + 1))),
                ((x, y - 1), grid.get(&(x, y - 1))),
            ]
            .iter()
            .filter_map(|(coord, opt)| opt.filter(|&v| v == next_value).map(|_| *coord))
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>(), &(next_value + 1), grid)
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_one() {
        assert_eq!(part_one(INPUT), 36);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(INPUT), 81);
    }
}
