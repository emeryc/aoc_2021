use std::collections::HashSet;

use aoc::{
    board::{Board, Point},
    helpers::read_input,
};
use eyre::{Report, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let board: Board<u32, 100, 100> = parse_board(input.as_str())?;

    println!("Part1: {}", risk_levels(&board).into_iter().sum::<u32>());
    println!(
        "Part2: {}",
        basins(&board)
            .into_iter()
            .sorted()
            .rev()
            .take(3)
            .product::<u32>()
    );

    Ok(())
}

fn risk_locations<const X: usize, const Y: usize>(board: &Board<u32, X, Y>) -> Vec<Point> {
    (0..Y)
        .cartesian_product(0..X)
        .map(|(y, x)| Point { x, y })
        .filter(|pt| {
            let v = board.get(pt);
            board.neighbors(pt).iter().all(|adj| v < board.get(adj))
        })
        .collect_vec()
}

fn risk_levels<const X: usize, const Y: usize>(board: &Board<u32, X, Y>) -> Vec<u32> {
    risk_locations(board)
        .into_iter()
        .map(|pt| *board.get(&pt) + 1)
        .collect()
}

fn basins<const X: usize, const Y: usize>(board: &Board<u32, X, Y>) -> Vec<u32> {
    risk_locations(board)
        .into_iter()
        .map(|pt| find_basin(&pt, board).len())
        .map(|u| u as u32)
        .collect()
}

fn find_basin<const X: usize, const Y: usize>(
    pt: &Point,
    board: &Board<u32, X, Y>,
) -> HashSet<Point> {
    let mut start: HashSet<Point> = [*pt].into_iter().collect();
    let mut next: HashSet<Point> = Default::default();
    while start != next {
        next = start.clone();
        for x in next.iter() {
            start.extend(
                board
                    .neighbors(x)
                    .into_iter()
                    .filter(|pt| board.get(pt) != &9),
            )
        }
    }
    start
}

trait Neighbors {
    fn neighbors(&self, pt: &Point) -> Vec<Point>;
}

impl<T, const X: usize, const Y: usize> Neighbors for Board<T, X, Y> {
    fn neighbors(&self, Point { x, y }: &Point) -> Vec<Point> {
        let x: isize = *x as isize;
        let y: isize = *y as isize;
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter_map(|(x, y)| {
                if x < 0 || x >= X as isize || y < 0 || y >= Y as isize {
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

fn parse_board<const X: usize, const Y: usize>(input: &str) -> Result<Board<u32, X, Y>> {
    Ok(Board::new(
        input
            .lines()
            .map(|line| -> Result<[u32; X]> {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect_vec()
                    .try_into()
                    .map_err(|_v| Report::msg("Collect Failure"))
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_v| Report::msg("Collect Failure"))?,
    ))
}

#[cfg(test)]
mod test {
    use aoc::board::Board;

    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let _input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        let board: Board<u32, 10, 5> = parse_board(_input)?;

        assert_eq!(15, risk_levels(&board).into_iter().sum::<u32>());

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let _input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        let board: Board<u32, 10, 5> = parse_board(_input)?;

        assert_eq!(3, find_basin(&Point { x: 1, y: 0 }, &board).len());

        assert_eq!(
            1134,
            basins(&board)
                .into_iter()
                .sorted()
                .rev()
                .take(3)
                .product::<u32>()
        );

        Ok(())
    }
}
