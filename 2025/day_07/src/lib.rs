use lib::Matrix;

pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> Matrix<Tile> {
    Matrix::parse(input, |b| match b {
        b'S' => Tile::Start,
        b'.' => Tile::Empty,
        b'^' => Tile::Splitter,
        t => panic!("invalid tile {}", t as char),
    })
}

#[derive(PartialEq)]
enum Tile {
    Splitter,
    Start,
    Empty,
}
