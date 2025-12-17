use std::ops::Rem;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut counter = 50;
    let mut passed = 0;
    input.lines().for_each(|tick| {
        // Split the string into first char and the rest
        let (first, rest) = tick.split_at(1);
        let mut num = rest.parse::<i64>().unwrap();
        if first == "L" {
            num *= -1;
        }

        counter = (counter + num).rem_euclid(100);
        if counter == 0 {
            passed += 1;
        }
    });
    Some(passed)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counter: i64 = 50;
    let mut passed = 0;
    input.lines().for_each(|tick| {
        // Split the string into first char and the rest
        let (first, rest) = tick.split_at(1);
        let num = rest.parse::<i64>().unwrap();
        let dir: i64 = match first {
            "L" => -1,
            "R" => 1,
            _ => panic!("Invalid direction"),
        };

        let previous = counter;
        counter = (counter + num * dir).rem_euclid(100);

        if dir == 1 && previous != 0 {
            if num.rem(100) + previous >= 100 {
                // Passed clockwise
                passed += 1;
            }
        } else if dir == -1 && previous != 0 && previous <= num.rem(100) {
                // Passed counter-clockwise
                passed += 1;
            }

        // Multiples full laps
        passed += num / 100;
    });
    Some(passed as u64)
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
        assert_eq!(result, Some(6));
    }
}
