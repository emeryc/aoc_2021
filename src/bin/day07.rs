use aoc::helpers::{read_input, std_parse};
use eyre::Result;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let t: Vec<u64> = parse_ints(input.as_str().trim())?;
    println!("part1: {}", min_fuel(t.as_slice()));
    println!("part2: {}", min_fuel2(t.as_slice()));

    Ok(())
}

fn parse_ints(input: &str) -> Result<Vec<u64>> {
    input
        .split(",")
        .map(|d| Ok(d.parse()?))
        .collect::<Result<_>>()
}

fn min_fuel(positions: &[u64]) -> u64 {
    let median = median(positions);

    positions
        .iter()
        .map(|v| i64::abs(median as i64 - *v as i64) as u64)
        .sum()
}

fn median(numbers: &[u64]) -> u64 {
    let mut numbers: Vec<_> = numbers.iter().collect();
    numbers.sort();
    let mid = numbers.len() / 2;
    *numbers[mid]
}

fn min_fuel2(positions: &[u64]) -> u64 {
    let mean = dbg!(mean(positions));
    let mut last = (mean, u64::MAX);
    loop {
        let t1 = positions
            .iter()
            .map(|v| fact(i64::abs((last.0 + 1) as i64 - *v as i64) as u64))
            .sum();
        let t2 = positions
            .iter()
            .map(|v| fact(i64::abs((last.0 - 1) as i64 - *v as i64) as u64))
            .sum();
        if t1 < last.1 {
            last = (last.0 + 1, t1);
        } else if t2 < last.1 {
            last = (last.0 - 1, t2);
        } else {
            return dbg!(last).1;
        }
    }
}

fn fact(num: u64) -> u64 {
    (1..=num).sum()
}

fn mean(numbers: &[u64]) -> u64 {
    let mean = dbg!(numbers.iter().sum::<u64>() as f64 / numbers.len() as f64);

    mean.round() as u64
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(37, min_fuel(parse_ints(input)?.as_slice()));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(168, min_fuel2(parse_ints(input)?.as_slice()));

        Ok(())
    }
}
