advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = vec![];
    for (i, c) in input.trim().chars().enumerate() {
        let occupied = i % 2 == 0;
        let id = i / 2;
        let num = c.to_string().parse::<u64>().unwrap();
        for _ in 0..num {
            if occupied {
                disk.push(Some(id));
            } else {
                disk.push(None)
            }
        }
    }

    let mut i = 0;
    let mut j = disk.len() - 1;

    loop {
        loop {
            if i == disk.len() - 1 || disk[i].is_none() {
                break;
            }
            i += 1;
        }

        loop {
            if j == 0 || disk[j].is_some() {
                break;
            }
            j -= 1;
        }

        if i < j && disk[i].is_none() && disk[j].is_some() {
            disk[i] = disk[j];
            disk[j] = None;
        } else {
            break;
        }
    }

    let mut sum = 0_u64;
    for (i, v) in disk.iter().enumerate() {
        if let Some(v) = v {
            sum += (v * i) as u64;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
