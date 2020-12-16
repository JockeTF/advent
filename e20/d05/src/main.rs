use std::convert::TryInto;
use std::mem::size_of;
use std::str::FromStr;

#[cfg(test)]
mod tests;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Clone, Debug)]
struct ParserError(&'static str);

type ParserResult<T> = Result<T, ParserError>;

#[derive(Clone, Debug)]
enum Bound {
    Lower,
    Upper,
}

impl Bound {
    fn try_from_row(value: char) -> ParserResult<Self> {
        match value {
            'F' => Ok(Bound::Lower),
            'B' => Ok(Bound::Upper),
            _ => Err(ParserError("Invalid row value")),
        }
    }

    fn try_from_col(value: char) -> ParserResult<Self> {
        match value {
            'L' => Ok(Bound::Lower),
            'R' => Ok(Bound::Upper),
            _ => Err(ParserError("Invalid col value")),
        }
    }
}

#[derive(Clone, Debug)]
struct Seat {
    row: [Bound; 7],
    col: [Bound; 3],
}

impl Seat {
    fn number(slice: &[Bound]) -> u16 {
        use Bound::*;

        let len = slice.len();

        if size_of::<u16>() * 8 <= len {
            panic!("Unsupported slice length");
        }

        let mut lower = 0u16;
        let mut upper = 2u16.pow(len as u32);

        let start = &slice[..len - 1];

        start.iter().for_each(|spec| match spec {
            Lower => upper -= (upper - lower) / 2,
            Upper => lower += (upper - lower) / 2,
        });

        match slice[len - 1] {
            Lower => lower,
            Upper => upper - 1,
        }
    }

    fn row_number(&self) -> u16 {
        Self::number(&self.row)
    }

    fn col_number(&self) -> u16 {
        Self::number(&self.col)
    }

    fn seat_number(&self) -> u32 {
        let row = self.row_number() as u32;
        let col = self.col_number() as u32;

        row * 8 + col
    }
}

impl FromStr for Seat {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let mut iter = s.chars();

        let row = (&mut iter)
            .take(7)
            .map(Bound::try_from_row)
            .collect::<ParserResult<Vec<_>>>()?
            .try_into()
            .map_err(|_| ParserError("Invalid row count"))?;

        let col = (&mut iter)
            .take(3)
            .map(Bound::try_from_col)
            .collect::<ParserResult<Vec<_>>>()?
            .try_into()
            .map_err(|_| ParserError("Invalid col count"))?;

        if iter.count() == 0 {
            Ok(Seat { row, col })
        } else {
            Err(ParserError("Invalid value count"))
        }
    }
}

fn main() {
    let mut seats = INPUT
        .lines()
        .map(str::trim)
        .map(Seat::from_str)
        .collect::<ParserResult<Vec<Seat>>>()
        .expect("Invalid seat specification");

    println!("Occupied seat count: {}", seats.len());

    seats.sort_by_key(|seat| seat.seat_number());

    let lower = seats.first().expect("No first seat").seat_number();
    let upper = seats.last().expect("No last seat").seat_number();
    let range = lower..=upper;

    let (empty, _) = range
        .zip(seats.iter())
        .find(|(i, seat)| *i != seat.seat_number())
        .expect("Could not find empty seat");

    println!("Last occupied seat: {}", upper);
    println!("First empty seat: {}", empty);
}
