use crate::{Range, parse};

pub fn cafeteria(input: &str) -> usize {
    let (mut fresh, _) = parse(input);
    fresh.sort_by_key(|r| r.start);

    let merged = fresh.iter().fold(Vec::new(), |mut acc: Vec<Range>, r| {
        match acc.last_mut() {
            Some(last) if r.start <= last.end => {
                last.end = last.end.max(r.end);
            }
            _ => acc.push(*r),
        }
        acc
    });

    merged.iter().map(|r| r.end - r.start + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        let result = cafeteria(input);
        assert_eq!(14, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = cafeteria(input);
        assert_eq!(354226555270043, result);
    }
}
