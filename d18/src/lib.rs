use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender};
use std::thread;
use std::time;
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
    let mut snd = 0;
    let mut regs = HashMap::new();
    while ip >= 0 && ip < prg.len() as isize {
        let instr = &prg[ip as usize];
        match instr[0] {
            "snd" => {
                snd = get(instr[1], &regs);
            }
            "set" => {
                *regs.entry(instr[1]).or_default() = get(instr[2], &regs);
            }
            "add" => {
                *regs.entry(instr[1]).or_default() += get(instr[2], &regs);
            }
            "mul" => {
                *regs.entry(instr[1]).or_default() *= get(instr[2], &regs);
            }
            "mod" => {
                *regs.entry(instr[1]).or_default() %= get(instr[2], &regs);
            }
            "rcv" => {
                if *regs.entry(instr[1]).or_default() != 0 {
                    return snd;
                }
            }
            "jgz" => {
                if *regs.entry(instr[1]).or_default() > 0 {
                    ip += get(instr[2], &regs);
                    continue;
                }
            }
            _ => panic!(),
        }
        ip += 1;
    }
    println!("NO RECEIVE");
    return snd;
}

fn prgrm(
    input: String,
    p: isize,
    snd: SyncSender<isize>,
    rcv: Receiver<isize>,
    rsend: Sender<isize>,
) {
    let prg: Vec<Vec<&str>> = input.lines().map(|l| l.split(' ').collect()).collect();
    let mut ip: isize = 0;
    let mut regs = HashMap::new();
    regs.insert("p", p);
    while ip >= 0 && ip < prg.len() as isize {
        let instr = &prg[ip as usize];
        match instr[0] {
            "snd" => {
                let _ = snd.send(get(instr[1], &regs));
                if p == 1 {
                    let _ = rsend.send(1);
                }
            }
            "set" => {
                *regs.entry(instr[1]).or_default() = get(instr[2], &regs);
            }
            "add" => {
                *regs.entry(instr[1]).or_default() += get(instr[2], &regs);
            }
            "mul" => {
                *regs.entry(instr[1]).or_default() *= get(instr[2], &regs);
            }
            "mod" => {
                *regs.entry(instr[1]).or_default() %= get(instr[2], &regs);
            }
            "rcv" => {
                *regs.entry(instr[1]).or_default() = rcv.recv().unwrap();
            }
            "jgz" => {
                if *regs.entry(instr[1]).or_default() > 0 {
                    ip += get(instr[2], &regs);
                    continue;
                }
            }
            _ => panic!(),
        }
        ip += 1
    }
    println!("Done");
}

fn part2(input: String) -> isize {
    //200796272
    let (s0, r1) = sync_channel(200000000);
    let (s1, r0) = sync_channel(200000000);
    let (sres, rres) = channel();
    let sres2 = sres.clone();
    let i0 = input.clone();
    let i1 = input.clone();
    let t0 = thread::spawn(|| {
        prgrm(i0, 0, s0, r0, sres);
    });
    let t1 = thread::spawn(|| {
        prgrm(i1, 1, s1, r1, sres2);
    });
    let mut res = 0;
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        while let Ok(_) = rres.try_recv() {
            res += 1;
        }
        println!("res : {:?}", res);
    }

    let _ = t0.join();
    let _ = t1.join();
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
