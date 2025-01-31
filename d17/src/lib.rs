use std::error::Error;
use std::fs;
use std::time::Instant;

//1947 too high
fn part1(input: String) -> isize {
    let mut buf = Vec::from([0]);
    buf.reserve(2017);
    let mut i = 0;
    let step = 312;
    for val in 1..2018 {
        i = ((i + step) % val) + 1;
        buf.insert(i, val);
    }

    return buf[(i + 1) % buf.len()] as isize;
}

fn part2(input: String) -> isize {
    let mut i = 0;
    let step = 312;
    let mut res = 0;
    for val in 1..50000001 {
        i = ((i + step) % val) + 1;
        if i == 1 {
            res = val;
        }
    }

    return res;
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
