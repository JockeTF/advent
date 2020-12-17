use std::str::FromStr;

#[cfg(test)]
mod tests;

const INPUT: &'static str = include_str!("input.txt");
const BOUND: u32 = 'a' as u32;

#[derive(Clone, Debug)]
struct ParserError(&'static str);

type ParserResult<T> = Result<T, ParserError>;

#[derive(Clone, Debug)]
struct Sheet(u32);

impl Sheet {
    fn bitmask(chr: char) -> ParserResult<u32> {
        let ord = match chr {
            chr @ 'a'..='z' => Ok(u32::from(chr)),
            _ => Err(ParserError("Invalid character")),
        }?;

        Ok(1 << (ord - BOUND))
    }
}

impl FromStr for Sheet {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let pattern = s
            .trim()
            .chars()
            .map(Sheet::bitmask)
            .try_fold(0, |acc, elem| Ok(acc | elem?))?;

        Ok(Sheet(pattern))
    }
}

#[derive(Clone, Debug)]
struct Group(Vec<Sheet>);

impl Group {
    fn count(bits: u32) -> u32 {
        (0..32).map(|i| (bits >> i) & 1).filter(|n| *n == 1).count() as u32
    }

    fn union(&self) -> u32 {
        self.0.iter().fold(u32::MIN, |acc, elem| acc | elem.0)
    }

    fn intersection(&self) -> u32 {
        self.0.iter().fold(u32::MAX, |acc, elem| acc & elem.0)
    }
}

impl FromStr for Group {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let sheets = s
            .trim()
            .lines()
            .map(Sheet::from_str)
            .collect::<ParserResult<_>>()?;

        Ok(Group(sheets))
    }
}

fn main() {
    let groups = INPUT
        .trim()
        .split("\n\n")
        .map(Group::from_str)
        .collect::<ParserResult<Vec<_>>>()
        .expect("Found invalid group");

    println!("Group count: {}", groups.len());

    let union = groups
        .iter()
        .map(Group::union)
        .map(Group::count)
        .sum::<u32>();

    println!("Group union sum: {}", union);

    let intersection = groups
        .iter()
        .map(Group::intersection)
        .map(Group::count)
        .sum::<u32>();

    println!("Group intersection sum: {}", intersection);
}
