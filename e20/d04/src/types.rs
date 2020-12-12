use super::ParserError;
use super::ParserResult;
use std::num::ParseIntError;
use std::str::FromStr;

impl From<ParseIntError> for ParserError {
    fn from(_: ParseIntError) -> Self {
        ParserError("Value is not a number")
    }
}

#[derive(Clone, Debug)]
pub struct BirthYear(u16);

impl FromStr for BirthYear {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let value = s.parse()?;

        if 1920 <= value && value <= 2002 {
            Ok(BirthYear(value))
        } else {
            Err(ParserError("Invalid byr range"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct IssueYear(u16);

impl FromStr for IssueYear {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let value = s.parse()?;

        if 2010 <= value && value <= 2020 {
            Ok(IssueYear(value))
        } else {
            Err(ParserError("Invalid iyr range"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExpirationYear(u16);

impl FromStr for ExpirationYear {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let value = s.parse()?;

        if 2020 <= value && value <= 2030 {
            Ok(ExpirationYear(value))
        } else {
            Err(ParserError("Invalid eyr range"))
        }
    }
}

#[derive(Clone, Debug)]
pub enum Height {
    Cm(u16),
    In(u16),
}

impl FromStr for Height {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        use Height::*;

        let height = if s.ends_with("cm") {
            Cm(s[..s.len() - 2].parse::<u16>()?)
        } else if s.ends_with("in") {
            In(s[..s.len() - 2].parse::<u16>()?)
        } else {
            return Err(ParserError("Invalid hgt format"));
        };

        match height {
            Cm(n) if 150 <= n && n <= 193 => Ok(Cm(n)),
            In(n) if 59 <= n && n <= 76 => Ok(In(n)),
            _ => Err(ParserError("Invalid hgt range")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HairColor(String);

impl FromStr for HairColor {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let mut iter = s.chars();

        match iter.next() {
            Some('#') => Ok(()),
            _ => Err(ParserError("Missing hcl prefix")),
        }?;

        let iter = iter.map(|chr| match chr {
            chr @ '0'..='9' | chr @ 'a'..='f' => Ok(chr),
            _ => Err(ParserError("Invalid hcl character")),
        });

        let chars = iter.collect::<ParserResult<Vec<_>>>()?;

        if chars.len() == 6 {
            Ok(HairColor(s.into()))
        } else {
            Err(ParserError("Invalid hcl length"))
        }
    }
}

#[derive(Clone, Debug)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        use EyeColor::*;

        match s {
            "amb" => Ok(Amber),
            "blu" => Ok(Blue),
            "brn" => Ok(Brown),
            "grn" => Ok(Green),
            "gry" => Ok(Gray),
            "hzl" => Ok(Hazel),
            "oth" => Ok(Other),
            _ => Err(ParserError("Invalid ecl value")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PassportId(String);

impl FromStr for PassportId {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let iter = s.chars().map(|chr| match chr {
            chr @ '0'..='9' => Ok(chr),
            _ => Err(ParserError("Invalid pid character")),
        });

        let chars = iter.collect::<ParserResult<Vec<_>>>()?;

        if chars.len() == 9 {
            Ok(PassportId(s.into()))
        } else {
            Err(ParserError("Invalid pid length"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct CountryId(String);

impl FromStr for CountryId {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        Ok(CountryId(s.into()))
    }
}
