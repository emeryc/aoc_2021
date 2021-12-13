use std::{collections::HashMap, str::FromStr};

use aoc::helpers::{read_input, std_parse};
use eyre::{Error, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(file!())?;
    println!("Part1: {}", part1(std_parse(input.as_str())?));
    println!("Part2: {}", part2(std_parse(input)?));

    Ok(())
}

fn part1(input: Vec<Pair>) -> usize {
    all_paths::<Path1>(input.as_slice()).len()
}

fn part2(input: Vec<Pair>) -> usize {
    all_paths::<Path2>(input.as_slice()).len()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Room {
    Big(String),
    Small(String),
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == s.to_lowercase() {
            Room::Small(s.to_string())
        } else {
            Room::Big(s.to_string())
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pair {
    start: Room,
    end: Room,
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .trim()
            .split_once('-')
            .ok_or_else(|| Error::msg("Unable to split"))?;
        Ok(Pair {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

trait Pathing {
    fn new(p: Vec<Room>) -> Self;
    fn last(&self) -> &Room;
    fn rooms(&self) -> Vec<Room>;
    fn extend(&self, rooms: &[Room]) -> (Vec<Self>, Vec<Self>)
    where
        Self: std::marker::Sized,
    {
        rooms
            .iter()
            .map(|room| {
                let mut npath = self.rooms();
                npath.push(room.clone());
                Self::new(npath)
            })
            .filter(|path| path.is_valid())
            .partition(|path| path.last() == &Room::Small("end".to_string()))
    }
    fn is_valid(&self) -> bool;
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Path1(Vec<Room>);
impl Pathing for Path1 {
    fn new(p: Vec<Room>) -> Self {
        Path1(p)
    }

    fn last(&self) -> &Room {
        self.0.last().unwrap()
    }

    fn extend(&self, rooms: &[Room]) -> (Vec<Path1>, Vec<Path1>) {
        rooms
            .iter()
            .map(|room| {
                let mut npath = self.0.clone();
                npath.push(room.clone());
                Path1(npath)
            })
            .filter(|path| path.is_valid())
            .partition(|path| path.last() == &Room::Small("end".to_string()))
    }

    fn is_valid(&self) -> bool {
        self.0.iter().sorted().tuple_windows().all(|(a, b)| {
            if let (Room::Small(a), Room::Small(b)) = (a, b) {
                a != b
            } else {
                true
            }
        })
    }

    fn rooms(&self) -> Vec<Room> {
        self.0.clone()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Path2(Vec<Room>);
impl Pathing for Path2 {
    fn new(p: Vec<Room>) -> Self {
        Path2(p)
    }
    fn rooms(&self) -> Vec<Room> {
        self.0.clone()
    }
    fn last(&self) -> &Room {
        self.0.last().unwrap()
    }

    fn is_valid(&self) -> bool {
        let mut seen = false;
        self.0.iter().sorted().tuple_windows().all(|(a, b)| {
            if let (Room::Small(a), Room::Small(b)) = (a, b) {
                if a == b && (a == "end" || a == "start" || seen) {
                    false
                } else if a == b {
                    seen = true;
                    true
                } else {
                    true
                }
            } else {
                true
            }
        })
    }
}

fn all_paths<T: Pathing>(room_list: &[Pair]) -> Vec<T> {
    let connections = connected(room_list);
    let mut queue: Vec<T> = vec![T::new(vec![Room::Small("start".to_string())])];
    let mut done: Vec<T> = Default::default();
    while !queue.is_empty() {
        let mut next_queue: Vec<T> = Default::default();
        for path in queue.iter() {
            let next = connections.get(path.last()).unwrap();
            let (done_paths, not_done) = path.extend(next);
            done.extend(done_paths);
            next_queue.extend(not_done);
        }
        queue = next_queue
    }
    done
}

fn connected(room_list: &[Pair]) -> HashMap<Room, Vec<Room>> {
    let mut map: HashMap<Room, Vec<Room>> = Default::default();
    for x in room_list {
        map.entry(x.start.clone()).or_default().push(x.end.clone());
        map.entry(x.end.clone()).or_default().push(x.start.clone());
    }
    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() -> Result<()> {
        let input1 = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";

        assert_eq!(10, part1(std_parse(input1)?));

        let input2 = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";

        assert_eq!(19, part1(std_parse(input2)?));

        let input3 = "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW";

        assert_eq!(226, part1(std_parse(input3)?));

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let input1 = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";

        assert_eq!(36, part2(std_parse(input1)?));

        let input2 = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";

        assert_eq!(103, part2(std_parse(input2)?));

        let input3 = "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW";

        assert_eq!(3509, part2(std_parse(input3)?));

        Ok(())
    }
}
