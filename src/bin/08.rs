use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashMap::new();

    let mut rows = 0_i32;
    let mut cols = 0_i32;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                let m = map.entry(c).or_insert(vec![]);
                m.push((i as i32, j as i32));
            }
            cols = (j + 1) as i32;
        }
        rows = (i + 1) as i32;
    }

    let mut antinodes = HashSet::new();
    for (_c, positions) in map.iter() {
        for p in positions {
            for q in positions {
                if p != q {
                    let diff = (p.0 - q.0, p.1 - q.1);
                    let antinode = (p.0 + diff.0, p.1 + diff.1);
                    antinodes.insert(antinode);
                }
            }
        }
    }

    let antinodes = antinodes
        .iter()
        .filter(|a| a.0 >= 0 && a.1 >= 0 && a.0 < rows && a.1 < cols)
        .collect::<Vec<_>>();

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashMap::new();

    let mut rows = 0_i32;
    let mut cols = 0_i32;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                let m = map.entry(c).or_insert(vec![]);
                m.push((i as i32, j as i32));
            }
            cols = (j + 1) as i32;
        }
        rows = (i + 1) as i32;
    }

    let mut antinodes = HashSet::new();
    for (_c, positions) in map.iter() {
        for p in positions {
            antinodes.insert(*p);
            for q in positions {
                if p != q {
                    let diff = (p.0 - q.0, p.1 - q.1);

                    let mut antinode = *p;
                    loop {
                        antinode = (antinode.0 + diff.0, antinode.1 + diff.1);
                        if antinode.0 < 0
                            || antinode.1 < 0
                            || antinode.0 >= rows
                            || antinode.1 >= cols
                        {
                            break;
                        }
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    let antinodes = antinodes
        .iter()
        .filter(|a| a.0 >= 0 && a.1 >= 0 && a.0 < rows && a.1 < cols)
        .collect::<Vec<_>>();

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
