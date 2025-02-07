use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> isize {
    let (init_s, states_s) = input.split_once("\n\n").unwrap();
    let mut init_it = init_s.lines();
    let mut state = init_it.next().unwrap().chars().rev().nth(1).unwrap();
    let steps: isize = init_it
        .next()
        .unwrap()
        .split(" ")
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();
    let mut states: HashMap<char, [(isize, isize, char); 2]> = HashMap::new();
    for state_s in states_s.split("\n\n") {
        let vals: Vec<char> = state_s
            .lines()
            .map(|l| {
                let last_word = l.split(" ").last().unwrap();
                let mut char_it = last_word.chars();
                char_it.next().unwrap()
            })
            .collect();
        let lr0 = match vals[3] {
            'r' => 1,
            'l' => -1,
            _ => panic!(),
        };
        let lr1 = match vals[7] {
            'r' => 1,
            'l' => -1,
            _ => panic!(),
        };
        states.insert(
            vals[0],
            [
                (vals[2].to_digit(10).unwrap() as isize, lr0, vals[4]),
                (vals[6].to_digit(10).unwrap() as isize, lr1, vals[8]),
            ],
        );
    }
    let mut tape: HashMap<isize, isize> = HashMap::new();
    let mut cursor = 0;
    for _ in 0..steps {
        let t_v = tape.entry(cursor).or_default();
        let s = states.get(&state).unwrap()[*t_v as usize];
        *t_v = s.0;
        cursor += s.1;
        state = s.2;
    }

    return tape.values().sum();
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
        let input = "\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
"
        .to_string();
        assert_eq!(3, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
