pub mod part_1;
pub mod part_2;

fn hash(input: &str) -> usize {
    input
        .bytes()
        .fold(0, |acc, b| ((acc + b as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let result = hash("HASH");

        assert_eq!(result, 52);
    }
}
