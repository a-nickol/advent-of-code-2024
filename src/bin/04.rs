advent_of_code::solution!(4);

fn xmas_count(i: usize, j: usize, chars: &Vec<Vec<char>>) -> u32 {
    let mut c = 0;
    let rows = chars.len();
    let cols = chars[0].len();

    if chars[i][j] == 'X' {
        if i + 3 < rows
            && chars[i + 1][j] == 'M'
            && chars[i + 2][j] == 'A'
            && chars[i + 3][j] == 'S'
        {
            c += 1;
        }
        if i + 3 < rows
            && j + 3 < cols
            && chars[i + 1][j + 1] == 'M'
            && chars[i + 2][j + 2] == 'A'
            && chars[i + 3][j + 3] == 'S'
        {
            c += 1;
        }
        if i + 3 < rows
            && j > 2
            && chars[i + 1][j - 1] == 'M'
            && chars[i + 2][j - 2] == 'A'
            && chars[i + 3][j - 3] == 'S'
        {
            c += 1;
        }
        if i > 2 && chars[i - 1][j] == 'M' && chars[i - 2][j] == 'A' && chars[i - 3][j] == 'S' {
            c += 1;
        }
        if i > 2
            && j + 3 < cols
            && chars[i - 1][j + 1] == 'M'
            && chars[i - 2][j + 2] == 'A'
            && chars[i - 3][j + 3] == 'S'
        {
            c += 1;
        }
        if i > 2
            && j > 2
            && chars[i - 1][j - 1] == 'M'
            && chars[i - 2][j - 2] == 'A'
            && chars[i - 3][j - 3] == 'S'
        {
            c += 1;
        }
        if j + 3 < cols
            && chars[i][j + 1] == 'M'
            && chars[i][j + 2] == 'A'
            && chars[i][j + 3] == 'S'
        {
            c += 1;
        }
        if j > 2 && chars[i][j - 1] == 'M' && chars[i][j - 2] == 'A' && chars[i][j - 3] == 'S' {
            c += 1;
        }
    }
    c
}

fn mas_count(i: usize, j: usize, chars: &Vec<Vec<char>>) -> u32 {
    let mut c = 0;
    let rows = chars.len();
    let cols = chars[0].len();

    if i > 0 && i + 1 < rows && j > 0 && j + 1 < cols {
        if chars[i][j] == 'A' {
            if chars[i + 1][j + 1] == 'M'
                && chars[i + 1][j - 1] == 'M'
                && chars[i - 1][j + 1] == 'S'
                && chars[i - 1][j - 1] == 'S'
            {
                c += 1;
            }
            if chars[i - 1][j + 1] == 'M'
                && chars[i - 1][j - 1] == 'M'
                && chars[i + 1][j + 1] == 'S'
                && chars[i + 1][j - 1] == 'S'
            {
                c += 1;
            }
            if chars[i + 1][j - 1] == 'M'
                && chars[i - 1][j - 1] == 'M'
                && chars[i + 1][j + 1] == 'S'
                && chars[i - 1][j + 1] == 'S'
            {
                c += 1;
            }
            if chars[i + 1][j + 1] == 'M'
                && chars[i - 1][j + 1] == 'M'
                && chars[i + 1][j - 1] == 'S'
                && chars[i - 1][j - 1] == 'S'
            {
                c += 1;
            }
        }
    }
    c
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut chars = vec![];
    for line in input.lines() {
        let mut l = vec![];
        for c in line.chars() {
            l.push(c);
        }
        if l.len() > 0 {
            chars.push(l);
        }
    }

    let mut res = 0_u32;
    for i in 0..chars.len() {
        let l = &chars[i];
        for j in 0..l.len() {
            res += xmas_count(i, j, &chars);
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut chars = vec![];
    for line in input.lines() {
        let mut l = vec![];
        for c in line.chars() {
            l.push(c);
        }
        if l.len() > 0 {
            chars.push(l);
        }
    }

    let mut res = 0_u32;
    for i in 0..chars.len() {
        let l = &chars[i];
        for j in 0..l.len() {
            res += mas_count(i, j, &chars);
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
