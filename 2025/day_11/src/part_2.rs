use std::collections::HashMap;
use crate::{parse, NodeId};

pub fn reactor(input: &str) -> usize {
    let graph = parse(input);

    count_paths(&"svr".into(), &graph, &mut HashMap::new(), (false, false))
}

type State = (bool, bool);

fn count_paths(
    node: &NodeId,
    graph: &HashMap<NodeId, Vec<NodeId>>,
    memo: &mut HashMap<(NodeId, State), usize>,
    mut state: State,
) -> usize {
    let key = (node.clone(), state);
    
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }
    
    match node.as_ref() {
        "fft" => state.0 = true,
        "dac" => state.1 = true,
        "out" => return if state.0 && state.1 { 1 } else { 0 },
        _ => {}
    }

    let total = graph
        .get(node)
        .map(|nodes| nodes.iter().fold(0, |acc, next| {
            acc + count_paths(next, graph, memo, state)
        })).unwrap_or_default();

    memo.insert(key, total);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

        let result = reactor(input);
        assert_eq!(2, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = reactor(input);
        assert_eq!(290219757077250, result);
    }
}
