use std::convert::TryFrom;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
const RUNS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

#[derive(Debug)]
enum ParserError {
    InvalidCharacter,
}

type ParserResult<T> = Result<T, ParserError>;

#[derive(Clone, Debug)]
enum Tile {
    Empty,
    Tree,
}

impl TryFrom<char> for Tile {
    type Error = ParserError;

    fn try_from(value: char) -> ParserResult<Self> {
        use ParserError::InvalidCharacter;

        match value {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Tree),
            _ => Err(InvalidCharacter),
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for Board {
    type Err = ParserError;

    fn from_str(s: &str) -> ParserResult<Self> {
        let tiles = s
            .lines()
            .map(|line| line.chars().map(Tile::try_from).collect())
            .collect::<ParserResult<Vec<Vec<Tile>>>>()?;

        Ok(Board { tiles })
    }
}

impl Board {
    fn get_tile(&self, row: usize, column: usize) -> Option<Tile> {
        let row = self.tiles.get(row)?;
        let tile = row.get(column % row.len())?;

        Some(tile.clone())
    }

    fn count_trees(&self, col_speed: usize, row_speed: usize) -> usize {
        let row_count = self.tiles.len();

        let mut trees = 0;
        let mut current_row = row_speed;
        let mut current_col = col_speed;

        while current_row < row_count {
            let tile = self
                .get_tile(current_row, current_col)
                .expect("Invalid board position");

            trees += match tile {
                Tile::Empty => 0,
                Tile::Tree => 1,
            };

            current_row += row_speed;
            current_col += col_speed;
        }

        trees
    }
}

fn main() {
    let board = INPUT.parse::<Board>().expect("Could not parse board");

    let first: usize = board.count_trees(3, 1);
    println!("Number of trees (first task): {}", first);

    let second = RUNS
        .iter()
        .map(|run| board.count_trees(run.0, run.1))
        .product::<usize>();

    println!("Number of trees (second task): {}", second);
}
