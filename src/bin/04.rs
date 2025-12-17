advent_of_code::solution!(4);

fn get_neighbors(twod: Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut cnt = 0;
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    // check if neighbors = @
    for (di, dj) in directions.iter() {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if ni >= 0
            && ni < twod.len() as isize
            && nj >= 0
            && nj < twod[0].len() as isize
            && twod[ni as usize][nj as usize] == '@'
        {
            cnt += 1;
        }
    }
    cnt
}

pub fn part_one(input: &str) -> Option<u64> {
    let twod: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for i in 0..twod.len() {
        for j in 0..twod[i].len() {
            if twod[i][j] == '@' {
                let neighbors = get_neighbors(twod.clone(), i, j);
                if neighbors < 4 {
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

    loop {
        let mut inner = 0;

        for i in 0..twod.len() {
            for j in 0..twod[i].len() {
                if twod[i][j] == '@' {
                    let neighbors = get_neighbors(twod.clone(), i, j);
                    if neighbors < 4 {
                        twod[i][j] = '.';
                        count += 1;
                        inner += 1;
                    }
                } else {
                    continue;
                }
            }
        }

        if inner == 0 {
            break;
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
