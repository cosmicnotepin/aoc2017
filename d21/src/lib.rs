use ndarray::{arr2, s, Array2, Axis};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn flipped_horiz(pat: &Array2<char>) -> Array2<char> {
    let mut res = pat.clone();
    res.invert_axis(Axis(0));
    res
}

fn rot90(pat: &Array2<char>) -> Array2<char> {
    let mut res = pat.clone();
    res.swap_axes(0, 1);
    res.invert_axis(Axis(0));
    res
}

fn to_array2(s: &str) -> Array2<char> {
    let v: Vec<_> = s.chars().filter(|&c| c != '/').collect();
    match s.len() {
        5 => Array2::from_shape_vec((2, 2), v).unwrap(),
        11 => Array2::from_shape_vec((3, 3), v).unwrap(),
        19 => Array2::from_shape_vec((4, 4), v).unwrap(),
        _ => panic!(),
    }
}

fn part1(input: String, iters: usize) -> usize {
    let mut rules: HashMap<Array2<char>, Array2<char>> = HashMap::new();
    for l in input.lines() {
        let (from_s, to_s) = l.split_once(" => ").unwrap();
        let (mut from, to) = (to_array2(from_s), to_array2(to_s));
        for _ in 0..4 {
            rules.insert(from.clone(), to.clone());
            rules.insert(flipped_horiz(&from), to.clone());
            from = rot90(&from);
        }
    }
    let mut art = arr2(&[['.', '#', '.'], ['.', '.', '#'], ['#', '#', '#']]);

    for _ in 0..iters {
        let size = art.shape()[0];
        let divs;
        let div;
        if size % 2 == 0 {
            divs = size / 2;
            div = 2;
        } else {
            divs = size / 3;
            div = 3;
        }
        let new_size = size + divs;
        let new_div = div + 1;
        let mut new_art = Array2::from_elem((new_size, new_size), '.');
        for r in 0..divs {
            for c in 0..divs {
                let to_get = art
                    .slice(s![r * div..(r + 1) * div, c * div..(c + 1) * div])
                    .to_owned();
                let to = rules.get(&to_get);
                new_art
                    .slice_mut(s![
                        r * new_div..(r + 1) * new_div,
                        c * new_div..(c + 1) * new_div
                    ])
                    .assign(to.clone().unwrap());
            }
        }
        art = new_art;
    }
    return art.iter().filter(|&c| *c == '#').count();
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1, 5);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part1(input2, 18);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"
            .to_string();
        assert_eq!(12, part1(input, 2));
    }
}
