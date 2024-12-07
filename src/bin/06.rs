use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d*,\d*) through (\d*,\d*)").unwrap();

    let mut map = HashMap::new();
    for line in input.lines() {
        if let Some(cap) = re.captures(line) {
            let mode = cap.get(1).unwrap().as_str();
            let from = cap.get(2).unwrap().as_str().split_once(',').unwrap();
            let to = cap.get(3).unwrap().as_str().split_once(',').unwrap();
            let from = (
                from.0.parse::<u32>().unwrap(),
                from.1.parse::<u32>().unwrap(),
            );
            let to = (to.0.parse::<u32>().unwrap(), to.1.parse::<u32>().unwrap());

            for i in from.0..=to.0 {
                for j in from.1..=to.1 {
                    let coord = (i, j);
                    let e = map.entry(coord).or_insert(false);
                    match mode {
                        "toggle" => {
                            *e = !*e;
                        }
                        "turn on" => *e = true,
                        "turn off" => *e = false,
                        mode => panic!("Unknown mode: {mode}"),
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if *map.get(&(i, j)).unwrap_or(&false) {
                sum += 1;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d*,\d*) through (\d*,\d*)").unwrap();

    let mut map = HashMap::new();
    for line in input.lines() {
        if let Some(cap) = re.captures(line) {
            let mode = cap.get(1).unwrap().as_str();
            let from = cap.get(2).unwrap().as_str().split_once(',').unwrap();
            let to = cap.get(3).unwrap().as_str().split_once(',').unwrap();
            let from = (
                from.0.parse::<u32>().unwrap(),
                from.1.parse::<u32>().unwrap(),
            );
            let to = (to.0.parse::<u32>().unwrap(), to.1.parse::<u32>().unwrap());

            for i in from.0..=to.0 {
                for j in from.1..=to.1 {
                    let coord = (i, j);
                    let e = map.entry(coord).or_insert(0);
                    match mode {
                        "toggle" => {
                            *e += 2;
                        }
                        "turn on" => *e += 1,
                        "turn off" => {
                            if *e > 0 {
                                *e -= 1;
                            }
                        }
                        mode => panic!("Unknown mode: {mode}"),
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            sum += *map.get(&(i, j)).unwrap_or(&0);
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
        assert_eq!(result, Some(999999));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2000001));
    }
}
