use std::collections::{HashMap, HashSet};

use crate::{build_graph, parse};

pub fn lan_party(input: &str) -> usize {
    let connections = parse(input);

    let graph = build_graph(&connections);

    let cycles = get_cycles(&graph);

    cycles
        .iter()
        .filter(|s| s.iter().any(|c| c.starts_with("t")))
        .count()
}

fn get_cycles<'a>(graph: &'a HashMap<&'a str, HashSet<&'a str>>) -> HashSet<Vec<&str>> {
    let mut cycles = HashSet::new();

    for node in graph.keys() {
        let neighbors = &graph[node];

        for &n1 in neighbors {
            let other_neighbours = &graph[&n1];

            for &n2 in other_neighbours {
                if neighbors.contains(&n2) {
                    let mut cycle = vec![node, n1, n2];
                    cycle.sort();
                    cycles.insert(cycle);
                }
            }
        }
    }

    cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

        let result = lan_party(input);
        assert_eq!(7, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = lan_party(input);
        assert_eq!(1370, result);
    }
}
