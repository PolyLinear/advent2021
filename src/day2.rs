#![allow(unused)]
use std::{error::Error, fs};

/*
*Questions and Answers
*
* Question #1: Is this overly engineered?:
* Answer #1: Other solutions parse the input without the usage of enums and immediately use the
* values in the calculations. This current solution is slower, as it first creates a Vector of
* Directions and then handles the calculations. I mostly wanted an excuse to use Enums and the
* different traits provided by Rust.
*
*/

#[derive(Debug)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl TryFrom<&str> for Direction {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, magnitude) = value
            .split_once(' ')
            .ok_or("Invalid line provided".to_string())?;

        let direction = match (direction.as_bytes()[0], magnitude.trim().parse()?) {
            (b'f', mag) => Self::Forward(mag),
            (b'd', mag) => Self::Down(mag),
            (b'u', mag) => Self::Up(mag),
            _ => return Err("Could not parse line".into()),
        };

        Ok(direction)
    }
}

fn parse_input(path: &str) -> Result<Vec<Direction>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?
        .lines()
        .filter_map(|line| Direction::try_from(line).ok())
        .collect();

    Ok(data)
}

fn day2_part1(data: &[Direction]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for direction in data.iter() {
        match direction {
            Direction::Up(mag) => depth -= mag,
            Direction::Down(mag) => depth += mag,
            Direction::Forward(mag) => horizontal += mag,
        }
    }

    horizontal * depth
}

fn day2_part2(data: &[Direction]) -> u32 {
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);

    for direction in data.iter() {
        match direction {
            Direction::Up(mag) => aim -= mag,
            Direction::Down(mag) => aim += mag,
            Direction::Forward(mag) => {
                horizontal += mag;
                depth += aim * mag;
            }
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2() -> Result<(), Box<dyn Error>> {
        let data = parse_input("src/input/day2.txt")?;
        assert_eq!(1936494, day2_part1(&data));
        assert_eq!(1997106066, day2_part2(&data));
        Ok(())
    }
}
