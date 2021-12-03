use aoc::helpers::{read_input, std_parse};
use eyre::{Error, Result};
use std::str::FromStr;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let (h, d) = find_pos(std_parse(&input)?);
    println!("part1: {}", h * d);
    let (h, d, _) = find_aim(std_parse(input)?);
    println!("part2: {}", h * d);

    Ok(())
}

enum Direction {
    Down(i64),
    Up(i64),
    Forward(i64),
}

fn find_pos(input: Vec<Direction>) -> (i64, i64) {
    input.iter().fold((0, 0), |(hor, depth), dir| match dir {
        Direction::Down(d) => (hor, depth + d),
        Direction::Up(d) => (hor, depth - d),
        Direction::Forward(d) => (hor + d, depth),
    })
}

fn find_aim(input: Vec<Direction>) -> (i64, i64, i64) {
    input
        .iter()
        .fold((0, 0, 0), |(hor, depth, aim), dir| match dir {
            Direction::Down(d) => (hor, depth, aim + d),
            Direction::Up(d) => (hor, depth, aim - d),
            Direction::Forward(d) => (hor + d, depth + (aim * d), aim),
        })
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(' ') {
            Some(("forward", i)) => Direction::Forward(i.parse()?),
            Some(("up", i)) => Direction::Up(i.parse()?),
            Some(("down", i)) => Direction::Down(i.parse()?),
            _ => return Err(Error::msg(format!("Not Forward/Up or Down: {}", s)))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() -> Result<()> {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";

        assert_eq!(find_pos(std_parse(input)?), (15, 10));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";

        assert_eq!(find_aim(std_parse(input)?), (15, 60, 10));

        Ok(())
    }
}
