advent_of_code::solution!(4);

fn get_neighbors(twod: &[Vec<char>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0
                && ni < twod.len() as isize
                && nj >= 0
                && nj < twod[0].len() as isize
                && twod[ni as usize][nj as usize] == '@'
            {
                neighbors.push((ni as usize, nj as usize));
            }
        }
    }
    neighbors
}

pub fn part_one(input: &str) -> Option<u64> {
    let twod: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for i in 0..twod.len() {
        for j in 0..twod[i].len() {
            if twod[i][j] == '@' {
                let neighbors = get_neighbors(&twod, i, j);
                if neighbors.len() < 4 {
                    count += 1;
                }
            } else {
                continue;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut twod: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let mut to_check: Vec<(usize, usize)> = vec![];

    for i in 0..twod.len() {
        for j in 0..twod[i].len() {
            if twod[i][j] == '@' {
                let neighbors = get_neighbors(&twod.clone(), i, j);
                if neighbors.len() < 4 {
                    twod[i][j] = '.';
                    count += 1;
                    to_check.extend(neighbors);
                }
            } else {
                continue;
            }
        }
    }

    loop {
        if to_check.is_empty() {
            break;
        }
        let (i, j) = to_check.pop().unwrap();
        if twod[i][j] == '@' {
            let neighbors = get_neighbors(&twod.clone(), i, j);
            if neighbors.len() < 4 {
                twod[i][j] = '.';
                count += 1;
                to_check.extend(neighbors);
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
