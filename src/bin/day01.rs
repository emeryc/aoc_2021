use aoc::helpers::std_parse;
use eyre::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../days/01.txt");
    println!("increases: {}", increases(std_parse(input)?));
    println!("window: {}", window(std_parse(input)?));

    Ok(())
}

fn increases(input: Vec<i64>) -> usize {
    input
        .into_iter()
        .fold((0, -1), |(cnt, prev), i| {
            (if i > prev { cnt + 1 } else { cnt }, i)
        })
        .0
        - 1
}

fn window(input: Vec<i64>) -> usize {
    input
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .fold((0, -1), |(cnt, prev), i| {
            (if i > prev { cnt + 1 } else { cnt }, i)
        })
        .0
        - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        assert_eq!(7, increases(std_parse(input)?));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        assert_eq!(5, window(std_parse(input)?));

        Ok(())
    }
}
