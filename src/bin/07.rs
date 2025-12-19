advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut positions = vec![grid[0].iter().position(|c| *c == 'S')?];

    // println!("{}", grid[0].iter().collect::<String>());

    let mut counter = 0;

    for row in grid.iter().skip(1) {
        let mut new_positions = positions.clone();
        for pos in &positions {
            if row[*pos] == '^' {
                // Remove current position from new_positions
                new_positions.retain(|&x| x != *pos);
                if *pos > 0 {
                    new_positions.push(*pos - 1);
                }
                if *pos + 1 < row.len() {
                    new_positions.push(*pos + 1);
                }

                counter += 1;
            }
        }

        /*
        Logging (very cool)
        for (idx, char) in row.iter().enumerate() {
            if positions.contains(&idx) {
                if *char == '^' {
                    print!("^");
                } else {
                    print!("|");
                }
            } else if *char == '^' {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!()
        */

        new_positions.sort();
        new_positions.dedup();
        positions = new_positions;

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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
