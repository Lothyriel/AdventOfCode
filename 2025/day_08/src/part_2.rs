use lib::Dsu;

use crate::{get_unique_pairs, parse};

pub fn playground(input: &str) -> usize {
    let boxes: Vec<_> = parse(input).collect();

    let pairs = get_unique_pairs(&boxes);

    let mut dsu = Dsu::new(boxes.len());

    for &(_, i, j) in &pairs {
        if dsu.union(i, j) && dsu.count() == 1 {
            return (boxes[i].x * boxes[j].x) as usize;
        }
    }

    unreachable!("impossibru");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

        let result = playground(input);
        assert_eq!(25272, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = playground(input);
        assert_eq!(7017750530, result);
    }
}
