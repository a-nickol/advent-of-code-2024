use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(7);

fn eval(field: &str, wires: &HashMap<&str, &str>, values: &mut HashMap<String, u16>) -> u16 {
    let re_d = Regex::new(r"^\d*$").unwrap();
    let re_f = Regex::new(r"^([a-z]*)$").unwrap();
    let re_and = Regex::new(r"^(.*) AND (.*)$").unwrap();
    let re_or = Regex::new(r"^(.*) OR (.*)$").unwrap();
    let re_lshift = Regex::new(r"^(.*) LSHIFT (\d*)$").unwrap();
    let re_rshift = Regex::new(r"^(.*) RSHIFT (\d*)$").unwrap();
    let re_not = Regex::new(r"^NOT (.*)$").unwrap();

    if let Some(_cap) = re_d.captures(field.trim()) {
        field.parse::<u16>().unwrap()
    } else if let Some(cap) = re_and.captures(field) {
        eval(cap.get(1).unwrap().as_str(), wires, values)
            & eval(cap.get(2).unwrap().as_str(), wires, values)
    } else if let Some(cap) = re_or.captures(field) {
        eval(cap.get(1).unwrap().as_str(), wires, values)
            | eval(cap.get(2).unwrap().as_str(), wires, values)
    } else if let Some(cap) = re_lshift.captures(field) {
        eval(cap.get(1).unwrap().as_str(), wires, values)
            << eval(cap.get(2).unwrap().as_str(), wires, values)
    } else if let Some(cap) = re_rshift.captures(field) {
        eval(cap.get(1).unwrap().as_str(), wires, values)
            >> eval(cap.get(2).unwrap().as_str(), wires, values)
    } else if let Some(cap) = re_not.captures(field) {
        !eval(cap.get(1).unwrap().as_str(), wires, values)
    } else if let Some(_cap) = re_f.captures(field) {
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
