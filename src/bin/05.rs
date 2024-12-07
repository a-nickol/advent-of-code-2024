advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let mut vowels = 0;
        let mut last_c = ' ';
        let mut double_c = false;
        for c in line.chars() {
            if "aeiou".contains(c) {
                vowels += 1;
            }
            if last_c == c {
                double_c = true;
            }
            last_c = c;
        }
        let naughty = ["ab", "cd", "pq", "xy"]
            .iter()
            .any(|str| line.contains(str));
        if vowels > 2 && double_c && !naughty {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let mut has_window = false;
        let chars = line.chars().collect::<Vec<char>>();
        let char_slice = chars.as_slice();
        for w in char_slice.windows(3) {
            if w[0] == w[2] {
                has_window = true;
            }
        }
        let mut has_double_pairs = false;
        for (i, pair) in char_slice.windows(2).enumerate() {
            let mut rest = vec![];
            rest.append(&mut chars[..i].to_vec());
            rest.push(' ');
            rest.push(' ');
            rest.append(&mut chars[i + 2..].to_vec());
            has_double_pairs = rest.windows(2).any(|w| w == pair);
            if has_double_pairs {
                break;
            }
        }
        if has_double_pairs && has_window {
            count += 1;
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
        assert_eq!(result, Some(0));
    }
}
