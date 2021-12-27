use std::{collections::HashMap, str::FromStr};

use aoc::helpers::read_input;
use eyre::{Error, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    let (template, rules) = input.split_once("\n\n").unwrap();
    let rules: Vec<Production> = rules
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>>>()?;
    let rule_map = rules
        .into_iter()
        .map(|rule| (rule.input, rule.out))
        .collect::<HashMap<_, _>>();

    println!("Part1: {}", part1(template, &rule_map));
    println!("Part2: {}", part2(template, &rule_map));

    Ok(())
}

fn part1(template: &str, rules: &HashMap<(char, char), char>) -> u128 {
    let mut counts: HashMap<char, u128> = template
        .chars()
        .counts()
        .into_iter()
        .map(|(a, b)| (a, b as u128))
        .collect();
    let mut pair_counts: HashMap<(char, char), u128> = Default::default();
    for (a, b) in template.chars().tuple_windows() {
        *pair_counts.entry((a, b)).or_default() += 1;
    }
    for _r in 0..10 {
        let mut new = HashMap::new();
        for ((a, b), score) in pair_counts {
            let rule = rules[&(a, b)];
            *counts.entry(rule).or_default() += score;
            *new.entry((a, rule)).or_default() += score;
            *new.entry((rule, b)).or_default() += score;
        }
        pair_counts = new;
    }
    println!("Counts: {:?}", counts);
    match counts.iter().minmax_by_key(|(_, s)| **s) {
        itertools::MinMaxResult::NoElements => todo!(),
        itertools::MinMaxResult::OneElement(_) => todo!(),
        itertools::MinMaxResult::MinMax((_, min), (_, max)) => max - min,
    }
}

fn part2(template: &str, rules: &HashMap<(char, char), char>) -> u128 {
    let mut counts: HashMap<char, u128> = template
        .chars()
        .counts()
        .into_iter()
        .map(|(a, b)| (a, b as u128))
        .collect();
    let mut pair_counts: HashMap<(char, char), u128> = Default::default();
    for (a, b) in template.chars().tuple_windows() {
        *pair_counts.entry((a, b)).or_default() += 1;
    }
    for _r in 0..40 {
        let mut new = HashMap::new();
        for ((a, b), score) in pair_counts {
            let rule = rules[&(a, b)];
            *counts.entry(rule).or_default() += score;
            *new.entry((a, rule)).or_default() += score;
            *new.entry((rule, b)).or_default() += score;
        }
        pair_counts = new;
    }
    println!("Counts: {:?}", counts);
    match counts.iter().minmax_by_key(|(_, s)| **s) {
        itertools::MinMaxResult::NoElements => todo!(),
        itertools::MinMaxResult::OneElement(_) => todo!(),
        itertools::MinMaxResult::MinMax((_, min), (_, max)) => max - min,
    }
}

struct Production {
    input: (char, char),
    out: char,
}

impl FromStr for Production {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, out) = s
            .split_once(" -> ")
            .ok_or_else(|| Error::msg("no -> found"))?;

        Ok(Production {
            input: input.chars().collect_tuple().unwrap(),
            out: out.chars().last().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let (template, rules) = input.split_once("\n\n").unwrap();
        let rules: Vec<Production> = rules
            .lines()
            .map(|line| line.trim().parse())
            .collect::<Result<Vec<_>>>()?;
        let rule_map = rules
            .into_iter()
            .map(|rule| (rule.input, rule.out))
            .collect::<HashMap<_, _>>();
        assert_eq!(1588, part1(template, &rule_map));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let _input = "";

        //assert_eq!(5, window(std_parse(input)?));

        Ok(())
    }
}
