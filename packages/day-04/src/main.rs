use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let grid = create_grid(input);

    println!("Day three");
    println!("Part one: {}", &part_one(&grid));
    println!("Part two: {}", &part_two(&grid));
}

fn create_grid(input: &str) -> HashMap<(isize, isize), char> {
    let mut grid: HashMap<(isize, isize), char> = HashMap::new();

    input.lines().enumerate().for_each(|(y, horizontal)| {
        horizontal.chars().enumerate().for_each(|(x, char)| {
            grid.insert((x as isize, y as isize), char);
        })
    });

    grid
}

fn part_one(grid: &HashMap<(isize, isize), char>) -> usize {
    let mut total = 0;

    for (p, c) in grid.iter().filter(|(_, c)| *c == &'X') {
        total += [
            check(c, p, &grid, diag_left_up),
            check(c, p, &grid, diag_left_down),
            check(c, p, &grid, diag_right_up),
            check(c, p, &grid, diag_right_down),
            check(c, p, &grid, horizontal_left),
            check(c, p, &grid, horizontal_right),
            check(c, p, &grid, vertical_up),
            check(c, p, &grid, vertical_down),
        ]
        .into_iter()
        .filter(|a| *a)
        .count();
    }

    total
}

fn part_two(grid: &HashMap<(isize, isize), char>) -> usize {
    let get_chars =
        |p: &(isize, isize), g: &HashMap<(isize, isize), char>| *g.get(p).unwrap_or(&' ');

    grid.iter()
        .filter(|(_, c)| *c == &'A')
        .filter(|((x, y), _)| {
            [
                [
                    get_chars(&(x - 1, y + 1), grid),
                    get_chars(&(*x, *y), grid),
                    get_chars(&(x + 1, y - 1), grid),
                ],
                [
                    get_chars(&(x + 1, y + 1), grid),
                    get_chars(&(*x, *y), grid),
                    get_chars(&(x - 1, y - 1), grid),
                ],
            ]
            .into_iter()
            .filter(|word| {
                let string = word.iter().collect::<String>();

                string == "MAS" || string == "SAM"
            })
            .count()
                == 2
        })
        .count()
}

fn check(
    c: &char,
    p: &(isize, isize),
    grid: &HashMap<(isize, isize), char>,
    get_p_next: fn(&(isize, isize)) -> (isize, isize),
) -> bool {
    let p_next = get_p_next(p);

    if let Some(next_c) = grid.get(&p_next) {
        if *next_c == next_char_xmas(c) {
            if *next_c == 'S' {
                return true;
            }

            return check(next_c, &p_next, grid, get_p_next);
        }
    }

    false
}

fn next_char_xmas(search: &char) -> char {
    match search {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => 'O',
    }
}

fn diag_left_up((x, y): &(isize, isize)) -> (isize, isize) {
    (x - 1, y + 1)
}
fn diag_left_down((x, y): &(isize, isize)) -> (isize, isize) {
    (x - 1, y - 1)
}
fn diag_right_up((x, y): &(isize, isize)) -> (isize, isize) {
    (x + 1, y + 1)
}
fn diag_right_down((x, y): &(isize, isize)) -> (isize, isize) {
    (x + 1, y - 1)
}
fn horizontal_left((x, y): &(isize, isize)) -> (isize, isize) {
    (x - 1, *y)
}
fn horizontal_right((x, y): &(isize, isize)) -> (isize, isize) {
    (x + 1, *y)
}
fn vertical_up((x, y): &(isize, isize)) -> (isize, isize) {
    (*x, y + 1)
}
fn vertical_down((x, y): &(isize, isize)) -> (isize, isize) {
    (*x, y - 1)
}

#[cfg(test)]
mod tests {
    use crate::{create_grid, part_one, part_two};

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_one() {
        let grid = create_grid(INPUT);

        assert_eq!(part_one(&grid), 18);
    }

    #[test]
    fn test_two() {
        let grid = create_grid(INPUT);

        assert_eq!(part_two(&grid), 9);
    }
}
