use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct BingoGame {
    draw: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl FromStr for BingoGame {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct BingoBoard {
    board: Vec<(u32, bool)>,
}

impl FromStr for BingoBoard {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board_numbers: Vec<u32> = s
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        let board: Vec<(u32, bool)> = board_numbers.into_iter().map(|x| (x, false)).collect();
        Ok(BingoBoard { board })
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_board() {
        let board_str = "22 13 17 11  0\n
                          8  2 23  4 24\n
                         21  9 14 16  7\n
                          6 10  3 18  5\n
                          1 12 20 15 19\n";

        let board = board_str
            .parse::<BingoBoard>()
            .expect("failed to parse board");
        assert_eq!(
            board,
            BingoBoard {
                board: vec![
                    (22, false),
                    (13, false),
                    (17, false),
                    (11, false),
                    (0, false),
                    (8, false),
                    (2, false),
                    (23, false),
                    (4, false),
                    (24, false),
                    (21, false),
                    (9, false),
                    (14, false),
                    (16, false),
                    (7, false),
                    (6, false),
                    (10, false),
                    (3, false),
                    (18, false),
                    (5, false),
                    (1, false),
                    (12, false),
                    (20, false),
                    (15, false),
                    (19, false),
                ]
            }
        )
    }
}
