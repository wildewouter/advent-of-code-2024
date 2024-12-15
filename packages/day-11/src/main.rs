use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../input");
    let start = Instant::now();
    println!("Day eleventy");

    let stones: Vec<usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    println!("Part one: {}", &part_one(&stones));
    println!("Part two: {}", &part_two(&stones));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration.as_micros());
}

fn part_one(stones: &[usize]) -> usize {
    blink(stones, &25)
}

fn part_two(stones: &[usize]) -> usize {
    blink(stones, &75)
}

fn blink(stones: &[usize], blink_number: &usize) -> usize {
    let mut stones: HashMap<usize, usize> = stones.iter().map(|stone| (*stone, 1)).collect();

    for _ in 0..*blink_number {
        let mut updated_stones = HashMap::new();
        for (stone, amount) in stones.iter() {
            if amount == &0 {
                continue;
            }

            let new_stones = flip(stone);

            new_stones.iter().for_each(|new_stone| {
                let og = updated_stones.get(new_stone).unwrap_or(&0);
                updated_stones.insert(*new_stone, og + amount);
            });
        }

        stones = updated_stones.into_iter().filter(|&(_,v)| v != 0usize).collect::<HashMap<usize,usize>>();
    }

    stones.values().sum()
}

fn flip(stone: &usize) -> Vec<usize> {
    if stone == &0 {
        return vec![1usize];
    }

    let count = digit_count(stone);

    if count % 2 == 0 {
        let divisor = 10usize.pow((count / 2) as u32);
        let left = stone / divisor;
        let right = stone % divisor;
        return vec![left, right];
    }

    vec![stone * 2024usize]
}

fn digit_count(n: &usize) -> usize {
    if n == &0 {
        1
    } else {
        (*n as f64).log10().floor() as usize + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    const INPUT: &str = r#"125 17"#;

    #[test]
    fn test_one() {
        let stones: Vec<usize> = INPUT
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        assert_eq!(part_one(&stones), 55312);
    }
}
