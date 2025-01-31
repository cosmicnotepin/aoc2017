use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> (String, usize) {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut pos = (0, map[0].iter().position(|&c| c == '|').unwrap() as isize);
    let mut dir = (1isize, 0isize);
    let mut res = String::new();
    let mut steps = 0;
    loop {
        let (row_n, col_n) = (pos.0 + dir.0, pos.1 + dir.1);
        steps += 1;
        let next = map[row_n as usize][col_n as usize];
        let (row_d, col_d) = dir;
        match next {
            '|' | '-' => pos = (row_n, col_n),
            ' ' => break,
            '+' => {
                if map[(row_n + col_d) as usize][(col_n + row_d) as usize] == ' ' {
                    pos = (row_n, col_n);
                    dir = (-col_d, -row_d);
                } else {
                    pos = (row_n, col_n);
                    dir = (col_d, row_d);
                }
            }
            c => {
                res.push(c);
                pos = (row_n, col_n);
            }
        }
    }
    return (res, steps);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let (p1, p2) = part1(input1);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    println!("part 2: {} in {:.2?}", p2, before1.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = fs::read_to_string("input1_t").unwrap();
        let (p1, p2) = part1(input);
        assert_eq!("ABCDEF", p1);
        assert_eq!(38, p2);
    }
}
