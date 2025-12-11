use std::collections::HashMap;
use crate::{parse, NodeId};

pub fn reactor(input: &str) -> usize {
    let graph = parse(input);

    count_paths(&"you".into(), &graph, &mut HashMap::new())
}

fn count_paths(
    node: &NodeId,
    graph: &HashMap<NodeId, Vec<NodeId>>,
    memo: &mut HashMap<NodeId, usize>,
) -> usize {
    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    if node.as_ref() == "out" {
        return 1;
    }

    let total = graph
        .get(node)
        .map(|nodes| nodes.iter().fold(0, |acc, next| {
            acc + count_paths(next, graph, memo)
        })).unwrap_or_default();

    memo.insert(node.clone(), total);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

        let result = reactor(input);
        assert_eq!(5, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = reactor(input);
        assert_eq!(733, result);
    }
}