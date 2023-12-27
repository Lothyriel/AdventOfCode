pub fn get_groups_value(input: &str) -> usize {
    let components = crate::parse(input);

    for c in &components {
        println!("{:?}", c.1.borrow());
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_groups_value(include_str!("example"));
        assert_eq!(result, 54);
    }

    // #[test]
    // fn puzzle() {
    //     let result = get_groups_value(include_str!("input"));
    //     assert_eq!(result, 0);
    // }
}
