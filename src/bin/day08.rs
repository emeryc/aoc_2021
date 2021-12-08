use std::{collections::HashSet, str::FromStr};

use aoc::helpers::{read_input, std_parse};
use eyre::{Error, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let input: Vec<Display> = std_parse(input)?;
    println!("Part1: {}", part1(input.as_slice()));
    println!("Part2: {}", part2(input.as_slice()));

    Ok(())
}

fn part1(input: &[Display]) -> usize {
    input
        .iter()
        .map(|display| {
            display
                .output
                .iter()
                .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                .count()
        })
        .sum()
}

fn part2(input: &[Display]) -> usize {
    input.iter().map(|d| solve_display(d)).sum()
}

fn solve_display(display: &Display) -> usize {
    let one = display
        .patterns
        .iter()
        .filter(|s| s.len() == 2)
        .last()
        .unwrap();
    let seven = display
        .patterns
        .iter()
        .filter(|s| s.len() == 3)
        .last()
        .unwrap();
    let four = display
        .patterns
        .iter()
        .filter(|s| s.len() == 4)
        .last()
        .unwrap();
    let eight = display
        .patterns
        .iter()
        .filter(|s| s.len() == 7)
        .last()
        .unwrap();
    let nine = display
        .patterns
        .iter()
        .find(|s| s.len() == 6 && s.is_superset(four))
        .unwrap();
    let six = display
        .patterns
        .iter()
        .find(|s| s.len() == 6 && s.intersection(seven).count() == 2)
        .unwrap();
    let zero = display
        .patterns
        .iter()
        .filter(|s| s.len() == 6 && ![six, nine].contains(s))
        .last()
        .unwrap();

    let five = display
        .patterns
        .iter()
        .find(|s| s.len() == 5 && s.is_subset(six))
        .unwrap();
    let three = display
        .patterns
        .iter()
        .find(|s| s.len() == 5 && s.is_subset(nine) && !s.is_subset(six))
        .unwrap();
    let two = display
        .patterns
        .iter()
        .find(|s| {
            s.len() == 5 && ![zero, one, three, four, five, six, seven, eight, nine].contains(s)
        })
        .unwrap();

    display
        .output
        .iter()
        .map(|d| {
            if d == zero {
                0
            } else if d == one {
                1
            } else if d == two {
                2
            } else if d == three {
                3
            } else if d == four {
                4
            } else if d == five {
                5
            } else if d == six {
                6
            } else if d == seven {
                7
            } else if d == eight {
                8
            } else if d == nine {
                9
            } else {
                println!("D: {:?}", d);
                println!(
                    "{:?}",
                    [zero, one, two, three, four, five, six, seven, eight, nine]
                );
                panic!("???")
            }
        })
        .join("")
        .parse()
        .unwrap()
}

struct Display {
    patterns: [HashSet<char>; 10],
    output: [HashSet<char>; 4],
}

impl FromStr for Display {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, o) = s.split_once('|').expect("should have a pipe");
        Ok(Display {
            patterns: p
                .split_ascii_whitespace()
                .map(|s| s.chars().collect())
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|v| Error::msg(format!("bad vec pattern: {:?}", v)))?,
            output: o
                .split_ascii_whitespace()
                .map(|s| s.chars().collect())
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|v| Error::msg(format!("bad vec output: {:?}", v)))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let input: Vec<Display> = std_parse(input)?;
        assert_eq!(26, part1(input.as_slice()));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let input: Vec<Display> = std_parse(input)?;
        assert_eq!(61229, part2(input.as_slice()));

        Ok(())
    }
}
