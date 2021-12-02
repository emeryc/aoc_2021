use std::str::FromStr;

pub fn std_parse<T: FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
    Ok(input
        .lines()
        .map(|l| l.trim())
        .map(|i| i.parse::<T>())
        .collect::<Result<Vec<_>, _>>()?)
}
