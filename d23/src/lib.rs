use num_prime::nt_funcs::is_prime64;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn get(s: &str, regs: &HashMap<&str, isize>) -> isize {
    if let Ok(n) = s.parse() {
        return n;
    }
    if regs.contains_key(s) {
        return *regs.get(s).unwrap();
    }
    return 0;
}

fn part1(input: String) -> isize {
    let prg: Vec<Vec<&str>> = input.lines().map(|l| l.split(' ').collect()).collect();
    let mut ip: isize = 0;
    let mut regs = HashMap::new();
    let mut res = 0;
    while ip >= 0 && ip < prg.len() as isize {
        let instr = &prg[ip as usize];
        match instr[0] {
            "set" => {
                *regs.entry(instr[1]).or_default() = get(instr[2], &regs);
            }
            "sub" => {
                *regs.entry(instr[1]).or_default() -= get(instr[2], &regs);
            }
            "mul" => {
                *regs.entry(instr[1]).or_default() *= get(instr[2], &regs);
                res += 1;
            }
            "jnz" => {
                if get(instr[1], &regs) != 0 {
                    ip += get(instr[2], &regs);
                    continue;
                }
            }
            _ => panic!(),
        }
        ip += 1;
    }
    println!("NO RECEIVE");
    println!("{}", regs.get(&"h").unwrap());
    return res;
}

fn part2(input: String) -> isize {
    //1000 too high
    //912 too low
    //see blah1 for why this is that :)
    let mut res = 0;
    for n in (109900..126901).step_by(17) {
        if !is_prime64(n) {
            res += 1;
        }
    }
    res
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
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"
            .to_string();
        assert_eq!(4, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
"
        .to_string();
        assert_eq!(0, part2(input));
    }
}
