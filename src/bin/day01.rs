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
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

fn window(input: Vec<i64>) -> usize {
    increases(
        input
            .into_iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect(),
    )
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
