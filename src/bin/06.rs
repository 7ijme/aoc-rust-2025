use std::iter::Map;

advent_of_code::solution!(6);

// Source - https://stackoverflow.com/a
// Posted by Netwave, modified by community. See post 'Timeline' for change history
// Retrieved 2025-12-18, License - CC BY-SA 4.0

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let arithmetic = lines.pop()?.split_whitespace().collect::<Vec<&str>>();
    let nums = transpose(
        lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>(),
    );

    let mut counter: u64 = 0;

    for i in 0..arithmetic.len() {
        let result = match arithmetic[i] {
            "+" => nums[i].iter().sum(),
            "*" => nums[i].iter().product(),
            _ => 0,
        };
        counter += result;
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(3263827));
        assert_eq!(result, None);
    }
}
