use anyhow::Result;
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let mut buf = vec![];
    io::stdin().read_to_end(&mut buf)?;
    let instructions_vec = Instruction::from_bytes(buf);
    let mut machine = Machine::new();

    let mut instruction_ptr: isize = 0;
    // let mut instructions_executed = 0;
    'inst_consume: loop {
        match instructions_vec.get(instruction_ptr as usize) {
            Some(instr) => {
                // println!(
                //     "{}",
                //     Instruction::to_string(instructions_vec[instruction_ptr as usize..].to_vec())
                // );
                // println!("-----");
                // println!(
                //     "({:?}){:?}       {:?}",
                //     machine.state, instr, machine.memory
                // );
                instruction_ptr += machine.run_instruction(instr)?;
                // instructions_executed += 1;
                assert!(instruction_ptr >= 0);
                // assert!(instructions_executed < 100);
                // println!("=====");
                // println!("=====");
            }
            None => break 'inst_consume,
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Machine {
    memory: Vec<i64>,
    memory_ptr: usize,
    state: MachineState,
}

#[derive(Debug, Copy, Clone)]
enum SearchDirection {
    Forward,
    Backward,
}

#[derive(Debug)]
enum MachineState {
    Executing,
    FindingCond(SearchDirection, usize),
}

impl Machine {
    fn new() -> Machine {
        Machine {
            memory: vec![0; 65535],
            memory_ptr: 0,
            state: MachineState::Executing,
        }
    }
    fn run_instruction(&mut self, instr: &Instruction) -> Result<isize> {
        let memory_val = self.memory.get_mut(self.memory_ptr).unwrap();
        // println!("MemVal: {}", memory_val);
        match self.state {
            MachineState::Executing => match instr {
                Instruction::IncrementPc => {
                    self.memory_ptr += 1;
                    if self.memory_ptr > self.memory.len() - 1 {
                        self.memory.push(0);
                    }
                    Ok(1)
                }
                Instruction::DecrementPc => {
                    self.memory_ptr -= 1;
                    Ok(1)
                }
                Instruction::IncrementPtr => {
                    *memory_val += 1;
                    Ok(1)
                }
                Instruction::DecrementPtr => {
                    *memory_val -= 1;
                    Ok(1)
                }
                Instruction::OutputPtr => {
                    io::stdout().write(&[*memory_val as u8])?;
                    Ok(1)
                }
                Instruction::InputPtr => {
                    unimplemented!();
                }
                Instruction::CondStart(pair) => {
                    if *memory_val == 0 {
                        self.state = MachineState::FindingCond(SearchDirection::Forward, *pair);
                        Ok(0)
                    } else {
                        Ok(1)
                    }
                }
                Instruction::CondEnd(pair) => {
                    if *memory_val != 0 {
                        self.state = MachineState::FindingCond(SearchDirection::Backward, *pair);
                        Ok(0)
                    } else {
                        Ok(1)
                    }
                }
            },

            MachineState::FindingCond(direction, pair) => match direction {
                SearchDirection::Forward => match instr {
                    Instruction::CondEnd(sub_pair) => {
                        if *sub_pair == pair {
                            self.state = MachineState::Executing;
                            Ok(0)
                        } else {
                            Ok(1)
                        }
                    }
                    _ => Ok(1),
                },
                SearchDirection::Backward => match instr {
                    Instruction::CondStart(sub_pair) => {
                        if *sub_pair == pair {
                            self.state = MachineState::Executing;
                            Ok(0)
                        } else {
                            Ok(-1)
                        }
                    }
                    _ => Ok(-1),
                },
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    // the pointer value itself
    IncrementPc,
    DecrementPc,
    // the value
    IncrementPtr,
    DecrementPtr,
    //
    OutputPtr,
    InputPtr,
    // like x86 jump-if-zero
    CondStart(usize),
    CondEnd(usize),
}

impl Instruction {
    fn from_bytes(buf: Vec<u8>) -> Vec<Instruction> {
        let mut cond_count = 0;
        buf.iter()
            .map(|byte| match *byte {
                0x3e => Some(Instruction::IncrementPc),
                0x3c => Some(Instruction::DecrementPc),
                0x2b => Some(Instruction::IncrementPtr),
                0x2d => Some(Instruction::DecrementPtr),
                0x2e => Some(Instruction::OutputPtr),
                0x2c => Some(Instruction::InputPtr),
                0x5b => {
                    cond_count += 1;
                    Some(Instruction::CondStart(cond_count - 1))
                }
                0x5d => {
                    cond_count -= 1;
                    Some(Instruction::CondEnd(cond_count))
                }
                _ => None,
            })
            .filter(|maybe| maybe.is_some())
            .map(|maybe| maybe.unwrap())
            .collect::<Vec<_>>()
    }
    // fn to_string(instructions: Vec<Instruction>) -> String {
    //     instructions
    //         .iter()
    //         .map(|instr| match instr {
    //             Instruction::IncrementPc => ">",
    //             Instruction::DecrementPc => "<",
    //             Instruction::IncrementPtr => "+",
    //             Instruction::DecrementPtr => "-",
    //             Instruction::OutputPtr => ".",
    //             Instruction::InputPtr => ",",
    //             Instruction::CondStart(_) => "[",
    //             Instruction::CondEnd(_) => "]",
    //         })
    //         .fold(String::new(), |a, b| a + b)
    // }
}
