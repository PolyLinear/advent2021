#![allow(unused)]
use std::error::Error;
use std::fs;
#[cfg(test)]

fn parse_input(path: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();
    Ok(data)
}

fn day1_part1(data: &[u32]) -> usize {
    data.windows(2).filter(|e| e[1] > e[0]).count()
}

fn day1_part2(data: &[u32]) -> usize {
    let win: Vec<u32> = data
        .windows(3)
        .map(|window| window.iter().sum::<u32>())
        .collect();

    //same as day1_part1. Could call it,but I want each function to be it's own unique solution
    win.windows(2).filter(|e| e[1] > e[0]).count()
}

mod tests {
    use super::*;
    #[test]
    pub fn day1() -> Result<(), Box<dyn Error>> {
        let data = parse_input("src/input/day1.txt")?;
        assert_eq!(day1_part1(&data), 1557);
        assert_eq!(day1_part2(&data), 1608);
        Ok(())
    }
}
