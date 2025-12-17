advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut counter = 0;
    input.trim().split(",").for_each(|range| {
        let (lower, upper) = range
            .split_once('-')
            .map(|(l, u)| (l.parse::<u64>().unwrap(), u.parse::<u64>().unwrap()))
            .unwrap();
        for num in lower..=upper {
            let str_num = num.to_string();
            // split into two halves
            let (left, right) = str_num.split_at(str_num.len() / 2);
            if left == right {
                counter += num;
            }
        }
    });
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counter = 0;
    input.trim().split(",").for_each(|range| {
        let (lower, upper) = range
            .split_once('-')
            .map(|(l, u)| (l.parse::<u64>().unwrap(), u.parse::<u64>().unwrap()))
            .unwrap();
        for num in lower..=upper {
            let str_num = num.to_string();
            for i in 1..=(str_num.len() / 2) {
                if str_num.len() % i != 0 {
                    continue;
                }

                let chuncks = str_num.as_bytes().chunks(i);
                let chunk = &str_num[0..i].as_bytes();
                if chuncks.clone().all(|c| c == *chunk) {
                    counter += num;
                    break;
                }
            }
        }
    });
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
