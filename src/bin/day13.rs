use std::str::FromStr;

use aoc::{
    board::Point,
    helpers::{read_input, std_parse},
};
use eyre::{Error, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let (positions, instructions) = input.split_once("\n\n").expect("Two parts");
    let positions: Vec<Point> = std_parse(positions)?;
    let instructions: Vec<Instruction> = std_parse(instructions)?;
    println!(
        "Part1: {}",
        fold(positions.as_slice(), instructions[0]).len()
    );
    println!(
        "Part2:\n{}",
        to_str(fold_all(positions.as_slice(), instructions.as_slice()).as_slice())
    );

    Ok(())
}

fn to_str(points: &[Point]) -> String {
    let x = points.iter().max_by_key(|pt| pt.x).unwrap().x;
    let y = points.iter().max_by_key(|pt| pt.y).unwrap().y;

    let mut map = vec![vec!["."; x + 1]; y + 1];
    for pt in points {
        map[pt.y][pt.x] = "#";
    }
    map.into_iter().map(|v| v.join("")).join("\n")
}

fn fold_all(positions: &[Point], instruction: &[Instruction]) -> Vec<Point> {
    instruction
        .iter()
        .fold(positions.to_vec(), |p, f| fold(p.as_slice(), *f))
}

fn fold(positions: &[Point], instruction: Instruction) -> Vec<Point> {
    positions
        .iter()
        .map(|pos| {
            if instruction.orientation == Orientation::Y && instruction.pos < pos.y {
                Point {
                    x: pos.x,
                    y: (2 * instruction.pos) - pos.y,
                }
            } else if instruction.orientation == Orientation::X && instruction.pos < pos.x {
                Point {
                    x: (2 * instruction.pos) - pos.x,
                    y: pos.y,
                }
            } else {
                *pos
            }
        })
        .unique()
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Orientation {
    X,
    Y,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Instruction {
    orientation: Orientation,
    pos: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, i) = s
            .trim()
            .rsplit_once(' ')
            .ok_or_else(|| Error::msg("No space?"))?;
        let (o, p) = i.split_once('=').ok_or_else(|| Error::msg("No Eq"))?;
        Ok(Instruction {
            orientation: match o {
                "y" => Orientation::Y,
                "x" => Orientation::X,
                _ => return Err(Error::msg("Not x, or y")),
            },
            pos: p.parse()?,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5";

        let (positions, instructions) = input.split_once("\n\n").expect("Two parts");
        let positions: Vec<Point> = std_parse(positions)?;
        let instructions: Vec<Instruction> = std_parse(instructions)?;
        assert_eq!(17, fold(positions.as_slice(), instructions[0]).len());

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5";

        let (positions, instructions) = input.split_once("\n\n").expect("Two parts");
        let positions: Vec<Point> = std_parse(positions)?;
        let instructions: Vec<Instruction> = std_parse(instructions)?;
        assert_eq!(
            16,
            fold_all(positions.as_slice(), instructions.as_slice()).len()
        );

        Ok(())
    }
}
