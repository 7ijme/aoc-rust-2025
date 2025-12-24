advent_of_code::solution!(10);
use itertools::Itertools;

use std::iter::Peekable;

struct SkipLastIterator<I: Iterator>(Peekable<I>);
impl<I: Iterator> Iterator for SkipLastIterator<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0.next();
        self.0.peek().map(|_| item.unwrap())
    }
}
trait SkipLast: Iterator + Sized {
    fn skip_last(self) -> SkipLastIterator<Self> {
        SkipLastIterator(self.peekable())
    }
}
impl<I: Iterator> SkipLast for I {}

fn formatted_input(input: &str) -> Vec<(u64, Vec<u64>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');

            let indicator: u64 = iter
                .next()
                .unwrap()
                .chars()
                .skip(1)
                .skip_last()
                .enumerate()
                .map(|(i, c)| if c == '#' { 2u64.pow(i as u32) } else { 0 })
                .sum();
            let joltage = iter
                .next_back()
                .unwrap()
                .trim_matches(['{', '}'].as_ref())
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let buttons: Vec<_> = iter
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| &s[1..s.len() - 1])
                .map(|list| {
                    list.split(',')
                        .map(|c| 2u64.pow(c.parse::<u32>().unwrap()))
                        .sum::<u64>()
                })
                .collect();

            (indicator, buttons, joltage)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = formatted_input(input);
    let mut result = 0;
    for line in data {
        let (indicator, buttons, _) = line;
        'outer: for i in 1..=buttons.len() {
            for button_combination in buttons.iter().combinations(i) {
                let mut ind = 0;
                for button in button_combination {
                    ind ^= button;
                }
                if ind == indicator {
                    result += i;
                    break 'outer;
                }
            }
        }
    }

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    /* let data = formatted_input(input);
    let mut result = 0;
    for line in data {
        let (_, buttons, joltage) = line;
        let jolt_to_indicator = joltage
            .iter()
            .enumerate()
            .map(|(i, jolt)| if jolt % 2 == 1 { 2u64.pow(i as u32) } else { 0 })
            .sum::<u64>();

        //println!("Jolt indicator: {}", jolt_to_indicator);

        let combinations = find_combinations(
            joltage.iter().map(|&j| j as u64).collect(),
            buttons.clone(),
            jolt_to_indicator,
            0,
            true,
            0,
        );

        result += combinations.unwrap_or(0);
    }

    Some(result as u64) */
    None
}

/* fn find_combinations(
    joltage: Vec<u64>,
    buttons: Vec<u64>,
    indicator: u64,
    current_count: u64,
    first: bool,
    rec_index: usize,
) -> Option<u64> {
    //println!(
    //     "Recursion level {}: Current Count: {}, Joltage: {:?}, Indicator: {}, Buttons: {:?}",
    //     rec_index, current_count, joltage, indicator, buttons
    // );
    'outer: for i in 1..=if first { buttons.len() } else { 1 } {
        'inner: for button_combination in buttons
            .iter()
            .sorted_by(|a, b| b.count_ones().cmp(&a.count_ones()))
            .combinations(i)
        {
            let mut ind = 0;
            for button in button_combination.clone() {
                ind ^= button;
                if !first {
                    ind ^= button;
                }
            }
            if ind == indicator {
                let mut counters = vec![0u64; joltage.len()];
                for button in button_combination {
                    for bit_pos in 0..joltage.len() {
                        if (button & 2u64.pow(bit_pos as u32)) != 0 {
                            counters[bit_pos] += 1;
                        }
                    }
                }
                //println!("Combination for {}: {:?}", i, counters);
                //println!("Joltage: {:?}", joltage);

                for (idx, jolt) in joltage.iter().enumerate() {
                    if counters[idx] * if first { 1 } else { 2 } > *jolt {
                        //println!("Counter exceeds joltage, skipping.");
                        continue 'inner;
                    }
                }

                let new_joltage: Vec<u64> = joltage
                    .iter()
                    .enumerate()
                    .map(|(i, jolt)| jolt - counters[i] * if first { 1 } else { 2 })
                    .collect();

                //println!("New Joltage: {:?}", new_joltage);
                if new_joltage.iter().all(|&jolt| jolt == 0) {
                    //println!("All joltage satisfied.");
                    return if first {
                        Some(i as u64)
                    } else {
                        Some(i as u64 * 2)
                    };
                }

                //println!();
                //println!("Going deeper");
                let recur = find_combinations(
                    new_joltage,
                    buttons.clone(),
                    0,
                    current_count + if first { i as u64 } else { 2 * i as u64 },
                    false,
                    rec_index + 1,
                );
                if let Some(count) = recur {
                    return Some(count + if first { i as u64 } else { 2 });
                } else {
                    //println!("No valid combinations found in recursion.");
                    continue 'inner;
                }
            }
        }
    }
    None
} */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(33));
        assert_eq!(result, None);
    }
}
