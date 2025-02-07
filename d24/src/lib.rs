use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::time::Instant;

fn build(
    last: usize,
    parts: &HashMap<usize, Vec<usize>>,
    used: &mut HashSet<(usize, usize)>,
    p1: bool,
) -> (usize, usize) {
    let mut res;
    if p1 {
        res = (used.iter().map(|(l, r)| l + r).sum(), used.len());
    } else {
        res = (used.len(), used.iter().map(|(l, r)| l + r).sum());
    }
    if let Some(p_v) = parts.get(&last) {
        for p in p_v {
            if used.contains(&(last, *p)) || used.contains(&(*p, last)) {
                continue;
            }
            used.insert((last, *p));
            res = max(res, build(*p, parts, used, p1));
            used.remove(&(last, *p));
        }
    }
    res
}

fn part1(input: String, p1: bool) -> usize {
    let mut parts: HashMap<usize, Vec<usize>> = HashMap::new();
    for part_s in input.lines() {
        let (l_s, r_s) = part_s.split_once("/").unwrap();
        let l = l_s.parse().unwrap();
        let r = r_s.parse().unwrap();
        parts.entry(l).or_default().push(r);
        if l != r {
            parts.entry(r).or_default().push(l);
        }
    }
    let mut used = HashSet::new();
    let res = build(0, &parts, &mut used, p1);
    if p1 {
        res.0
    } else {
        res.1
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1, true);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part1(input2, false);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"
            .to_string();
        assert_eq!(31, part1(input, true));
    }

    #[test]
    fn p2_1() {
        let input = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"
            .to_string();
        assert_eq!(19, part1(input, false));
    }
}
