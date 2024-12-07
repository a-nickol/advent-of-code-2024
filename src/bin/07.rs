advent_of_code::solution!(7);

fn calculation_possible(base: u64, result: u64, rest: &[u64], allow_concat: bool) -> Option<u64> {
    if rest.is_empty() {
        return if base == result { Some(result) } else { None };
    }
    let next_value = rest[0];

    let add = calculation_possible(base + next_value, result, &rest[1..], allow_concat);
    if add.is_some() {
        return add;
    }

    let multi = calculation_possible(base * next_value, result, &rest[1..], allow_concat);
    if multi.is_some() {
        return multi;
    }

    if allow_concat {
        let concat = calculation_possible(
            format!("{base}{next_value}").parse::<u64>().unwrap(),
            result,
            &rest[1..],
            allow_concat,
        );
        if concat.is_some() {
            return concat;
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        if !line.trim().is_empty() {
            let (result, numbers) = line.split_once(':').unwrap();
            let result = result.parse::<u64>().unwrap();
            let numbers: Vec<u64> = numbers
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let r = calculation_possible(numbers[0], result, &numbers[1..], false);
            if let Some(r) = r {
                sum += r;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        if !line.trim().is_empty() {
            let (result, numbers) = line.split_once(':').unwrap();
            let result = result.parse::<u64>().unwrap();
            let numbers: Vec<u64> = numbers
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let r = calculation_possible(numbers[0], result, &numbers[1..], true);
            if let Some(r) = r {
                sum += r;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
