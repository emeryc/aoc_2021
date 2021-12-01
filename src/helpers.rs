use std::str::FromStr;

pub fn std_parse<T: FromStr>(input: &str) -> eyre::Result<Vec<T>>
where
    T::Err: 'static + std::error::Error + Send + Sync,
{
    Ok(input
        .trim()
        .split("\n")
        .map(|s| s.trim())
        .map(|i| i.parse::<T>())
        .collect::<Result<Vec<_>, _>>()?)
}
