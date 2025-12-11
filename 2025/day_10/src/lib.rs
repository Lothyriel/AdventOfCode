use std::collections::VecDeque;

pub mod part_1;

#[derive(Debug)]
struct Machine {
    buttons: Vec<Button>,
    final_config: Vec<LightState>,
    joltage: Vec<usize>,
}

impl Machine {
    fn parse(input: &str) -> Self {
        let mut parts: VecDeque<_> = input.split(' ').collect();

        let final_config = parts.pop_front().unwrap().trim_matches(&['[', ']']).bytes().map(|b| match b
        {
            b'#' => LightState::On,
            b'.' => LightState::Off,
            l => panic!("Invalid light state {}", l as char)
        }).collect();

        let joltage = parts.pop_back().unwrap()
            .trim_matches(&['{', '}'])
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let buttons = parts
            .drain(..)
            .map(|x| x.trim_matches(&['(', ')']))
            .map(|x| {
                let lights = x.split(',').map(|x| x.parse().unwrap()).collect();
                Button{ lights  }
            })
            .collect();

        Machine { final_config, buttons, joltage }
    }
}

#[derive(Debug)]
struct Button {
    lights: Vec<usize>
}

impl Button {
    fn press(&self, state: &mut Vec<LightState>) {
        for &light in &self.lights {
            state[light].flip()
        }
    }

    fn press_joltage(&self, state: &mut Vec<usize>) {
        for &light in &self.lights {
            state[light] += 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum LightState {
    On,
    Off,
}

impl LightState {
    fn flip(&mut self) {
        *self = match self {
            LightState::On => LightState::Off,
            LightState::Off => LightState::On,
        }
    }
}