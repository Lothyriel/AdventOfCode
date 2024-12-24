use std::collections::{HashMap, VecDeque};

pub mod part_1;
pub mod part_2;

type Wire = (String, bool);

#[derive(Debug)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Operator::And => a && b,
            Operator::Or => a || b,
            Operator::Xor => a ^ b,
        }
    }
}

#[derive(Debug)]
struct Gate {
    left: String,
    op: Operator,
    right: String,
    output: String,
}

fn to_number(wires: &[Wire]) -> usize {
    wires
        .iter()
        .rev()
        .fold(0, |acc, w| (acc << 1) | (w.1 as usize))
}

fn parse(input: &str) -> (HashMap<String, bool>, VecDeque<Gate>) {
    let mut sections = input.split("\n\n");

    let wires = sections
        .next()
        .expect("Wires")
        .lines()
        .map(parse_wire)
        .collect();

    let gates = sections
        .next()
        .expect("Gates")
        .lines()
        .map(parse_gates)
        .collect();

    (wires, gates)
}

fn parse_gates(input: &str) -> Gate {
    let mut parts = input.split_whitespace();

    let left = parts.next().expect("Wire a").to_owned();

    let op = match parts.next().expect("State") {
        "AND" => Operator::And,
        "OR" => Operator::Or,
        "XOR" => Operator::Xor,
        _ => panic!("Invalid operator"),
    };

    let right = parts.next().expect("Wire b").to_owned();

    let output = parts.nth(1).expect("Wire c").to_owned();

    Gate {
        left,
        op,
        right,
        output,
    }
}

fn parse_wire(input: &str) -> Wire {
    let mut parts = input.split(": ");

    let wire = parts.next().expect("Wire").to_owned();

    let state = match parts.next().expect("State") {
        "1" => true,
        "0" => false,
        _ => panic!("Invalid state"),
    };

    (wire, state)
}
