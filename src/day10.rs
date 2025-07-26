#![allow(unused)]
use std::error::Error;

fn parse_input(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let data = std::fs::read(path)?
        .split(|b| b.is_ascii_whitespace())
        .filter_map(|line| (!line.is_empty()).then_some(line.to_owned()))
        .collect();
    Ok(data)
}

#[inline]
fn check_conditions(top: u8, b: u8) -> bool {
    (top == b'{' && b == b'}')
        || (top == b'(' && b == b')')
        || (top == b'<' && b == b'>')
        || (top == b'{' && b == b'}')
        || (top == b'[' && b == b']')
}

#[inline]
fn get_points_day_1(b: u8) -> u64 {
    match b {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!("improper char found"),
    }
}

fn get_points_day_2(b: u8) -> u64 {
    match b {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => unreachable!("improper char found"),
    }
}

#[derive(Default)]
struct Answer {
    day1: u64,
    day2: u64,
}

/*
* Since part 2 depends on filtering the "corrupted" lines from part1 it makes sense to process the inputed lines at the same time. This could be further improved by processing the lines as you receive them, since each line is independent. I try to separate the solutions for part1, part2, and parsing, but this has shown to be more effective
*
*/
fn day10_sol(data: &[Vec<u8>]) -> Answer {
    let mut stack: Vec<u8> = Vec::with_capacity(128);
    let mut answer = Answer::default();
    let mut good: Vec<u64> = Vec::new();

    for line in data.iter() {
        let mut corrupted = false;
        for b in line.iter() {
            let b = *b;
            if b"{(<[".iter().find(|&&v| v == b).is_some() {
                stack.push(b);
            } else {
                let top = stack.pop().unwrap();
                if !check_conditions(top, b) {
                    stack.clear();
                    corrupted = true;
                    answer.day1 += get_points_day_1(b);
                    break;
                }
            }
        }

        if !corrupted {
            good.push(
                stack
                    .drain(..)
                    .rev()
                    .fold(0, |acc, e| (acc * 5) + get_points_day_2(e)),
            );
        }
    }

    good.sort_unstable();
    answer.day2 = good[good.len() / 2];
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day10() -> Result<(), Box<dyn Error>> {
        let data = parse_input("src/input/day10.txt")?;
        let Answer { day1, day2 } = day10_sol(&data);
        assert_eq!(day1, 243939);
        assert_eq!(day2, 2421222841);
        Ok(())
    }
}
