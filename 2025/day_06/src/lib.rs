pub mod part_1;
pub mod part_2;

#[derive(Debug)]
struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> usize {
        let init = match self.operation {
            Operation::Addition => 0,
            Operation::Multiplication => 1,
        };

        self.numbers
            .iter()
            .fold(init, |acc, n| match self.operation {
                Operation::Addition => acc + n,
                Operation::Multiplication => acc * n,
            })
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Addition,
    Multiplication,
}
