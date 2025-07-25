use std::error::Error;

#[derive(Default, Clone)]
struct CrabState {
    lower: u32,
    higher: u32,
    data: Vec<u32>,
}

fn parse_input(path: &str) -> Result<CrabState, Box<dyn Error>> {
    std::fs::read_to_string(path)?
        .split(",")
        .try_fold(CrabState::default(), |mut acc, e| {
            let pos = e.trim().parse()?;
            acc.lower = acc.lower.min(pos);
            acc.higher = acc.higher.max(pos);
            acc.data.push(pos);
            Ok(acc)
        })
}

fn day7_part1(cs: &CrabState) -> Result<u32, &'static str> {
    (cs.lower..=cs.higher)
        .map(|pivot| cs.data.iter().map(|v| v.abs_diff(pivot)).sum())
        .min()
        .ok_or("Could not find minimum in day 7 part 1")
}

fn day7_part2(cs: &CrabState) -> Result<u32, &'static str> {
    (cs.lower..=cs.higher)
        .map(|pivot| {
            cs.data
                .iter()
                .map(|v| {
                    let v = v.abs_diff(pivot);
                    v * (v + 1) / 2
                })
                .sum()
        })
        .min()
        .ok_or("Could not find minimum in day 7 part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() -> Result<(), Box<dyn Error>> {
        let data = parse_input("src/input/day7.txt")?;

        assert_eq!(day7_part1(&data)?, 352997);
        assert_eq!(day7_part2(&data)?, 101571302);
        Ok(())
    }
}
