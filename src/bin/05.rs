use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules = true;
    let mut rule_1 = HashMap::new();

    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            rules = false;
            continue;
        }
        if rules {
            let (a, b) = line.split_once("|").unwrap();
            rule_1
                .entry(a.to_string())
                .or_insert(vec![])
                .push(b.to_string());
        } else {
            let nums = line.split(",").collect::<Vec<_>>();

            let mut nums_of_line = HashSet::new();
            nums.iter().for_each(|s| {
                if !nums_of_line.insert(s.to_string()) {
                    panic!("duplicate values?");
                }
            });

            let mut correct_order = true;
            for (i, n) in nums.iter().enumerate() {
                let rules = rule_1.get(*n);
                match rules {
                    Some(rules) => {
                        for later_num in rules.iter() {
                            if nums_of_line.contains(later_num) {
                                let pos = &nums.iter().position(|s| s == later_num);
                                if let Some(pos) = *pos {
                                    if pos < i {
                                        correct_order = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    None => (),
                }
                if !correct_order {
                    break;
                }
            }
            if correct_order {
                let middle = nums[nums.len() / 2].parse::<u32>().unwrap();
                sum += middle;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules = true;
    let mut rule_1 = HashMap::new();

    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            rules = false;
            continue;
        }
        if rules {
            let (a, b) = line.split_once("|").unwrap();
            rule_1
                .entry(a.to_string())
                .or_insert(vec![])
                .push(b.to_string());
        } else {
            let nums = line.split(",").collect::<Vec<_>>();

            let mut nums_of_line = HashSet::new();
            nums.iter().for_each(|s| {
                if !nums_of_line.insert(s.to_string()) {
                    panic!("duplicate values?");
                }
            });

            let mut correct_order = true;
            for (i, n) in nums.iter().enumerate() {
                let rules = rule_1.get(*n);
                match rules {
                    Some(rules) => {
                        for later_num in rules.iter() {
                            if nums_of_line.contains(later_num) {
                                let pos = &nums.iter().position(|s| s == later_num);
                                if let Some(pos) = *pos {
                                    if pos < i {
                                        correct_order = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    None => (),
                }
                if !correct_order {
                    break;
                }
            }
            if !correct_order {
                // TODO

                let middle = nums[nums.len() / 2].parse::<u32>().unwrap();
                sum += middle;
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
