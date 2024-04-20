use std::collections::HashMap;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[cfg(test)]
mod tests;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct BagError(&'static str);

type BagResult<T> = Result<T, BagError>;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct BagType(String, String);

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct BagRule(BagType, u32);

#[derive(Clone, Debug)]
struct RulePair(BagType, Vec<BagRule>);

#[derive(Clone, Debug)]
struct RuleBook(HashMap<BagType, Vec<BagRule>>);

impl From<ParseIntError> for BagError {
    fn from(_: ParseIntError) -> Self {
        BagError("Invalid number format")
    }
}

impl FromStr for BagType {
    type Err = BagError;

    fn from_str(s: &str) -> BagResult<Self> {
        let split = s.split_whitespace().collect::<Vec<&str>>();

        let [modifier, color, suffix] = match split[..] {
            [m, c, s] => Ok([m.trim(), c.trim(), s.trim()]),
            _ => Err(BagError("Invalid type syntax")),
        }?;

        if suffix == "bag" || suffix == "bags" {
            Ok(BagType(color.into(), modifier.into()))
        } else {
            Err(BagError("Invalid type suffix"))
        }
    }
}

impl FromStr for BagRule {
    type Err = BagError;

    fn from_str(s: &str) -> BagResult<Self> {
        let split = s.trim().splitn(2, ' ').collect::<Vec<&str>>();

        let [num, typ] = match split[..] {
            [n, t] => Ok([n.trim(), t.trim()]),
            _ => Err(BagError("Invalid rule syntax")),
        }?;

        Ok(BagRule(typ.parse()?, num.parse()?))
    }
}

impl FromStr for RulePair {
    type Err = BagError;

    fn from_str(s: &str) -> BagResult<Self> {
        let split = s
            .trim()
            .strip_suffix('.')
            .ok_or(BagError("Missing pair suffix"))?
            .split("contain")
            .collect::<Vec<&str>>();

        let [key, values] = match split[..] {
            [k, v] => Ok([k.trim(), v.trim()]),
            _ => Err(BagError("Invalid pair syntax")),
        }?;

        let key = key.parse()?;

        if values == "no other bags" {
            return Ok(RulePair(key, vec![]));
        }

        let rules = values
            .split(',')
            .map(BagRule::from_str)
            .collect::<BagResult<_>>()?;

        Ok(RulePair(key, rules))
    }
}

impl FromStr for RuleBook {
    type Err = BagError;

    fn from_str(s: &str) -> BagResult<Self> {
        let pairs = s
            .trim()
            .lines()
            .map(RulePair::from_str)
            .collect::<BagResult<Vec<_>>>()?;

        let count = pairs.len();

        let map = pairs
            .into_iter()
            .map(|pair| (pair.0, pair.1))
            .collect::<HashMap<_, _>>();

        let complete = map
            .values()
            .flatten()
            .map(|val| map.contains_key(&val.0))
            .all(|elem| elem);

        if !complete {
            Err(BagError("Found undefined keys"))
        } else if count != map.len() {
            Err(BagError("Found duplicate keys"))
        } else {
            Ok(RuleBook(map))
        }
    }
}

impl RuleBook {
    fn invert(&self) -> HashMap<&BagType, HashSet<&BagType>> {
        let keys = self.0.keys();

        let mut result = keys
            .map(|key| (key, HashSet::new()))
            .collect::<HashMap<_, HashSet<_>>>();

        for (key, values) in self.0.iter() {
            for value in values.iter() {
                result
                    .get_mut(&value.0)
                    .expect("Corrupt rule book")
                    .insert(key);
            }
        }

        result
    }

    fn solve(&self, bag: &BagType) -> BagResult<HashSet<BagType>> {
        let mut result = HashSet::new();
        let mut remain = vec![bag];

        let inverted = self.invert();

        while let Some(bag) = remain.pop() {
            let iter = match inverted.get(bag) {
                Some(list) => Ok(list.iter()),
                None => Err(BagError("Unknown bag type")),
            }?;

            remain.extend(iter.filter(|bag| result.insert(**bag)));
        }

        Ok(result.into_iter().cloned().collect())
    }

    fn count(&self, bag: &BagType) -> BagResult<u32> {
        let mut iter = match self.0.get(bag) {
            Some(item) => Ok(item.iter()),
            None => Err(BagError("Unknown bag type")),
        }?;

        iter.try_fold(1, |acc, elem| Ok(acc + self.count(&elem.0)? * elem.1))
    }
}

fn main() {
    let target = BagType("gold".into(), "shiny".into());

    let book: RuleBook = INPUT.parse().expect("Invalid input");
    println!("Number of rules: {}", book.0.len());

    let solve = book.solve(&target).expect("Invalid solve result");
    println!("Contains shiny gold: {}", solve.len());

    let count = book.count(&target).expect("Invalid count result");
    println!("Shiny gold contains: {}", count - 1);
}
