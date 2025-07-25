#![allow(unused)]
use std::{collections::HashMap, error::Error, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl Line {
    fn straight_iter(&self) -> Box<dyn Iterator<Item = (u16, u16)> + '_> {
        let Line { x1, y1, x2, y2 } = *self;
        if x1 == x2 {
            return Box::new((y1.min(y2)..=y1.max(y2)).map(move |y| (x1, y)).into_iter());
        }
        Box::new((x1.min(x2)..=x1.max(x2)).map(move |x| (x, y1)).into_iter())
    }

    fn is_straight(&self) -> bool {
        self.x2 == self.x1 || self.y1 == self.y2
    }
}

fn parse_coord(s: &str) -> Result<(u16, u16), &'static str> {
    let (x, y) = s.split_once(',').ok_or("Missing comma between coords")?;
    let x = x.trim().parse().map_err(|_| "Invalid x coords")?;
    let y = y.trim().parse().map_err(|_| "Invalid y coords")?;
    Ok((x, y))
}

fn day5_part1(input: &[Line]) -> u32 {
    let mut map: HashMap<(u16, u16), u8> = HashMap::new();
    let mut count = 0;

    for line in input.iter().filter(|line| line.is_straight()) {
        for coord in line.straight_iter() {
            if *map.entry(coord).and_modify(|v| *v += 1).or_insert(1) == 2 {
                count += 1;
            }
        }
    }
    count
}

impl FromStr for Line {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pair1, pair2) = s.split_once(" -> ").ok_or("Missing arrow")?;
        let (x1, y1) = parse_coord(pair1)?;
        let (x2, y2) = parse_coord(pair2)?;
        Ok(Line { x1, y1, x2, y2 })
    }
}

fn parse_input(path: &str) -> Result<Vec<Line>, Box<dyn Error>> {
    std::fs::read_to_string(path)?
        .lines()
        .map(|line| Line::from_str(line).map_err(|val| val.into()))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn day5() -> Result<(), Box<dyn Error>> {
        let input = parse_input("src/input/day5.txt")?;
        assert_eq!(day5_part1(&input), 7380);
        Ok(())
    }
}
