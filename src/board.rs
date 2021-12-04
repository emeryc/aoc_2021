use eyre::Result;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Board<T, const L: usize>([T; L]);
impl<T, const L: usize> FromStr for Board<T, L>
where
    T: FromStr,
    T::Err: 'static + Send + Sync + std::error::Error,
{
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_ascii_whitespace()
                .map(|t| Ok(t.parse::<T>()?))
                .collect::<Result<Vec<_>>>()?
                .try_into()
                .map_err(|_v| eyre::Error::msg("Couldn't make a correctly sized Board"))?,
        ))
    }
}
impl<T, const L: usize> Board<T, L> {
    pub fn get(&self, p: &Point) -> &T {
        &self.0[p.x + p.y * ((L as f32).sqrt() as usize)]
    }

    pub fn set(&mut self, p: &Point, v: T) {
        self.0[p.x + p.y * ((L as f32).sqrt() as usize)] = v;
    }

    pub fn find(&self, v: T) -> Option<Point>
    where
        T: PartialEq,
    {
        let len = (L as f32).sqrt() as usize;
        self.0
            .iter()
            .enumerate()
            .find(|(_, i)| **i == v)
            .map(|(l, _)| Point {
                x: l % len,
                y: l / len,
            })
    }

    pub fn row(&self, y: usize) -> Vec<&T> {
        let len = (L as f32).sqrt() as usize;
        let mut v = Vec::new();
        for r in (y * len)..(y * len + len) {
            v.push(&self.0[r])
        }
        v
    }

    pub fn col(&self, x: usize) -> Vec<&T> {
        let len = (L as f32).sqrt() as usize;
        let mut v = Vec::new();
        for c in 0..len {
            v.push(&self.0[x + c * len])
        }
        v
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }
}
pub struct Point {
    pub x: usize,
    pub y: usize,
}
