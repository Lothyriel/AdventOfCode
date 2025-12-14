use crate::{parse, rectangles};

pub fn movie_theater(input: &str) -> usize {
    let points = parse(input);

    rectangles(&points).map(|r| r.area()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

        let result = movie_theater(input);
        assert_eq!(50, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = movie_theater(input);
        assert_eq!(4781546175, result);
    }
}
