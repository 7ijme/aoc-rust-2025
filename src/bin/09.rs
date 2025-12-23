advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let coords = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: i64 = parts.next().unwrap().trim().parse().unwrap();
            let y: i64 = parts.next().unwrap().trim().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<(i64, i64)>>();

    // let max_x = coords.iter().map(|(x, _)| *x).max().unwrap();
    // let max_y = coords.iter().map(|(_, y)| *y).max().unwrap();

    // for x in 0..=max_x + 2 {
    //     for y in 0..=max_y + 2 {
    //         let char = if coords.contains(&(x, y)) { '#' } else { '.' };
    //         print!("{}", char);
    //     }
    //     println!();
    // }

    let mut max_area = 0;
    for cor1 in &coords {
        for cor2 in &coords {
            if cor1 != cor2 {
                let area = ((cor1.0 - cor2.0 + 1) * (cor1.1 - cor2.1 + 1)).unsigned_abs();
                if area > max_area {
                    max_area = area;
                }

                // println!("Comparing {:?} and {:?}, area: {}", cor1, cor2, area);
                // for x in 0..=max_x + 2 {
                //     for y in 0..=max_y + 2 {
                //         // O if inside area
                //         let char = if x >= cor1.0.min(cor2.0)
                //             && x <= cor1.0.max(cor2.0)
                //             && y >= cor1.1.min(cor2.1)
                //             && y <= cor1.1.max(cor2.1)
                //         {
                //             'O'
                //         } else if coords.contains(&(x, y)) {
                //             '#'
                //         } else {
                //             '.'
                //         };
                //         print!("{}", char);
                //     }
                //     println!();
                // }
            }
        }
    }

    Some(max_area)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
