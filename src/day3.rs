#![allow(unused)]
use std::error::Error;

//required to get the lower 12 bits of a line
const MASK: u32 = 4095;

fn parse_input(path: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    let data = std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| u16::from_str_radix(line, 2).ok())
        .collect();

    Ok(data)
}

fn day3_part1(data: &[u16]) -> u32 {
    let mut frequencies = [0u16; 12];

    //calculate the frequency of 1s
    for &binary in data.iter() {
        for (i, freq) in frequencies.iter_mut().enumerate() {
            *freq += (binary >> i) & 0x1;
        }
    }

    let gamma = frequencies.iter().enumerate().fold(0, |acc, (i, &v)| {
        let v = (v as usize * 2 >= data.len()) as u32;
        acc | (v << i)
    }) as u32;

    gamma * (!gamma & MASK)
}

fn day3_part2(data: &[u16]) -> u32 {
    let mut get_frequency_at_offset = |data: &mut [u16], shift| {
        (data.iter().filter(|&v| (v >> shift) & 0x1 == 1).count() * 2 >= data.len()) as u16
    };

    let mut oxygen = data.to_vec();
    let mut co2 = data.to_vec();
    (0..12).rev().for_each(|pos| {
        if oxygen.len() > 1 {
            let ox_freq = get_frequency_at_offset(&mut oxygen, pos as usize);
            oxygen.retain(|byte| (byte >> pos) & 0x1 == ox_freq);
        }

        if co2.len() > 1 {
            let c02_freq = get_frequency_at_offset(&mut co2, pos as usize);
            co2.retain(|byte| (byte >> pos) & 0x1 != c02_freq);
        }
    });

    oxygen[0] as u32 * co2[0] as u32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day3() -> Result<(), Box<dyn Error>> {
        let data = parse_input("src/input/day3.txt")?;
        assert_eq!(day3_part1(&data), 3549854);
        assert_eq!(day3_part2(&data), 3765399);
        Ok(())
    }
}
