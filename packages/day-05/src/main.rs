use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");

    println!("Day five");
    println!("Part one: {}", &part_one(input));
    println!("Part two: {}", &part_two(input));
}

fn part_one(input: &str) -> usize {
    let (da_cached_rules_indexed, manuals) = parse_input(input);

    manuals
        .iter()
        .filter(|pages| is_in_order(&da_cached_rules_indexed, pages))
        .map(|pages| *pages.get(pages.len() / 2).unwrap())
        .sum()
}
fn part_two(input: &str) -> usize {
    let (da_cached_rules_indexed, manuals) = parse_input(input);

    manuals
        .iter()
        .filter(|pages| !is_in_order(&da_cached_rules_indexed, pages))
        .map(|pages| {
            let mut ordered_pages = pages.clone();
            
            ordered_pages.sort_by(|a,b|  {
                let (before, _) = da_cached_rules_indexed.get(a).unwrap();
                
                if before.contains(b) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            
            ordered_pages
        })
        .map(|pages| *pages.get(pages.len() / 2).unwrap())
        .sum()
}

fn is_in_order(da_cached_rules_indexed: &HashMap<usize, (HashSet<usize>, HashSet<usize>)>, pages: &&Vec<usize>) -> bool {
    pages.iter().enumerate().all(|(index_of_page, page)| {
        let (before_page_rules, after_page_rules) =
            da_cached_rules_indexed.get(page).unwrap();

        (0..index_of_page).all(|lower_than_current_page_index| {
            let page_to_check = pages.get(lower_than_current_page_index).unwrap();
            before_page_rules.contains(page_to_check)
                && !after_page_rules.contains(page_to_check)
        }) && ((index_of_page + 1)..pages.len()).all(
            |higher_than_current_page_index| {
                let page_to_check = pages.get(higher_than_current_page_index).unwrap();
                !before_page_rules.contains(page_to_check)
                    && after_page_rules.contains(page_to_check)
            },
        )
    })
}

fn parse_input(input: &str) -> (HashMap<usize, (HashSet<usize>, HashSet<usize>)>, Vec<Vec<usize>>) {
    let split_input = input.split("\n\n").collect::<Vec<&str>>();

    let da_rules = split_input[0]
        .lines()
        .map(|line| {
            let left_and_right = line.split("|").collect::<Vec<&str>>();
            (
                left_and_right[0].parse::<usize>().unwrap(),
                left_and_right[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut da_cached_rules_indexed: HashMap<usize, (HashSet<usize>, HashSet<usize>)> =
        HashMap::new();

    for (left, right) in da_rules {
        let (before, mut after) = da_cached_rules_indexed
            .get(&left)
            .unwrap_or(&(HashSet::new(), HashSet::new()))
            .clone();

        after.insert(right);

        da_cached_rules_indexed.insert(left, (before, after));

        let (mut before, after) = da_cached_rules_indexed
            .get(&right)
            .unwrap_or(&(HashSet::new(), HashSet::new()))
            .clone();

        before.insert(left);

        da_cached_rules_indexed.insert(right, (before, after));
    }

    let manuals = split_input[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<_>>>();

    (da_cached_rules_indexed, manuals)
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_one() {
        assert_eq!(part_one(INPUT), 143);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(INPUT), 123);
    }
}
