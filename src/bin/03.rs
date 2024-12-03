use regex::Regex;

advent_of_code::solution!(3);

fn parse_mul(input: &str) -> u64 {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
    let mut res = 0_u64;
    re.captures_iter(input).for_each(|c| {
        let a = c.name("a").unwrap().as_str().parse::<u32>().unwrap();
        let b = c.name("b").unwrap().as_str().parse::<u32>().unwrap();
        res += (a * b) as u64;
    });
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut res = 0;
    res += parse_mul(input);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut res = 0;
    let line = input.replace("\n", "");
    let re = Regex::new(r"(?<input>.*?)(?<divider>don't\(\)|do\(\)|$)").unwrap();
    let mut d = String::new();
    re.captures_iter(&line).for_each(|c| {
        let i = c.name("input").unwrap().as_str();
        let divider = c.name("divider").unwrap().as_str();
        if d.is_empty() || d == "do()" {
            let m = parse_mul(i);
            res += m;
        }
        d = String::from(divider);
    });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(644));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48 * 4));
    }
}
