use regex::Regex;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<i64> {
    let button_regex = Regex::new(r"Butt.*X\+(\d*), Y\+(\d*)").unwrap();
    let price_regex = Regex::new(r"Prize.*X=(\d*), Y=(\d*)").unwrap();

    let mut button_a = None;
    let mut button_b = None;

    let mut tasks = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(cap) = button_regex.captures(line) {
            if button_a.is_some() {
                button_b = Some((
                    cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ));
            } else {
                button_a = Some((
                    cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ));
            }
        }
        if let Some(cap) = price_regex.captures(line) {
            let price = Some((
                cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            ));
            tasks.push((button_a.unwrap(), button_b.unwrap(), price.unwrap()));
            button_a = None;
            button_b = None;
        }
    }

    let mut sum = 0;
    for (a, b, p) in tasks.clone() {
        let mut matches = vec![];
        for i in 0..=(p.0 / a.0) {
            let vx = a.0 * i;
            let d = p.0 - vx;
            if d % b.0 == 0 {
                let j = d / b.0;
                if p == (a.0 * i + b.0 * j, a.1 * i + b.1 * j) {
                    matches.push(((i, j), i * 3 + j));
                }
            }
        }
        matches.sort_by_key(|m| m.1);
        if !matches.is_empty() {
            let cost = matches.first().unwrap().1;
            sum += cost;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let button_regex = Regex::new(r"Butt.*X\+(\d*), Y\+(\d*)").unwrap();
    let price_regex = Regex::new(r"Prize.*X=(\d*), Y=(\d*)").unwrap();

    let mut button_a = None;
    let mut button_b = None;

    let mut tasks = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(cap) = button_regex.captures(line) {
            if button_a.is_some() {
                button_b = Some((
                    cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ));
            } else {
                button_a = Some((
                    cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ));
            }
        }
        if let Some(cap) = price_regex.captures(line) {
            let price = Some((
                cap.get(1).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000,
                cap.get(2).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000,
            ));
            tasks.push((button_a.unwrap(), button_b.unwrap(), price.unwrap()));
            button_a = None;
            button_b = None;
        }
    }

    let mut sum = 0;
    for (a, b, p) in tasks.clone() {
        let mut matches = vec![];
        for i in 0..=(p.0 / a.0) {
            let vx = a.0 * i;
            let d = p.0 - vx;
            if d % b.0 == 0 {
                let j = d / b.0;
                if p == (a.0 * i + b.0 * j, a.1 * i + b.1 * j) {
                    matches.push(((i, j), i * 3 + j));
                }
            }
        }
        matches.sort_by_key(|m| m.1);
        if !matches.is_empty() {
            let cost = matches.first().unwrap().1;
            sum += cost;
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
