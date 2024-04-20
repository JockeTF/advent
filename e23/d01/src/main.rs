#[cfg(test)]
mod tests;

const INPUT: &str = include_str!("input.txt");
const VARIANTS: &[(&str, char); 9] = &[
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn extract_digit(input: &str) -> Option<char> {
    if let Some(chr) = input.chars().next() {
        if chr.is_ascii_digit() {
            return Some(chr);
        }
    }

    for (prefix, value) in VARIANTS {
        if input.starts_with(prefix) {
            return Some(*value);
        }
    }

    None
}

fn find_value(input: &str) -> Option<i32> {
    let mut digits = (0..input.len())
        .filter(|i| input.is_char_boundary(*i))
        .map(|i| input.split_at(i).1)
        .filter_map(extract_digit);

    let first = digits.next()?;
    let last = digits.last().unwrap_or(first);
    let number = String::from_iter([first, last]);

    Some(number.parse().unwrap())
}

fn scan_lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(str::trim).filter(|s| !s.is_empty())
}

fn sum_values(input: &str) -> Option<i32> {
    scan_lines(input).map(find_value).sum()
}

fn main() {
    match sum_values(INPUT) {
        Some(value) => println!("Calibration value: {value}"),
        None => println!("Could not find calibration value."),
    }
}
