use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(7);

lazy_static! {
    static ref RE_D: Regex = Regex::new(r"^\d*$").unwrap();
    static ref RE_F: Regex = Regex::new(r"^([a-z]*)$").unwrap();
    static ref RE_AND: Regex = Regex::new(r"^(.*) AND (.*)$").unwrap();
    static ref RE_OR: Regex = Regex::new(r"^(.*) OR (.*)$").unwrap();
    static ref RE_LSHIFT: Regex = Regex::new(r"^(.*) LSHIFT (\d*)$").unwrap();
    static ref RE_RSHIFT: Regex = Regex::new(r"^(.*) RSHIFT (\d*)$").unwrap();
    static ref RE_NOT: Regex = Regex::new(r"^NOT (.*)$").unwrap();
}

fn eval(field: &str, wires: &HashMap<&str, &str>, values: &mut HashMap<String, u16>) -> u16 {
    if let Some(_cap) = RE_D.captures(field.trim()) {
        field.parse::<u16>().unwrap()
    } else if let Some(cap) = RE_AND.captures(field) {
        eval(cap.get(1).unwrap().as_str(), wires, values)
            & eval(cap.get(2).unwrap().as_str(), wires, values)
    } else if let Some(cap) = RE_OR.captures(field) {
        eval(cap.get(1).unwrap().as_str(), wires, values)
            | eval(cap.get(2).unwrap().as_str(), wires, values)
    } else if let Some(cap) = RE_LSHIFT.captures(field) {
        eval(cap.get(1).unwrap().as_str(), wires, values)
            << eval(cap.get(2).unwrap().as_str(), wires, values)
    } else if let Some(cap) = RE_RSHIFT.captures(field) {
        eval(cap.get(1).unwrap().as_str(), wires, values)
            >> eval(cap.get(2).unwrap().as_str(), wires, values)
    } else if let Some(cap) = RE_NOT.captures(field) {
        !eval(cap.get(1).unwrap().as_str(), wires, values)
    } else if let Some(_cap) = RE_F.captures(field) {
        if values.contains_key(field) {
            return *values.get(field).unwrap();
        }
        let v = eval(wires.get(&field).unwrap(), wires, values);
        values.insert(field.to_string(), v);
        v
    } else {
        panic!("- {field} -");
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once("->").unwrap();
        wires.insert(right.trim(), left.trim());
    }

    Some(eval(wires.get("a").unwrap(), &wires, &mut HashMap::new()))
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once("->").unwrap();
        wires.insert(right.trim(), left.trim());
    }

    let a = eval(wires.get("a").unwrap(), &wires, &mut HashMap::new());

    let a = a.to_string();
    wires.insert("b", &a);
    Some(eval(wires.get("a").unwrap(), &wires, &mut HashMap::new()))
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
        assert_eq!(result, Some(123));
    }
}
