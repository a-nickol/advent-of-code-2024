use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let mut set = HashSet::new();
    let mut position = (0, 0);
    set.insert(position);

    for c in input.chars() {
        match c {
            '^' => {
                position = (position.0, position.1 + 1);
            }
            '<' => {
                position = (position.0 - 1, position.1);
            }
            '>' => {
                position = (position.0 + 1, position.1);
            }
            'v' => {
                position = (position.0, position.1 - 1);
            }
            c => {
                eprintln!("{c}");
            }
        }
        set.insert(position);
    }
    Some(set.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut set = HashSet::new();
    let mut position_1 = (0, 0);
    let mut position_2 = (0, 0);
    set.insert(position_1);
    set.insert(position_2);

    for (i, c) in input.chars().enumerate() {
        let mut position = if i % 2 == 0 { position_1 } else { position_2 };

        match c {
            '^' => {
                position = (position.0, position.1 + 1);
            }
            '<' => {
                position = (position.0 - 1, position.1);
            }
            '>' => {
                position = (position.0 + 1, position.1);
            }
            'v' => {
                position = (position.0, position.1 - 1);
            }
            c => {
                eprintln!("{c}");
            }
        }
        set.insert(position);

        if i % 2 == 0 {
            position_1 = position;
        } else {
            position_2 = position;
        }
    }
    Some(set.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
