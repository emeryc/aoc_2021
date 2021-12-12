use std::fmt::Debug;

use aoc::{
    board::{Board, IntBoard, Point},
    helpers::read_input,
};
use eyre::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    println!("Part1: {}", part1::<10, 10>(input.parse()?));
    println!("Part2: {}", part2::<10, 10>(input.parse()?));

    Ok(())
}

fn part1<const X: usize, const Y: usize>(input: IntBoard<X, Y>) -> u64 {
    let mut p1 = Part1(input.0);
    let mut flashes = 0;
    for _r in 1..=100 {
        flashes += p1.round();
        //if _r < 10 || _r % 10 == 0 {
        //    println!("Round: {}\nBoard:\n{:?}\nEND", _r, p1);
        //}
    }
    flashes
}

fn part2<const X: usize, const Y: usize>(input: IntBoard<X, Y>) -> u64 {
    let mut p1 = Part1(input.0);
    let mut i = 0;
    loop {
        i += 1;
        let flashes = p1.round();
        if flashes == 100 {
            return i;
        }
        //if _r < 10 || _r % 10 == 0 {
        //    println!("Round: {}\nBoard:\n{:?}\nEND", _r, p1);
        //}
    }
}

#[derive(Clone, Copy)]
struct Part1<const X: usize, const Y: usize>(Board<u32, X, Y>);
impl<const X: usize, const Y: usize> Part1<X, Y> {
    fn round(&mut self) -> u64 {
        let mut flash = 0;
        for pt in self.0.points() {
            self.0.incr(&pt)
        }
        loop {
            let nines = self.all_flashes();
            if nines.is_empty() {
                self.cleanup();
                return flash;
            }
            // Mark them as being used up
            for nine in nines.iter() {
                flash += 1;
                self.0.set(nine, 100);
            }
            let mods = nines
                .iter()
                .flat_map(|pt| self.0.neighbors(pt).into_iter())
                .collect::<Vec<_>>();
            for m in mods {
                self.0.incr(&m);
            }
        }
    }

    fn cleanup(&mut self) {
        for pt in self.0.points() {
            if self.0.get(&pt) > &9 {
                self.0.set(&pt, 0);
            }
        }
    }

    fn all_flashes(&self) -> Vec<Point> {
        self.0
            .points()
            .into_iter()
            .filter(|pt| self.0.get(pt) > &9 && self.0.get(pt) < &100)
            .collect()
    }
}

impl<const X: usize, const Y: usize> Debug for Part1<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

trait Neighbors {
    fn neighbors(&self, pt: &Point) -> Vec<Point>;
}

impl<T, const X: usize, const Y: usize> Neighbors for Board<T, X, Y> {
    fn neighbors(&self, Point { x, y }: &Point) -> Vec<Point> {
        let p_x: isize = *x as isize;
        let p_y: isize = *y as isize;
        ((p_x - 1)..=(p_x + 1))
            .cartesian_product((p_y - 1)..=(p_y + 1))
            .filter_map(|(x, y)| {
                if x < 0 || x >= X as isize || y < 0 || y >= Y as isize || (p_x == x && p_y == y) {
                    None
                } else {
                    Some(Point {
                        x: x as usize,
                        y: y as usize,
                    })
                }
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526";

        assert_eq!(1656, part1::<10, 10>(input.parse()?));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526";

        assert_eq!(195, part2::<10, 10>(input.parse()?));

        Ok(())
    }
}
