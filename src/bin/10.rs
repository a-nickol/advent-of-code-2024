use std::collections::HashSet;

use grid::Grid;

advent_of_code::solution!(10);

fn search_path(grid: &Grid<u32>, position: (usize, usize), c: u32) -> Vec<(usize, usize)> {
    let mut path = vec![];
    if grid[position] == c {
        if c == 9 {
            return vec![position];
        } else {
            if position.0 > 0 {
                path.append(&mut search_path(&grid, (position.0 - 1, position.1), c + 1));
            }
            if position.0 + 1 < grid.rows() {
                path.append(&mut search_path(&grid, (position.0 + 1, position.1), c + 1));
            }
            if position.1 > 0 {
                path.append(&mut search_path(&grid, (position.0, position.1 - 1), c + 1));
            }
            if position.1 + 1 < grid.cols() {
                path.append(&mut search_path(&grid, (position.0, position.1 + 1), c + 1));
            }
        }
    }
    path
}

fn count_path(grid: &Grid<u32>, position: (usize, usize), c: u32) -> usize {
    let mut count = 0_usize;
    if grid[position] == c {
        if c == 9 {
            return 1;
        } else {
            if position.0 > 0 {
                count += count_path(&grid, (position.0 - 1, position.1), c + 1);
            }
            if position.0 + 1 < grid.rows() {
                count += count_path(&grid, (position.0 + 1, position.1), c + 1);
            }
            if position.1 > 0 {
                count += count_path(&grid, (position.0, position.1 - 1), c + 1);
            }
            if position.1 + 1 < grid.cols() {
                count += count_path(&grid, (position.0, position.1 + 1), c + 1);
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();
    let v: Vec<u32> = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect();
    let grid = Grid::from_vec(v, input.lines().next().unwrap().len());

    let mut sum = 0;
    for (i, c) in grid.indexed_iter() {
        if *c == 0 {
            let set: HashSet<(usize, usize)> =
                HashSet::from_iter(search_path(&grid, i, 0).iter().cloned());
            sum += set.len();
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();
    let v: Vec<u32> = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect();
    let grid = Grid::from_vec(v, input.lines().next().unwrap().len());

    let mut sum = 0;
    for (i, c) in grid.indexed_iter() {
        if *c == 0 {
            sum += count_path(&grid, i, 0);
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
