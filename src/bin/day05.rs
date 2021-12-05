use std::str::FromStr;

use aoc::helpers::{read_input, std_parse};
use eyre::{Report, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let p1 = part1(std_parse(input.as_str())?);
    println!("Part1: {}", p1);
    let p2 = part2(std_parse(input.as_str())?);
    println!("Part2: {}", p2);

    Ok(())
}

fn part1(lines: Vec<LineSegment>) -> usize {
    lines
        .iter()
        .flat_map(|line| line.covered_v1().into_iter())
        .map(|point| (point, 1))
        .into_group_map()
        .into_iter()
        .filter(|(_pt, v)| v.len() >= 2)
        .count()
}

fn part2(lines: Vec<LineSegment>) -> usize {
    lines
        .iter()
        .flat_map(|line| line.covered_v2().into_iter())
        .map(|point| (point, 1))
        .into_group_map()
        .into_iter()
        .filter(|(_pt, v)| v.len() >= 2)
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LineSegment(Point, Point);
impl FromStr for LineSegment {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once(" -> ")
            .ok_or_else(|| Report::msg(format!("Poorly constructed line segment: {}", s)))?;

        Ok(LineSegment(start.parse()?, end.parse()?))
    }
}
impl LineSegment {
    fn covered_v1(&self) -> Vec<Point> {
        if self.0.x == self.1.x {
            let (min, max) = (self.0.y.min(self.1.y), self.0.y.max(self.1.y));
            (min..=max).map(|y| Point { x: self.0.x, y }).collect()
        } else if self.0.y == self.1.y {
            let (min, max) = (self.0.x.min(self.1.x), self.0.x.max(self.1.x));
            (min..=max).map(|x| Point { x, y: self.0.y }).collect()
        } else {
            Vec::new()
        }
    }

    fn covered_v2(&self) -> Vec<Point> {
        if self.0.x == self.1.x {
            let (min, max) = (self.0.y.min(self.1.y), self.0.y.max(self.1.y));
            (min..=max).map(|y| Point { x: self.0.x, y }).collect()
        } else if self.0.y == self.1.y {
            let (min, max) = (self.0.x.min(self.1.x), self.0.x.max(self.1.x));
            (min..=max).map(|x| Point { x, y: self.0.y }).collect()
        } else {
            let (miny, maxy) = (self.0.y.min(self.1.y), self.0.y.max(self.1.y));
            let (minx, maxx) = (self.0.x.min(self.1.x), self.0.x.max(self.1.x));

            let mut y_range = (miny..=maxy).collect::<Vec<_>>();
            if miny == self.1.y {
                y_range = y_range.into_iter().rev().collect();
            }

            let mut x_range = (minx..=maxx).collect::<Vec<_>>();
            if minx == self.1.x {
                x_range = x_range.into_iter().rev().collect();
            }

            x_range
                .into_iter()
                .zip(y_range.into_iter())
                .map(|(x, y)| Point { x, y })
                .collect()
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}
impl FromStr for Point {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(",")
            .ok_or_else(|| Report::msg(format!("Poorly constructed: {}", s)))?;

        Ok(Point {
            x: x.trim().parse()?,
            y: y.trim().parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

        assert_eq!(5, part1(std_parse(input)?));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

        assert_eq!(12, part2(std_parse(input)?));

        Ok(())
    }
}
