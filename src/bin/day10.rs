use core::panic;
use std::str::FromStr;

use aoc::helpers::{read_input, std_parse};
use eyre::{Error, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    println!("Part1: {}", part1(std_parse(input.as_str())?));
    println!("Part2: {}", part2(std_parse(input)?));

    Ok(())
}

struct Line(Vec<char>);
impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line(s.chars().collect()))
    }
}

fn get_opposite(c: char) -> char {
    match c {
        '{' => '}',
        '(' => ')',
        '<' => '>',
        '[' => ']',
        '}' => '{',
        ')' => '(',
        '>' => '<',
        ']' => '[',
        _ => panic!("no mattching char"),
    }
}

impl Line {
    fn corrupt_score(&self) -> u64 {
        let (_, corrupted) = self.run_stack();
        match corrupted {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            None => 0,
            _ => panic!("Something went wrong"),
        }
    }

    fn run_stack(&self) -> (Vec<char>, Option<char>) {
        let mut stack = Vec::new();
        for brace in self.0.iter() {
            match brace {
                '{' | '(' | '<' | '[' => {
                    stack.push(*brace);
                }
                '}' | ')' | '>' | ']' => {
                    if stack.last() == Some(&get_opposite(*brace)) {
                        stack.pop();
                    } else {
                        return (stack, Some(*brace));
                    }
                }
                _ => unreachable!("Only valid characters"),
            }
        }
        (stack, None)
    }

    fn build_close(&self) -> Vec<char> {
        let (stack, corrupted) = self.run_stack();
        if corrupted.is_some() {
            return Vec::new();
        }

        stack.into_iter().rev().map(get_opposite).collect()
    }

    fn score_close(&self) -> u64 {
        self.build_close().into_iter().fold(0, |total, close| {
            (match close {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!("Something went wrong"),
            }) + (total * 5)
        })
    }
}

fn part1(input: Vec<Line>) -> u64 {
    input.into_iter().map(|line| line.corrupt_score()).sum()
}

fn part2(input: Vec<Line>) -> u64 {
    let scores = input
        .into_iter()
        .map(|line| line.score_close())
        .filter(|s| s != &0)
        .sorted_unstable()
        .collect_vec();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(26397, part1(std_parse(input)?));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(288957, part2(std_parse(input)?));

        Ok(())
    }
}
