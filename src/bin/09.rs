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

#[derive(Clone, Copy, Debug)]
struct File {
    len: u64,
    pos: u64,
    idx: Option<u64>,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = vec![];
    let mut pos = 0;
    let mut idx_vec = vec![];
    for (i, c) in input.trim().chars().enumerate() {
        let occupied = i % 2 == 0;
        let id = (i / 2) as u64;
        let len = c.to_string().parse::<u64>().unwrap();
        if len > 0 {
            disk.push(File {
                len,
                pos,
                idx: if occupied { Some(id) } else { None },
            });
            if occupied {
                idx_vec.push(id);
            }
        }
        pos += len;
    }

    loop {
        let file_idx = idx_vec.pop();
        if file_idx.is_none() {
            break;
        }

        let file_idx = file_idx.unwrap();
        let j = disk.iter().position(|f| f.idx == Some(file_idx)).unwrap();

        let len = disk[j].len;
        let mut i = 0;
        loop {
            if i == disk.len() - 1 || (disk[i].idx.is_none() && disk[i].len >= len) {
                break;
            }
            i += 1;
        }

        if disk[i].pos < disk[j].pos && disk[i].len >= len && disk[i].idx.is_none() {
            let d = disk[i];
            disk[i] = disk[j];
            disk[i].pos = d.pos;
            disk[j].idx = None;
            if d.len > disk[i].len {
                disk.insert(
                    i + 1,
                    File {
                        len: d.len - disk[i].len,
                        pos: disk[i].pos + disk[i].len,
                        idx: None,
                    },
                );
            }
        }
    }

    let mut sum = 0_u64;
    for file in disk.iter() {
        for i in file.pos..(file.pos + file.len) {
            if let Some(idx) = file.idx {
                sum += idx * i;
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
