advent_of_code::solution!(5);

fn split_input(input: &str) -> (Vec<(usize, usize)>, Vec<(usize)>) {
    let mut sections = input.split("\n\n");
    let first_section = sections.next().unwrap();
    let second_section = sections.next().unwrap();

    let ranges = first_section
        .lines()
        .map(|line| {
            let mut nums = line.split('-').map(|num| num.parse::<usize>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let products = second_section
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    (ranges, products)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    let (ranges, products) = split_input(input);
    for product in products {
        if ranges
            .iter()
            .any(|(start, end)| product >= *start && product <= *end)
        {
            count += 1;
        }
    }
    Some(count)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(14));
        assert_eq!(result, None);
    }
}
