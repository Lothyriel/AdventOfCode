pub fn get_focusing_power(input: &str) -> usize {
    let steps = input.trim().split(',').map(Step::parse);

    get_boxes(steps).iter().map(Box::focal_power).sum()
}

fn get_boxes(steps: impl Iterator<Item = Step>) -> Vec<Box> {
    let mut boxes = Vec::new();

    for s in steps {
        step(s, &mut boxes);
    }

    boxes
}

fn step(s: Step, boxes: &mut Vec<Box>) -> Option<()> {
    let number = crate::hash(&s.label);

    let possible_box = boxes.iter_mut().find(|b| b.number == number);

    let lens = |p| Lens {
        label: s.label.to_owned(),
        focal_power: p,
    };

    match s.op {
        Operation::Upsert(p) => match possible_box {
            Some(b) => upsert_lens(b, &s.label, lens(p)),
            None => boxes.push(Box {
                lenses: vec![lens(p)],
                number,
            }),
        },
        Operation::Remove => {
            remove_lens(possible_box?, &s.label);
        }
    }

    Some(())
}

fn upsert_lens(boxx: &mut Box, label: &str, lens: Lens) {
    match boxx.get_lens_idx(label) {
        Some(l) => boxx.lenses[l] = lens,
        None => boxx.lenses.push(lens),
    }
}

fn remove_lens(boxx: &mut Box, label: &str) -> Option<()> {
    boxx.lenses.remove(boxx.get_lens_idx(label)?);

    Some(())
}

#[derive(Debug)]
struct Step {
    op: Operation,
    label: String,
}

impl Step {
    fn parse(input: &str) -> Self {
        let l = input
            .bytes()
            .take_while(|b| !matches!(b, b'=' | b'-'))
            .count();

        let label = input[..l].to_owned();

        let op = match &input[l..l + 1] {
            "=" => {
                let focal_power = input[l + 1..].parse().expect("Should contain focal_power");
                Operation::Upsert(focal_power)
            }
            "-" => Operation::Remove,
            _ => panic!("Invalid operation"),
        };

        Self { op, label }
    }
}

#[derive(Debug)]
enum Operation {
    Upsert(usize),
    Remove,
}

#[derive(Debug)]
struct Box {
    lenses: Vec<Lens>,
    number: usize,
}

impl Box {
    fn focal_power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, l)| (i + 1) * l.focal_power * (self.number + 1))
            .sum()
    }

    fn get_lens_idx(&self, label: &str) -> Option<usize> {
        self.lenses
            .iter()
            .enumerate()
            .find(|(_, l)| l.label == label)
            .map(|l| l.0)
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_power: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_focusing_power("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(result, 145);
    }

    #[test]
    fn puzzle() {
        let result = get_focusing_power(include_str!("input"));

        assert_eq!(result, 247933);
    }
}
