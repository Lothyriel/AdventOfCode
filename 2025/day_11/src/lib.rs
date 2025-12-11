use std::collections::HashMap;
use std::rc::Rc;

pub mod part_1;
pub mod part_2;

type NodeId = Rc<str>;

fn parse(input: &str) -> HashMap<NodeId, Vec<NodeId>>{
    let mut graph = HashMap::new();

    for line in input.lines() {
        if let Some((src, rest)) = line.split_once(':') {
            let src = src.trim().to_string().into();
            let outputs: Vec<_> = rest
                .split_whitespace()
                .map(|s| s.to_string().into())
                .collect();

            graph.insert(src, outputs);
        }
    }

    graph
}