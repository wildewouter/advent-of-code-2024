use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    println!("Day three");
    println!("Part one: {}", &part_one(input));
    // println!("Part two: {}", &part_two(input));
}

fn part_one(input: &str) -> usize {
    let mut grid: HashMap<(isize, isize), char> = HashMap::new();

    input.lines().enumerate().for_each(|(y, horizontal)| {
        horizontal.chars().enumerate().for_each(|(x, char)| {
            grid.insert((x as isize, y as isize), char);
        })
    });

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

fn check(
    c: &char,
    p: &(isize, isize),
    grid: &HashMap<(isize, isize), char>,
    next_p: fn(&(isize, isize)) -> (isize, isize),
) -> bool {
    let p_next = next_p(p);

    if let Some(next_c) = grid.get(&p_next).filter(|next_c| *next_c == c) {
        if *next_c == 'S' {
            return true;
        }

        if *next_c == next_char(c) {
            return check(next_c, &p_next, grid, next_p);
        }
    }

    false
}

fn next_char(search: &char) -> char {
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
    use crate::part_one;

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
        assert_eq!(part_one(INPUT), 18);
    }

    // #[test]
    // fn test_two() {
    //     assert_eq!(part_two(INPUT), 48);
    // }
}
