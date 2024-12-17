pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> Program {
    let mut lines = input.lines();

    let mut parse_register = || {
        lines
            .next()
            .expect("Register")
            .split(':')
            .nth(1)
            .expect("Number")
            .trim()
            .parse()
            .expect("Valid Number")
    };

    let a = parse_register();
    let b = parse_register();
    let c = parse_register();

    let instructions = lines
        .nth(1)
        .expect("Program")
        .split(':')
        .nth(1)
        .expect("Instructions")
        .split(',')
        .map(|n| match n.trim().as_bytes()[0] {
            b'0' => Instruction::adv,
            b'1' => Instruction::bxl,
            b'2' => Instruction::bst,
            b'3' => Instruction::jnz,
            b'4' => Instruction::bxc,
            b'5' => Instruction::out,
            b'6' => Instruction::bdv,
            b'7' => Instruction::cdv,
            _ => panic!("Unexpected instruction"),
        })
        .collect();

    Program {
        a,
        b,
        c,
        instructions,
        pointer: 0,
        output: vec![],
    }
}

struct Program {
    a: isize,
    b: isize,
    c: isize,
    instructions: Vec<Instruction>,
    pointer: usize,
    output: Vec<isize>,
}

impl Program {
    fn execute(&mut self) {
        while let Some((i, o)) = self.next_instruction() {
            if self.execute_instruction(i, o) {
                self.pointer += 2;
            }
        }
    }

    // returns if the pointer should be increased by two (didn't jump)
    fn execute_instruction(&mut self, i: Instruction, o: Instruction) -> bool {
        match i {
            Instruction::adv => self.a /= 2isize.pow(self.combo_operand_value(o) as u32),
            Instruction::bxl => self.b ^= self.literal_operand_value(o),
            Instruction::bst => self.b = self.combo_operand_value(o) % 8,
            Instruction::jnz => {
                if self.a != 0 {
                    self.pointer = self.literal_operand_value(o) as usize;
                    return false;
                }
            }
            Instruction::bxc => self.b ^= self.c,
            Instruction::out => self.output.push(self.combo_operand_value(o) % 8),
            Instruction::bdv => self.b = self.a / 2isize.pow(self.combo_operand_value(o) as u32),
            Instruction::cdv => self.c = self.a / 2isize.pow(self.combo_operand_value(o) as u32),
        };

        true
    }

    fn next_instruction(&self) -> Option<(Instruction, Instruction)> {
        let instruction = self.instructions.get(self.pointer).copied()?;

        let operand = self
            .instructions
            .get(self.pointer + 1)
            .copied()
            .expect("Operand outside of bounds");

        Some((instruction, operand))
    }

    fn literal_operand_value(&self, instruction: Instruction) -> isize {
        instruction as isize
    }

    fn combo_operand_value(&self, instruction: Instruction) -> isize {
        match instruction {
            Instruction::adv | Instruction::bxl | Instruction::bst | Instruction::jnz => {
                self.literal_operand_value(instruction)
            }
            Instruction::bxc => self.a,
            Instruction::out => self.b,
            Instruction::bdv => self.c,
            Instruction::cdv => panic!("cdv doesn't have a combo operand value"),
        }
    }

    fn read_output(&self) -> String {
        self.output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
enum Instruction {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}
