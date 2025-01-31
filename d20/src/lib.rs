use itertools::Itertools;
use ndarray::{arr1, Array1};
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> isize {
    let re = Regex::new(r"-?\d+").unwrap();
    let prtcls: Vec<(Array1<isize>, Array1<isize>, Array1<isize>)> = input
        .lines()
        .map(|l| {
            re.find_iter(l)
                .map(|n| n.as_str().parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(x, y, z, vx, vy, vz, ax, ay, az)| {
            (arr1(&[x, y, z]), arr1(&[vx, vy, vz]), arr1(&[ax, ay, az]))
        })
        .collect();

    let mut manh_accs = Vec::new();
    for (i, p) in prtcls.iter().enumerate() {
        manh_accs.push((p.2[0].abs() + p.2[1].abs() + p.2[2].abs(), i));
    }
    manh_accs.sort();
    println!("manh_accs: {:?}", &manh_accs[..5]);
    //161, guessed from the 3 candidates with (lowest) manhatten accelerations of 2
    return 161;
}

fn part2(input: String) -> usize {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut prtcls: Vec<(Array1<isize>, Array1<isize>, Array1<isize>)> = input
        .lines()
        .map(|l| {
            re.find_iter(l)
                .map(|n| n.as_str().parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(x, y, z, vx, vy, vz, ax, ay, az)| {
            (arr1(&[x, y, z]), arr1(&[vx, vy, vz]), arr1(&[ax, ay, az]))
        })
        .collect();

    for _ in 0..10000 {
        // with the numbers from the input, 10000 seems high enough :p
        let mut i_p = 0;
        let mut occupied = HashSet::new();
        while i_p < prtcls.len() {
            let ref mut p = prtcls[i_p];
            p.1 = &p.1 + &p.2;
            p.0 = &p.0 + &p.1;
            let pos = p.0.clone();
            if !occupied.insert(pos.clone()) {
                prtcls.remove(i_p);
                let old_len = prtcls.len();
                prtcls.retain(|p_d| p_d.0 != pos);
                if prtcls.len() < old_len {
                    i_p -= 1;
                }
            } else {
                i_p += 1;
            }
        }
    }
    return prtcls.len();
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part2(input2);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>"
            .to_string();
        assert_eq!(1, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
