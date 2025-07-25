#![allow(dead_code)]
use std::error::Error;
use std::fs;

fn parse_input(path: &str) -> Result<[u64; 9], Box<dyn Error>> {
    fs::read_to_string(path)?
        .split(",")
        .try_fold([0; 9], |mut acc, token| {
            let day: usize = token
                .trim()
                .parse()
                .map_err(|_| format!("Invalid digit: {}", token))?;
            acc[day] += 1;
            Ok(acc)
        })
}

fn simulate(mut data: [u64; 9], days: usize) -> u64 {
    (0..days).for_each(|_| {
        let add = data[0];
        data.rotate_left(1);
        data[6] += add;
        data[8] = add;
    });
    data.into_iter().fold(0, |acc, val| acc + val)
}

fn day6_part1(data: &[u64; 9]) -> u64 {
    simulate(data.clone(), 80)
}

fn day6_part2(data: &[u64; 9]) -> u64 {
    simulate(data.clone(), 256)
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn day6() -> Result<(), Box<dyn Error>> {
        let data = parse_input("src/input/day6.txt")?;
        assert_eq!(day6_part1(&data), 388419);
        assert_eq!(day6_part2(&data), 1740449478328);
        Ok(())
    }
}
