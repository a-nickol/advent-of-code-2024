advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let mut vowels = 0;
        let mut last_c = ' ';
        let mut double_c = false;
        for c in line.chars() {
            if "aeiou".contains(c) {
                vowels += 1;
            }
            if last_c == c {
                double_c = true;
            }
            last_c = c;
        }
        let naughty = ["ab", "cd", "pq", "xy"]
            .iter()
            .any(|str| line.contains(str));
        if vowels > 2 && double_c && !naughty {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
