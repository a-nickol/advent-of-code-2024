use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
            list1.push(a.parse::<u32>().unwrap());
            list2.push(b.parse::<u32>().unwrap());
        }
    }
    list1.sort();
    list2.sort();

    let mut diff = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        diff += a.abs_diff(*b);
    }

    Some(diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1 = vec![];
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
            list1.push(a.parse::<u32>().unwrap());
            map.entry(b.parse::<u32>().unwrap())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }
    list1.sort();

    let mut sim = 0;
    for a in list1.iter() {
        let count = *map.get(a).unwrap_or(&0);
        sim += *a * count;
    }

    Some(sim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
