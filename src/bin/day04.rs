use aoc::{
    board::{Board, Point},
    helpers::{read_input, std_parse},
};
use eyre::Result;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let (plays, boards) = read_board_input(input.as_str())?;
    println!("Part1: {}", play_bingo_v1(plays.clone(), boards.clone()));
    println!("Part2: {}", play_bingo_v2(plays, boards));

    let _t: Vec<String> = std_parse(input)?;

    Ok(())
}

fn play_bingo_v1(plays: Vec<u32>, mut boards: Vec<Board<u32, 5, 5>>) -> u32 {
    for play in plays {
        for board in boards.iter_mut() {
            if let Some(pt) = board.find(play) {
                board.set(&pt, 0);
                if is_winner(board, &pt) {
                    return board.iter().sum::<u32>() * play;
                }
            }
        }
    }
    unreachable!("This should never happen")
}

fn is_winner<const X: usize, const Y: usize>(board: &Board<u32, X, Y>, pt: &Point) -> bool {
    (board.row(pt.y).into_iter().sum::<u32>() == 0u32)
        || (board.col(pt.x).into_iter().sum::<u32>() == 0)
}

fn play_bingo_v2(plays: Vec<u32>, mut boards: Vec<Board<u32, 5, 5>>) -> u32 {
    for play in plays {
        let mut tboards = Vec::new();
        let len = boards.len();
        for board in boards.iter_mut() {
            if let Some(pt) = board.find(play) {
                board.set(&pt, 0);
                if !is_winner(board, &pt) {
                    tboards.push(*board);
                } else if len == 1 {
                    return board.iter().sum::<u32>() * play;
                }
            } else {
                tboards.push(*board)
            }
        }
        boards = tboards;
    }
    unreachable!("This should never happen")
}

fn read_board_input(input: &str) -> Result<(Vec<u32>, Vec<Board<u32, 5, 5>>)> {
    let (plays, rest) = input
        .split_once("\n")
        .expect("Should be formatted correctly");

    let plays = plays
        .split(',')
        .map(|p| Ok(p.parse::<u32>()?))
        .collect::<Result<_>>()?;
    let boards = rest
        .split("\n\n")
        .map(|b| b.parse::<Board<u32, 5, 5>>())
        .collect::<Result<_>>()?;

    Ok((plays, boards))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7";

        let (plays, boards) = read_board_input(input)?;

        assert_eq!(4512, play_bingo_v1(plays, boards));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7";

        let (plays, boards) = read_board_input(input)?;

        assert_eq!(1924, play_bingo_v2(plays, boards));

        Ok(())
    }
}
