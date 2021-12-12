use eyre::{Error, Result};
use std::{fmt::Debug, str::FromStr};

#[derive(Clone, Copy)]
pub struct Board<T, const X: usize, const Y: usize>([[T; X]; Y]);
impl<T, const X: usize, const Y: usize> Board<T, X, Y> {
    pub fn new(board: [[T; X]; Y]) -> Board<T, X, Y> {
        Board(board)
    }
}
impl<T, const X: usize, const Y: usize> FromStr for Board<T, X, Y>
where
    T: FromStr + Clone + Send + Sync + std::fmt::Debug,
    T::Err: 'static + Send + Sync + std::error::Error,
{
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_ascii_whitespace()
                .map(|t| Ok(t.parse::<T>()?))
                .collect::<Result<Vec<_>>>()?
                .as_slice()
                .chunks_exact(X)
                .map(|chunk| -> Result<[T; X]> {
                    chunk
                        .iter()
                        .map(|v| -> T { (*v).clone() })
                        .collect::<Vec<T>>()
                        .try_into()
                        .map_err(|_v| {
                            Error::msg(format!("Couldn't make correctly sized X Board: {:?}", _v))
                        })
                })
                .collect::<Result<Vec<_>>>()?
                .try_into()
                .map_err(|_v| {
                    eyre::Error::msg(format!("Couldn't make a correctly sized Board: {:?}", _v))
                })?,
        ))
    }
}
impl<T, const X: usize, const Y: usize> Board<T, X, Y> {
    pub fn get(&self, p: &Point) -> &T {
        &self.0[p.y][p.x]
    }

    pub fn set(&mut self, p: &Point, v: T) {
        self.0[p.y][p.x] = v;
    }

    pub fn find(&self, v: T) -> Option<Point>
    where
        T: PartialEq,
    {
        for y in 0..Y {
            for x in 0..X {
                if self.0[y][x] == v {
                    return Some(Point { x, y });
                }
            }
        }
        None
    }

    pub fn row(&self, y: usize) -> Vec<&T> {
        self.0[y].iter().collect()
    }

    pub fn col(&self, x: usize) -> Vec<&T> {
        self.0.iter().map(|row| &row[x]).collect()
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        Box::new(self.0.iter().flat_map(|r| r.iter()))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {})", self.x, self.y).as_str())
    }
}
