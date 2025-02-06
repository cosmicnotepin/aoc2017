use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String, start_pos: (isize, isize)) -> isize {
    let mut map: HashSet<(isize, isize)> = HashSet::new();
    for (r_i, row) in input.lines().enumerate() {
        for (c_i, col) in row.chars().enumerate() {
            if col == '#' {
                map.insert((r_i as isize, c_i as isize));
            }
        }
    }
    let mut pos = start_pos;
    let mut dir = (-1, 0);
    let mut res = 0;
    for _ in 0..10000 {
        if map.contains(&pos) {
            dir = (dir.1, -dir.0);
            map.remove(&pos);
        } else {
            dir = (-dir.1, dir.0);
            map.insert(pos);
            res += 1;
        }
        pos.0 += dir.0;
        pos.1 += dir.1;
    }
    return res;
}

fn part2(input: String, start_pos: (isize, isize)) -> isize {
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    for (r_i, row) in input.lines().enumerate() {
        for (c_i, col) in row.chars().enumerate() {
            if col == '#' {
                map.insert((r_i as isize, c_i as isize), 'i');
            }
        }
    }
    let mut pos = start_pos;
    let mut dir = (-1, 0);
    let mut res = 0;
    for _ in 0..10000000 {
        if !map.contains_key(&pos) {
            dir = (-dir.1, dir.0);
            map.insert(pos, 'w');
        } else {
            match map.get_mut(&pos) {
                Some(x) if *x == 'w' => {
                    *x = 'i';
                    res += 1;
                }
                Some(x) if *x == 'i' => {
                    dir = (dir.1, -dir.0);
                    *x = 'f'
                }
                Some('f') => {
                    dir = (-dir.0, -dir.1);
                    map.remove(&pos);
                }
                _ => panic!(),
            }
        }
        pos.0 += dir.0;
        pos.1 += dir.1;
    }
    return res;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1, (12, 12));
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part2(input2, (12, 12));
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
..#
#..
..."
        .to_string();
        assert_eq!(5587, part1(input, (1, 1)));
    }

    #[test]
    fn p2_1() {
        let input = "\
..#
#..
..."
        .to_string();
        assert_eq!(2511944, part2(input, (1, 1)));
    }
}
