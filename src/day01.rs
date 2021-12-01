use itertools::Itertools;

fn main() {
    let input = include_str!("../days/01.txt");
    println!(
        "increases: {}",
        increases(input.trim().split("\n").map(|s| s.trim()).collect())
    );
    println!(
        "window: {}",
        window(input.trim().split("\n").map(|s| s.trim()).collect())
    );
}

fn increases(input: Vec<&str>) -> usize {
    input
        .iter()
        .map(|i| i.parse::<i64>().unwrap())
        .fold((0, -1), |(cnt, prev), i| {
            (if i > prev { cnt + 1 } else { cnt }, i)
        })
        .0
        - 1
}

fn window(input: Vec<&str>) -> usize {
    input
        .iter()
        .map(|i| i.parse::<i64>().unwrap())
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
    fn t1() {
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

        assert_eq!(7, increases(input.split("\n").map(|v| v.trim()).collect()));
    }

    #[test]
    fn t2() {
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

        assert_eq!(5, window(input.split("\n").map(|v| v.trim()).collect()));
    }
}
