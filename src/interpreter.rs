use std::io::{self, Read, Write};

pub fn interpret(instructions: Vec<u8>) {
    let mut tape = vec![0u8; 30000];
    let mut data_idx: usize = 0;

    let mut pc: usize = 0;
    while pc < instructions.len() {
        match &instructions[pc] {
            b'>' => data_idx += 1,
            b'<' => data_idx -= 1,
            b'+' => tape[data_idx] += 1,
            b'-' => tape[data_idx] -= 1,
            b'.' => {
                io::stdout().write(&tape[data_idx..=data_idx]).unwrap();
            }
            b',' => {
                let mut buf = [0u8; 1];
                io::stdin().read_exact(&mut buf).unwrap();
                tape[data_idx] = buf[0];
            },
            b'[' => {
                if tape[data_idx] == 0 {
                    let mut layer: usize = 1;
                    while layer > 0  {
                        pc += 1;
                        match &instructions[pc] {
                            b'[' => layer += 1,
                            b']' => layer -= 1,
                            _ => ()
                        }
                    }
                }
            },
            b']' => {
                if tape[data_idx] != 0 {
                    let mut layer: usize = 1;
                    while layer > 0  {
                        pc -= 1;
                        match &instructions[pc] {
                            b']' => layer += 1,
                            b'[' => layer -= 1,
                            _ => ()
                        }
                    }
                }
            },
            _ => ()
        }
        pc += 1;
    }
}