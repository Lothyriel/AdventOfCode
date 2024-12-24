use std::collections::{HashMap, VecDeque};

use crate::{parse, to_number, Gate, Wire};

pub fn crossed_wires(input: &str) -> usize {
    let (mut wires, gates) = parse(input);

    let z_wires = simulate_gates(&mut wires, gates);

    to_number(&z_wires)
}

fn simulate_gates(wires: &mut HashMap<String, bool>, mut gates: VecDeque<Gate>) -> Vec<Wire> {
    let mut output = vec![];

    while let Some(c) = gates.pop_front() {
        let w = (wires.get(&c.left), wires.get(&c.right));

        if let (Some(&left), Some(&right)) = w {
            let state = c.op.apply(left, right);

            output.push((c.output.to_owned(), state));
            wires.insert(c.output, state);
        } else {
            gates.push_back(c);
        }
    }

    output.sort_by(|a, b| a.0.cmp(&b.0));
    output.retain(|w| w.0.starts_with('z'));
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_example() {
        let input = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

        let result = crossed_wires(input);
        assert_eq!(4, result);
    }

    #[test]
    fn larger_example() {
        let input = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

        let result = crossed_wires(input);
        assert_eq!(2024, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = crossed_wires(input);
        assert_eq!(65635066541798, result);
    }
}
