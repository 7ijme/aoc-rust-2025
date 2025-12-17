advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(sol(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(sol(input, 12))
}

fn sol(input: &str, max_digits: usize) -> u64 {
    let mut counter = 0;
    for line in input.lines() {
        let nums = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        let mut num = 0;
        let mut hi = 0;
        let mut nxt_index = 0;
        for n in (0..max_digits).rev() {
            for i in nxt_index..nums.len() - n {
                if hi < nums[i] {
                    hi = nums[i];
                    nxt_index = i + 1;
                }
                if hi == 9 {
                    break;
                }
            }
            num += hi * (10 as u64).pow(n as u32);
            hi = 0;
        }
        num += hi;
        counter += num;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
