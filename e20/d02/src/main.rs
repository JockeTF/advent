use std::str::FromStr;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug)]
enum ErrorType {
    EntryError(&'static str),
    PolicyError(&'static str),
}

#[derive(Debug)]
struct ParseError(ErrorType, usize);

type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
struct Policy {
    target: char,
    min: usize,
    max: usize,
}

impl FromStr for Policy {
    type Err = ErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ErrorType::PolicyError;

        let parts = s.split_whitespace().collect::<Vec<_>>();

        let (spec, target) = match parts[..] {
            [spec, target] => Ok((spec, target)),
            _ => Err(PolicyError("Invalid target divider")),
        }?;

        let spec = spec.split("-").map(str::trim).collect::<Vec<_>>();

        let (min, max) = match spec[..] {
            [min, max] => Ok((min, max)),
            _ => Err(PolicyError("Invalid specificer divider")),
        }?;

        let target = target
            .parse::<char>()
            .map_err(|_| PolicyError("Invalid target length"))?;

        let min = min
            .parse::<usize>()
            .map_err(|_| PolicyError("Invalid min specificer"))?;

        let max = max
            .parse::<usize>()
            .map_err(|_| PolicyError("Invalid max specificer"))?;

        Ok(Policy { target, min, max })
    }
}

#[derive(Debug)]
struct Entry {
    policy: Policy,
    password: String,
}

impl FromStr for Entry {
    type Err = ErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ErrorType::EntryError;

        let parts = s.split(":").map(str::trim).collect::<Vec<_>>();

        let (policy, password) = match parts[..] {
            [policy, password] => Ok((policy, password)),
            _ => Err(EntryError("Invalid password divider")),
        }?;

        let policy = policy.parse::<Policy>()?;
        let password = password.to_owned();

        Ok(Entry { policy, password })
    }
}

fn parse_line(line: (usize, &str)) -> ParseResult<Entry> {
    let (number, text) = line;

    text.parse().map_err(|error| ParseError(error, number))
}

fn parse() -> ParseResult<Vec<Entry>> {
    INPUT.lines().enumerate().map(parse_line).collect()
}

fn validate_primary(entry: &Entry) -> bool {
    let password = &entry.password;
    let policy = &entry.policy;

    let count = password.matches(policy.target).count();

    policy.min <= count && count <= policy.max
}

fn validate_secondary(entry: &Entry) -> bool {
    let password = &entry.password;
    let policy = &entry.policy;
    let target = &policy.target;

    let plucked = [
        password.chars().nth(policy.min - 1),
        password.chars().nth(policy.max - 1),
    ];

    let chars = plucked.iter().filter_map(|opt| *opt).collect::<Vec<char>>();
    let matches = chars.iter().filter(|chr| *chr == target).count();

    matches == 1
}

fn main() -> ParseResult<()> {
    let entries = parse()?;

    let primary = entries.iter().filter(|e| validate_primary(e)).count();
    println!("Primary count: {}", primary);

    let secondary = entries.iter().filter(|e| validate_secondary(e)).count();
    println!("Secondary count: {}", secondary);

    Ok(())
}
