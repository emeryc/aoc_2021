use std::str::FromStr;

use aoc::helpers::read_input;
use eyre::Result;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let t: Vec<usize> = csv_parse(input.trim())?;
    println!("Part 1: {}", simulate(t.as_slice(), 80));
    println!("Part 2: {}", simulate(t.as_slice(), 256));

    Ok(())
}

fn csv_parse<T>(input: &str) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
{
    input.split(',').map(|v| v.parse()).collect()
}

fn simulate(input: &[usize], generations: usize) -> u64 {
    let mut lifecycle = [0; 9];
    for i in input {
        lifecycle[*i] += 1;
    }

    for _ in 0..generations {
        lifecycle = generation(lifecycle);
    }
    return lifecycle.iter().sum();
}

fn generation(lifecycle: [u64; 9]) -> [u64; 9] {
    let mut new_lifecycle = [0; 9];
    for (i, x) in lifecycle.into_iter().enumerate() {
        if i == 0 {
            new_lifecycle[6] += x;
            new_lifecycle[8] += x;
        } else {
            new_lifecycle[i - 1] += x;
        }
    }
    new_lifecycle
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "3,4,3,1,2";

        assert_eq!(5934, simulate(csv_parse(input)?.as_slice(), 80));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "3,4,3,1,2";

        assert_eq!(26984457539, simulate(csv_parse(input)?.as_slice(), 256));

        Ok(())
    }
}
