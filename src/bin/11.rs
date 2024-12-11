use std::collections::HashMap;

advent_of_code::solution!(11);

fn blink_cached(num: u64, b: usize, map: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(r) = map.get(&(num, b)) {
        return *r;
    }
    let r = blink(num, b, map);
    map.insert((num, b), r);
    r
}

fn blink(num: u64, b: usize, map: &mut HashMap<(u64, usize), usize>) -> usize {
    if b == 0 {
        return 1;
    }

    // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    if num == 0 {
        return blink_cached(1, b - 1, map);
    }

    // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    let len = (num.ilog10() + 1) as u64;
    if len % 2 == 0 {
        let half = len / 2;
        let div = 10_u64.pow(half as u32);
        return blink_cached(num / div, b - 1, map) + blink_cached(num % div, b - 1, map);
    }
    // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
    blink_cached(num * 2024, b - 1, map)
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();
    let v: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect();

    let mut hashmap = HashMap::new();

    Some(v.iter().map(|n| blink_cached(*n, 25, &mut hashmap)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();
    let v: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect();

    let mut hashmap = HashMap::new();

    Some(v.iter().map(|n| blink_cached(*n, 75, &mut hashmap)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
