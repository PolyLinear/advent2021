use std::error::Error;

fn parse_input(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let data = std::fs::read(path)?
        .split(|b| b.is_ascii_whitespace())
        .filter_map(|line| (!line.is_empty()).then_some(line.to_owned()))
        .collect::<Vec<_>>();

    Ok(data)
}

const ADJ: [(isize, isize); 8] = [
    (1, 1),   //bottom right
    (-1, -1), //top left
    (-1, 1),  //top right
    (1, -1),  //bottom left
    (0, 1),   //right
    (1, 0),   //down
    (-1, 0),  //up
    (0, -1),  //left
];

fn dfs(data: &mut [Vec<u8>], row: usize, col: usize) -> u64 {
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(32);
    let mut result = 0;
    stack.push((row, col));
    while !stack.is_empty() {
        let (row, col) = stack.pop().unwrap();
        if data[row][col] < TARGET {
            continue;
        }
        data[row][col] = b'0';
        result += 1;
        for (y, x) in ADJ.iter() {
            let (row_i, col_i) = (((row as isize) + y) as usize, ((col as isize) + x) as usize);
            if let Some(val) = data.get_mut(row_i).and_then(|r| r.get_mut(col_i)) {
                if *val != b'0' {
                    *val += 1;
                    stack.push((row_i, col_i));
                }
            }
        }
    }

    result
}

const TARGET: u8 = b'9' + 1;
fn day11_part1(data: &mut [Vec<u8>]) -> u64 {
    let mut count = 0;

    (0..100).for_each(|_| {
        data.iter_mut()
            .for_each(|line| line.iter_mut().for_each(|v| *v += 1));

        for row in 0..data.len() {
            for col in 0..data[0].len() {
                if data[row][col] >= TARGET {
                    count += dfs(data, row, col);
                }
            }
        }
    });

    count
}

fn day11_part2(data: &mut [Vec<u8>]) -> Result<u64, Box<dyn Error>> {
    let expected = data.len() * data[0].len();
    for step in 0..1000 {
        data.iter_mut()
            .for_each(|line| line.iter_mut().for_each(|v| *v += 1));
        let mut count = 0;
        for row in 0..data.len() {
            for col in 0..data[0].len() {
                if data[row][col] >= TARGET {
                    count += dfs(data, row, col) as usize;
                }
            }
        }

        if count == expected {
            return Ok(step + 1);
        }
    }

    Err("Failed to find for day11 part 2".into())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn day11() -> Result<(), Box<dyn Error>> {
        let mut data = parse_input("src/input/day11.txt")?;
        assert_eq!(day11_part1(&mut data.clone()), 1741);
        assert_eq!(day11_part2(&mut data)?, 440);
        Ok(())
    }
}
