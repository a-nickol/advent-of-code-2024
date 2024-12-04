advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut iter = line.split('x');

        let l = iter.next().unwrap().parse::<u32>().unwrap();
        let w = iter.next().unwrap().parse::<u32>().unwrap();
        let h = iter.next().unwrap().parse::<u32>().unwrap();

        sum += 2 * l * w + 2 * w * h + 2 * h * l;
        sum += (l * w).min(l * h).min(w * h);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut iter = line.split('x');

        let l = iter.next().unwrap().parse::<u32>().unwrap();
        let w = iter.next().unwrap().parse::<u32>().unwrap();
        let h = iter.next().unwrap().parse::<u32>().unwrap();

        let mut v = vec![l, w, h];
        v.sort();

        sum += v.remove(0) * 2 + v.remove(0) * 2;
        sum += l * w * h;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
