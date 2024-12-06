use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");

    let grid = get_grid(input);

    println!("Day six");
    println!("Part one: {}", &part_one(&grid));
    println!("Part two: {}", &part_two(&grid));
}

fn part_one(grid: &HashMap<(isize, isize), char>) -> usize {
    let (visited, _) = walk(grid);

    visited.len()
}

fn part_two(grid: &HashMap<(isize, isize), char>) -> usize {
    let (mut visited, _) = walk(grid);

    let start = grid
        .iter()
        .find(|(_, &c)| c == '^')
        .map(|(&p, _)| p)
        .unwrap();
    visited.remove(&start);

    visited
        .into_par_iter()
        .filter(|&obstr_position| {
            let mut new_grid = grid.clone();
            new_grid.insert(obstr_position, '#');

            let (_, is_in_loop) = walk(&new_grid);

            is_in_loop
        })
        .count()
}

fn walk(grid: &HashMap<(isize, isize), char>) -> (HashSet<(isize, isize)>, bool) {
    let mut position = grid.iter().find(|(_, &c)| c == '^').map(|(&p, &c)| (p, c));
    let mut visited: HashSet<((isize, isize), char)> = HashSet::new();

    while let Some(((x, y), direction)) = position {
        if visited.contains(&((x, y), direction)) {
            return (
                visited.into_iter().map(|(p, _)| p).collect::<HashSet<_>>(),
                true,
            );
        }

        visited.insert(((x, y), direction));

        let next_p = get_next_coord(&direction, (&x, &y));

        position = grid.get(&next_p).map(|&c| match c {
            '#' => {
                let new_direction = get_next_direction(&direction);
                (get_next_coord(&new_direction, (&x, &y)), new_direction)
            }
            _ => (next_p, direction),
        });
    }

    (
        visited.into_iter().map(|(p, _)| p).collect::<HashSet<_>>(),
        false,
    )
}

fn get_next_coord(direction: &char, (x, y): (&isize, &isize)) -> (isize, isize) {
    match direction {
        '^' => (*x, y - 1),
        '<' => (x - 1, *y),
        '>' => (x + 1, *y),
        'v' => (*x, y + 1),
        _ => (*x, *y),
    }
}

fn get_next_direction(og: &char) -> char {
    match og {
        '^' => '>',
        '<' => '^',
        '>' => 'v',
        _ => '<',
    }
}

fn get_grid(input: &str) -> HashMap<(isize, isize), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use crate::{get_grid, part_one, part_two};

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_one() {
        assert_eq!(part_one(&get_grid(INPUT)), 41);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(&get_grid(INPUT)), 6);
    }
}
