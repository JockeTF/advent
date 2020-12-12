use self::types::*;
use std::convert::TryFrom;
use std::str::FromStr;

mod types;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Clone, Debug)]
pub struct ParserError(&'static str);

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Clone, Debug, Default)]
struct OptPassport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl OptPassport {
    fn split_entry(entry: &str) -> ParserResult<(&str, &str)> {
        let split = entry.splitn(2, ':').collect::<Vec<_>>();

        match split[..] {
            [key, value] => Ok((key, value)),
            _ => Err(ParserError("Invalid entry")),
        }
    }
}

impl FromStr for OptPassport {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let mut builder = OptPassport::default();

        let pairs = s
            .split_whitespace()
            .map(|text| text.trim())
            .filter(|text| !text.is_empty())
            .map(OptPassport::split_entry)
            .collect::<ParserResult<Vec<_>>>()?;

        for (key, value) in pairs {
            match key {
                "byr" => builder.byr = Some(value.into()),
                "iyr" => builder.iyr = Some(value.into()),
                "eyr" => builder.eyr = Some(value.into()),
                "hgt" => builder.hgt = Some(value.into()),
                "hcl" => builder.hcl = Some(value.into()),
                "ecl" => builder.ecl = Some(value.into()),
                "pid" => builder.pid = Some(value.into()),
                "cid" => builder.cid = Some(value.into()),
                _ => return Err(ParserError("Invalid key")),
            }
        }

        Ok(builder)
    }
}

#[derive(Clone, Debug)]
struct RawPassport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl TryFrom<OptPassport> for RawPassport {
    type Error = ParserError;

    fn try_from(value: OptPassport) -> ParserResult<Self> {
        Ok(RawPassport {
            byr: value.byr.ok_or_else(|| ParserError("Missing byr value"))?,
            iyr: value.iyr.ok_or_else(|| ParserError("Missing iyr value"))?,
            eyr: value.eyr.ok_or_else(|| ParserError("Missing eyr value"))?,
            hgt: value.hgt.ok_or_else(|| ParserError("Missing hgt value"))?,
            hcl: value.hcl.ok_or_else(|| ParserError("Missing hcl value"))?,
            ecl: value.ecl.ok_or_else(|| ParserError("Missing ecl value"))?,
            pid: value.pid.ok_or_else(|| ParserError("Missing pid value"))?,
            cid: value.cid,
        })
    }
}

#[derive(Clone, Debug)]
struct Passport {
    byr: BirthYear,
    iyr: IssueYear,
    eyr: ExpirationYear,
    hgt: Height,
    hcl: HairColor,
    ecl: EyeColor,
    pid: PassportId,
    cid: Option<CountryId>,
}

impl TryFrom<RawPassport> for Passport {
    type Error = ParserError;

    fn try_from(value: RawPassport) -> ParserResult<Self> {
        let country = match value.cid {
            Some(text) => Some(text.parse()?),
            None => None,
        };

        Ok(Passport {
            byr: value.byr.parse()?,
            iyr: value.iyr.parse()?,
            eyr: value.eyr.parse()?,
            hgt: value.hgt.parse()?,
            hcl: value.hcl.parse()?,
            ecl: value.ecl.parse()?,
            pid: value.pid.parse()?,
            cid: country,
        })
    }
}

fn main() {
    let items = INPUT
        .split("\n\n")
        .map(|text| text.trim())
        .filter(|text| !text.is_empty())
        .collect::<Vec<&str>>();

    let opts = items
        .into_iter()
        .map(OptPassport::from_str)
        .collect::<ParserResult<Vec<_>>>()
        .expect("Invalid passport item");

    println!("Opt passports: {}", opts.len());

    let raws = opts
        .into_iter()
        .map(RawPassport::try_from)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    println!("Raw passports: {}", raws.len());

    let valid = raws
        .into_iter()
        .map(Passport::try_from)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    println!("Valid Passports: {}", valid.len());
}
