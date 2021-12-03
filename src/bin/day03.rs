use aoc::helpers::{read_input, std_parse};
use eyre::{Error, Result};
use itertools::Itertools;
use std::{fmt::Debug, ops::Add, str::FromStr};

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let (g, e) = get_g_e(std_parse(&input)?);
    let g: u32 = g.into();
    let e: u32 = e.into();
    let ge: u32 = g * e;
    println!("part 1: {}", ge);
    let (o, c) = get_o_c(std_parse(input)?);
    println!("part 2: {}", o * c);

    Ok(())
}

#[derive(Clone)]
struct Line(Vec<u32>);

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line(
            s.chars()
                .map(|c| c.to_string().parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0.iter().map(|i| i.to_string()).join("")).as_str())
        //f.debug_tuple("Line").field(&self.0).finish()
    }
}

impl Add for Line {
    type Output = Line;

    fn add(self, rhs: Self) -> Self::Output {
        Line(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Line> for u32 {
    fn from(line: Line) -> Self {
        let mut o = 0u32;
        for i in line.0 {
            o = o << 1;
            o |= i;
        }
        o
    }
}

fn get_g_e(lines: Vec<Line>) -> (Line, Line) {
    let count = lines.len();
    let line_sum = lines
        .into_iter()
        .fold1(|a, b| a + b)
        .expect("At least one line");

    let mut gamma = Vec::<u32>::new();
    let mut epsilon = Vec::<u32>::new();
    for v in line_sum.0 {
        if v >= ((count as f32 / 2.0).ceil() as u32) {
            epsilon.push(0);
            gamma.push(1);
        } else {
            epsilon.push(1);
            gamma.push(0);
        }
    }
    (Line(gamma), Line(epsilon))
}

fn get_o_c(lines: Vec<Line>) -> (u32, u32) {
    let mut o = lines.clone();
    let mut pos = 0; //gamma.0.len();
    while o.len() > 1 {
        let (gamma, _) = get_g_e(o.clone());
        o = o
            .into_iter()
            .filter(|line| line.0[pos] == gamma.0[pos])
            .collect();
        pos += 1;
    }

    let mut c = lines.clone();
    let mut pos = 0;
    while c.len() > 1 {
        let (_, epsilon) = get_g_e(c.clone());
        c = c
            .into_iter()
            .filter(|line| line.0[pos] == epsilon.0[pos])
            .collect();
        pos += 1;
    }

    (
        o.into_iter().last().unwrap().into(),
        c.into_iter().last().unwrap().into(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

        let (g, e) = get_g_e(std_parse(input)?);
        assert_eq!((22, 9), (g.into(), e.into()));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

        assert_eq!((23, 10), get_o_c(std_parse(input)?));

        Ok(())
    }
}
