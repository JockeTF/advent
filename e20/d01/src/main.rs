use std::num::ParseIntError;

const INPUT: &str = include_str!("input.txt");
const TARGET: i32 = 2020;

fn parse() -> Result<Vec<i32>, ParseIntError> {
    INPUT.lines().map(|line| line.parse::<i32>()).collect()
}

fn find_pair(values: &[i32]) -> Option<(i32, i32)> {
    for x in values.iter() {
        let y = TARGET - x;

        if values.binary_search(&y).is_ok() {
            return Some((*x, y));
        }
    }

    None
}

fn find_triplet(values: &[i32]) -> Option<(i32, i32, i32)> {
    for (i, x) in values.iter().enumerate() {
        let (_, split) = values.split_at(i);

        for (j, y) in split.iter().enumerate() {
            let (_, split) = values.split_at(j);
            let z = TARGET - x - y;

            if split.binary_search(&z).is_ok() {
                return Some((*x, *y, z));
            }
        }
    }

    None
}

fn main() {
    let mut values = parse().expect("Input not all numbers.");
    values.sort();

    let pair = find_pair(&values).expect("Could not find pair.");
    println!("Target Pair: {}", pair.0 * pair.1);

    let triplet = find_triplet(&values).expect("Cound not find triplet.");
    println!("Target Triplet: {}", triplet.0 * triplet.1 * triplet.2);
}
