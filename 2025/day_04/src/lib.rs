pub mod part_1;
pub mod part_2;

use lib::Matrix;

fn parse(input: &str) -> Matrix<Tile> {
    Matrix::parse(input, |b| match b {
        b'@' => Tile::PaperRoll,
        b'.' => Tile::Empty,
        t => panic!("Invalid tile {}", t as char),
    })
}

#[derive(PartialEq)]
enum Tile {
    PaperRoll,
    Empty,
}
