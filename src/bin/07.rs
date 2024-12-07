use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut environment: HashMap<&str, u32> = HashMap::new();

    let re_d = Regex::new(r"^\d*$").unwrap();
    let re_and = Regex::new(r"^(.*) AND (.*)$").unwrap();
    let re_or = Regex::new(r"^(.*) OR (.*)$").unwrap();
    let re_lshift = Regex::new(r"^(.*) LSHIFT (\d*)$").unwrap();
    let re_rshift = Regex::new(r"^(.*) RSHIFT (\d*)$").unwrap();
    let re_not = Regex::new(r"^NOT (.*)$").unwrap();

    for line in input.lines() {
        let (left, right) = line.split_once("->").unwrap();
        let left = left.trim();
        let right = right.trim();

        let mut value = 0;

        if let Some(cap) = re_d.captures(left.trim()) {
            value = left.parse::<u32>().unwrap();
        } else if let Some(cap) = re_and.captures(left) {
            value = environment.get(cap.get(1).unwrap().as_str()).unwrap()
                & environment.get(cap.get(2).unwrap().as_str()).unwrap();
        } else if let Some(cap) = re_or.captures(left) {
            value = environment.get(cap.get(1).unwrap().as_str()).unwrap()
                | environment.get(cap.get(2).unwrap().as_str()).unwrap();
        } else if let Some(cap) = re_lshift.captures(left) {
            value = *environment.get(cap.get(1).unwrap().as_str()).unwrap()
                << cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        } else if let Some(cap) = re_rshift.captures(left) {
            value = *environment.get(cap.get(1).unwrap().as_str()).unwrap()
                >> cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        } else if let Some(cap) = re_not.captures(left) {
            value = !*environment.get(cap.get(1).unwrap().as_str()).unwrap();
        } else {
            panic!(" - {line} - ");
        }

        environment.insert(right, value);
    }

    environment.get("a").copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
