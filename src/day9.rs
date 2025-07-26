use std::{
    collections::{BinaryHeap, HashSet},
    error::Error,
    usize,
};
fn parse_input(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let data = std::fs::read(path)?
        .split(|b| b.is_ascii_whitespace())
        .filter_map(|line| (!line.is_empty()).then_some(line.to_owned()))
        .collect();
    Ok(data)
}

const ADJ: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
fn day9_part1(data: &[Vec<u8>]) -> u16 {
    let mut result = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if ADJ.iter().all(|(i, j)| {
                data.get((y as isize + j) as usize)
                    .and_then(|l| l.get((x as isize + i) as usize))
                    .map(|n| col < n)
                    .unwrap_or(true)
            }) {
                result += col.abs_diff(b'0') as u16 + 1
            }
        }
    }
    result
}

/*
* Originally, this method relied on a HashSet to store visited tiles. The new version instead
* relies on setting the value to a magic number (9) to denote visited sections. This made the
* implementation easier. A recursive solution, however, would have been easier to implement that an iterative DFS
*/

fn day9_part2(data: &mut [Vec<u8>]) -> u32 {
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(32);
    let mut results: Vec<u32> = Vec::with_capacity(32);

    let mut dfs = |data: &mut [Vec<u8>], row: usize, col: usize| {
        let mut count = 0;
        stack.push((row, col));
        while !stack.is_empty() {
            let (row, col) = stack.pop().unwrap();
            if data[row][col] == b'9' {
                continue;
            }
            data[row][col] = b'9';
            count += 1;
            for (i, j) in ADJ.iter() {
                let row_i = (row as isize + i) as usize;
                let col_i = (col as isize + j) as usize;
                if data
                    .get(row_i)
                    .and_then(|l| l.get(col_i))
                    .map(|n| *n < b'9')
                    .unwrap_or(false)
                {
                    stack.push((row_i, col_i));
                }
            }
        }
        count
    };

    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] != b'9' {
                results.push(dfs(data, row, col));
            }
        }
    }

    results.sort_unstable();
    results[results.len() - 3..].iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day9() -> Result<(), Box<dyn Error>> {
        let mut data = parse_input("src/input/day9.txt")?;
        assert_eq!(day9_part1(&data), 494);
        assert_eq!(day9_part2(&mut data), 1048128);
        Ok(())
    }
}
