use itertools::Itertools;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn exchange(a: usize, b: usize, ps: &mut Vec<char>) {
    let e1 = ps[a];
    ps[a] = ps[b];
    ps[b] = e1;
}

fn partner(a: char, b: char, ps: &mut Vec<char>) {
    let ia = ps.iter().position(|&e| e == a).unwrap();
    let ib = ps.iter().position(|&e| e == b).unwrap();
    exchange(ia, ib, ps);
}

fn spin(s: usize, ps: Vec<char>) -> Vec<char> {
    let mut new_ps = Vec::from(['x'; 16]);
    for (i, p) in ps.iter().enumerate() {
        new_ps[(i + s) % 16] = *p;
    }
    new_ps
}

fn part1(input: String) -> isize {
    let mut programs: Vec<char> = ('a'..='p').collect();
    let starting: Vec<char> = ('a'..='p').collect();
    //32: nechdblkjmgpfaoi
    //31: pabheomkcjfndlig
    for i in 0..40 {
        for mv_s in input.trim().split(',') {
            let mv = &mv_s[0..1];
            match mv {
                "s" => programs = spin(mv_s[1..].parse().unwrap(), programs),
                "x" => {
                    let (a, b) = &mv_s[1..]
                        .split('/')
                        .map(|n| n.parse::<usize>().unwrap())
                        .next_tuple()
                        .unwrap();
                    exchange(*a, *b, &mut programs);
                }
                "p" => {
                    let (a, b) = &mv_s[1..]
                        .split('/')
                        .map(|n| n.chars().next().unwrap())
                        .next_tuple()
                        .unwrap();
                    partner(*a, *b, &mut programs);
                }

                _ => (),
            }
        }
        if programs == starting {
            println!("i : {:?}", i);
            break;
        }
    }
    println!("programs : {:?}", programs.iter().collect::<String>());
    return input.len().try_into().unwrap();
}

fn part2(input: String) -> isize {
    return input.len().try_into().unwrap();
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
        let input = "".to_string();
        assert_eq!(0, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
