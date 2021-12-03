use eyre::{Error, Result};
use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

pub fn std_parse<R: AsRef<str>, T: FromStr>(input: R) -> Result<Vec<T>, T::Err> {
    input
        .as_ref()
        .lines()
        .map(|l| l.trim())
        .map(|i| i.parse::<T>())
        .collect::<Result<Vec<_>, _>>()
}

pub fn read_input(filename: &str) -> Result<String> {
    let file = File::open(format!(
        "days/{}.txt",
        filename
            .rsplit_once('/')
            .ok_or_else(|| Error::msg("Filename weirdness?"))?
            .1
            .split_once('.')
            .ok_or_else(|| Error::msg("Filename weirdness?"))?
            .0
    ))?;
    let mut buf = Vec::new();
    BufReader::new(file).read_to_end(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}
