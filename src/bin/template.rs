use aoc::helpers::{read_input, std_parse};
use eyre::Result;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let _t: Vec<String> = std_parse(input)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let _input = "";

        //assert_eq!(7, std_parse(input)?);

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let _input = "";

        //assert_eq!(5, window(std_parse(input)?));

        Ok(())
    }
}
