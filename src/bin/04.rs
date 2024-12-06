advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut i = 0;
    let input = input.trim();
    loop {
        let str = format!("{input}{i}");
        let hash = md5::compute(str);
        let hash = format!("{hash:?}");
        if hash.starts_with("00000") {
            break;
        }
        i += 1;
    }
    Some(i)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut i = 0;
    let input = input.trim();
    loop {
        let str = format!("{input}{i}");
        let hash = md5::compute(str);
        let hash = format!("{hash:?}");
        if hash.starts_with("000000") {
            break;
        }
        i += 1;
    }
    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(346386));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9958218));
    }
}
