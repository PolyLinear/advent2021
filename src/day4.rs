#![allow(unused)]
use std::error::Error;

/*
* bins = ['10000', '01000', '00100', '00010', '00001']
* [int("0b" + x * 5, 2) for x in bins]
* [17318416, 8659208, 4329604, 2164802, 1082401]
*
* bins = 31
* [bins << 5 * i for i in range(5)]
* [31, 992, 31744, 1015808, 32505856]
*/
const ROWS: [u32; 10] = [
    31, 992, 31744, 1015808, 32505856, 17318416, 8659208, 4329604, 2164802, 1082401,
];

#[derive(Debug)]
struct BingoState {
    numbers: Vec<u8>,
    tables: Vec<Vec<u8>>,
}

const WIDTH: usize = 5;
fn parse_input(path: &str) -> Result<BingoState, Box<dyn Error>> {
    let bind = std::fs::read_to_string(path)?;
    let mut iter = bind.lines();
    let numbers: Vec<u8> = iter
        .next()
        .ok_or("No Bingo Line is present")?
        .split(',')
        .filter_map(|token| token.parse().ok())
        .collect();

    let mut tables = Vec::with_capacity(20);
    while iter.next().is_some() {
        let stuff: Vec<u8> = iter
            .by_ref()
            .take(WIDTH)
            .flat_map(|line| line.split(' ').filter_map(|token| token.parse().ok()))
            .collect();

        tables.push(stuff);
    }

    Ok(BingoState { numbers, tables })
}

fn calculate_points(table: &[u8], map: u32) -> u32 {
    table
        .iter()
        .enumerate()
        .map(|(i, value)| ((map >> i) & 1 ^ 1) * *value as u32)
        .sum()
}

fn day4_part1(state: &BingoState) -> Result<u32, Box<dyn Error>> {
    let mut mapping = vec![0u32; state.tables.len()];
    for number in state.numbers.iter() {
        for (i, table) in state.tables.iter().enumerate() {
            if let Some(index) = table.iter().position(|value| value == number) {
                mapping[i] |= 1 << index;
                if ROWS.iter().any(|mask| (mapping[i] & *mask) == *mask) {
                    return Ok(*number as u32 * calculate_points(table, mapping[i]));
                }
            }
        }
    }
    Err("No winning bingo table".into())
}

fn day4_part2(state: &BingoState) -> Result<u32, Box<dyn Error>> {
    #[derive(Default, Clone)]
    struct BingoMap {
        board: u32,
        found: bool,
    }

    let mut mapping = vec![BingoMap::default(); state.tables.len()];
    let mut count = 0;

    for number in state.numbers.iter() {
        for (i, table) in state.tables.iter().enumerate() {
            if mapping[i].found {
                continue;
            }
            if let Some(index) = table.iter().position(|value| value == number) {
                mapping[i].board |= 1 << index;
                if ROWS.iter().any(|mask| (mapping[i].board & *mask) == *mask) {
                    mapping[i].found = true;
                    count += 1;
                    if count == state.tables.len() {
                        return Ok(*number as u32 * calculate_points(table, mapping[i].board));
                    }
                }
            }
        }
    }

    Err("No winning bingo table".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day4() -> Result<(), Box<dyn Error>> {
        let data = parse_input("src/input/day4.txt")?;
        assert_eq!(35711, day4_part1(&data)?);
        assert_eq!(5586, day4_part2(&data)?);
        Ok(())
    }
}
