advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut count = 0_i32;
    for c in input.chars() {
        if c == '(' {
            count += 1;
        }
        if c == ')' {
            count -= 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0_i32;
    let mut i = 1;
    for c in input.chars() {
        if c == '(' {
            count += 1;
        }
        if c == ')' {
            count -= 1;
        }
        if count == -1 {
            return Some(i);
        }
        i += 1;
    }
    None
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
        assert_eq!(result, None);
    }
}
