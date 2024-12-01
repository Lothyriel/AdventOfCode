pub fn get_hash_sum(input: &str) -> usize {
    input.trim().split(',').map(crate::hash).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_hash_sum("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(result, 1320);
    }

    #[test]
    fn puzzle() {
        let result = get_hash_sum(include_str!("input"));

        assert_eq!(result, 494980);
    }
}
