use std::collections::HashSet;

use grid::Grid;

advent_of_code::solution!(12);

fn find_field(grid: &Grid<char>, f: (usize, usize), c: &char) -> HashSet<(usize, usize)> {
    let mut fields = HashSet::new();
    add_to_field(grid, f, c, &mut fields);
    fields
}

fn add_to_field(
    grid: &Grid<char>,
    f: (usize, usize),
    c: &char,
    field: &mut HashSet<(usize, usize)>,
) {
    if grid.get(f.0, f.1) == Some(c) && !field.contains(&f) {
        field.insert(f);
        if f.0 > 0 {
            add_to_field(grid, (f.0 - 1, f.1), c, field);
        }
        if f.0 + 1 < grid.rows() {
            add_to_field(grid, (f.0 + 1, f.1), c, field);
        }
        if f.1 + 1 < grid.cols() {
            add_to_field(grid, (f.0, f.1 + 1), c, field);
        }
        if f.1 > 0 {
            add_to_field(grid, (f.0, f.1 - 1), c, field);
        }
    }
}

fn count_perimeter(field: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    for f in field {
        if f.0 > 0 {
            if field.get(&(f.0 - 1, f.1)).is_none() {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
        if field.get(&(f.0 + 1, f.1)).is_none() {
            perimeter += 1;
        }
        if f.1 > 0 {
            if field.get(&(f.0, f.1 - 1)).is_none() {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
        if field.get(&(f.0, f.1 + 1)).is_none() {
            perimeter += 1;
        }
    }
    perimeter
}

fn count_sides(field: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter = HashSet::new();
    for f in field {
        if f.0 > 0 {
            if field.get(&(f.0 - 1, f.1)).is_none() {
                perimeter.insert((f.clone(), 0));
            }
        } else {
            perimeter.insert((f.clone(), 0));
        }
        if field.get(&(f.0 + 1, f.1)).is_none() {
            perimeter.insert((f.clone(), 1));
        }
        if f.1 > 0 {
            if field.get(&(f.0, f.1 - 1)).is_none() {
                perimeter.insert((f.clone(), 2));
            }
        } else {
            perimeter.insert((f.clone(), 2));
        }
        if field.get(&(f.0, f.1 + 1)).is_none() {
            perimeter.insert((f.clone(), 3));
        }
    }
    let mut perimeter2 = perimeter.clone();
    for p in perimeter.iter() {
        if p.1 == 0 {
            if perimeter.contains(&((p.0 .0, p.0 .1 + 1), p.1)) {
                perimeter2.remove(&p);
            }
        }
        if p.1 == 1 {
            if perimeter.contains(&((p.0 .0, p.0 .1 + 1), p.1)) {
                perimeter2.remove(&p);
            }
        }
        if p.1 == 2 {
            if perimeter.contains(&((p.0 .0 + 1, p.0 .1), p.1)) {
                perimeter2.remove(&p);
            }
        }
        if p.1 == 3 {
            if perimeter.contains(&((p.0 .0 + 1, p.0 .1), p.1)) {
                perimeter2.remove(&p);
            }
        }
    }
    perimeter2.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();
    let v: Vec<char> = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let grid = Grid::from_vec(v, input.lines().next().unwrap().len());
    let mut fields = Vec::new();
    let mut hashset = HashSet::new();

    for (f, c) in grid.indexed_iter() {
        if hashset.get(&f).is_none() {
            let field = find_field(&grid, f, c);
            for i in find_field(&grid, f, c) {
                hashset.insert(i.clone());
            }
            fields.push(field);
        }
    }

    let mut sum = 0;
    for field in fields {
        let area = field.len();
        let perimeter = count_perimeter(&field);
        sum += area * perimeter;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();
    let v: Vec<char> = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let grid = Grid::from_vec(v, input.lines().next().unwrap().len());
    let mut fields = Vec::new();
    let mut hashset = HashSet::new();

    for (f, c) in grid.indexed_iter() {
        if hashset.get(&f).is_none() {
            let field = find_field(&grid, f, c);
            for i in find_field(&grid, f, c) {
                hashset.insert(i.clone());
            }
            fields.push(field);
        }
    }

    let mut sum = 0;
    for field in fields {
        let area = field.len();
        let perimeter = count_sides(&field);
        sum += area * perimeter;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
