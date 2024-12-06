use std::collections::HashSet;

advent_of_code::solution!(6);

fn find_position(array: &[Vec<char>]) -> Option<(i32, i32)> {
    for (i, l) in array.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if ['^', '<', '>', 'v'].contains(c) {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut array = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = array.len() as i32;
    let cols = array[0].len() as i32;

    let mut position = find_position(&array).unwrap();

    let mut finished = false;
    loop {
        let c = array[position.0 as usize][position.1 as usize];
        let (direction, next_dir) = match c {
            '^' => ((-1_i32, 0_i32), '>'),
            '<' => ((0, -1), '^'),
            '>' => ((0, 1), 'v'),
            'v' => ((1, 0), '<'),
            _ => panic!("unknown direction"),
        };

        loop {
            array[position.0 as usize][position.1 as usize] = 'X';

            let next_position = (position.0 + direction.0, position.1 + direction.1);
            if next_position.0 < 0
                || next_position.0 == cols
                || next_position.1 < 0
                || next_position.1 == rows
            {
                finished = true;
                break;
            }
            let next_c = array[next_position.0 as usize][next_position.1 as usize];
            if next_c == '#' {
                array[position.0 as usize][position.1 as usize] = next_dir;
                break;
            }
            position = next_position;
        }
        if finished {
            break;
        }
    }

    let sum = array.iter().fold(0, |a, v| {
        a + v.iter().fold(0, |a, c| if *c == 'X' { a + 1 } else { a })
    });
    Some(sum)
}

fn search_for_loop(array: &[Vec<char>]) -> bool {
    let mut array = array.to_vec();
    let mut positions = HashSet::new();

    let rows = array.len() as i32;
    let cols = array[0].len() as i32;

    let mut position = find_position(&array).unwrap();

    let mut finished = false;
    loop {
        let c = array[position.0 as usize][position.1 as usize];
        let (direction, next_dir) = match c {
            '^' => ((-1_i32, 0_i32), '>'),
            '<' => ((0, -1), '^'),
            '>' => ((0, 1), 'v'),
            'v' => ((1, 0), '<'),
            _ => panic!("unknown direction"),
        };

        loop {
            array[position.0 as usize][position.1 as usize] = 'X';

            if !positions.insert((position, direction)) {
                return true;
            }

            let next_position = (position.0 + direction.0, position.1 + direction.1);
            if next_position.0 < 0
                || next_position.0 == cols
                || next_position.1 < 0
                || next_position.1 == rows
            {
                finished = true;
                break;
            }
            let next_c = array[next_position.0 as usize][next_position.1 as usize];
            if next_c == '#' {
                array[position.0 as usize][position.1 as usize] = next_dir;
                break;
            }
            position = next_position;
        }
        if finished {
            break;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let array = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, line) in array.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '.' {
                let mut a = array.clone();
                a[i][j] = '#';
                if search_for_loop(&a) {
                    sum += 1;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
