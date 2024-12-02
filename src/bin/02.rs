advent_of_code::solution!(2);

fn is_safe(input: &Vec<u32>) -> bool {
    let diffs: Vec<i64> = input
        .as_slice()
        .windows(2)
        .map(|a| (a[0] as i64) - (a[1] as i64))
        .collect();

    let going_up = diffs.iter().all(|a| *a > 0);
    let going_down = diffs.iter().all(|a| *a < 0);
    let above_3 = diffs.iter().any(|a| a.abs() > 3);

    !above_3 && (going_up || going_down)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        if is_safe(&nums) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        if is_safe(&nums) {
            count += 1;
            continue;
        }

        let mut has_exception = false;
        for i in 0..nums.len() {
            if has_exception {
                break;
            }

            let mut v = nums.clone();
            v.remove(i);

            if is_safe(&v) {
                count += 1;
                has_exception = true;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_is_safe() {
        let result = is_safe(&vec![1, 2, 4, 5]);
        assert_eq!(result, true);
    }
}
