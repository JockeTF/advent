use std::num::ParseIntError;

const TARGET: i32 = 2020;

const INPUT: &'static str = include_str!("input.txt");

fn parse() -> Result<Vec<i32>, ParseIntError> {
    INPUT.lines().map(|line| line.parse::<i32>()).collect()
}

fn find_pair(values: &Vec<i32>) -> Option<(i32, i32)> {
    for i in values.iter() {
        for j in values.iter().rev() {
            if i + j == TARGET {
                return Some((*i, *j));
            } else if i + j < TARGET {
                break;
            }
        }
    }

    None
}

fn find_triplet(values: &Vec<i32>) -> Option<(i32, i32, i32)> {
    let mut mid = values.len() as isize / 2;
    let mut flip = -1;

    for i in 0..values.len() as isize {
        mid += i * flip;
        flip *= -1;

        dbg!(mid);

        let (left, right) = values.split_at(mid as usize);
        let (cent, right) = right.split_at(1);
        let pivot = &cent[0];

        for j in left.iter() {
            for k in right.iter().rev() {
                if pivot + j + k == TARGET {
                    return Some((*j, *pivot, *k));
                }
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
