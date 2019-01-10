use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Register {
    ZERO,
    R0,
    R1,
    R2,
    R3,
    PC,
    FLAGS,
}

type Value = i32;

type MemoryAddress = i32;

#[derive(Debug)]
enum Operand {
    Immediate(Value),
    Register(Register),
    Memory(MemoryAddress)
}

#[derive(Debug)]
enum Instruction {
    NOP{},
    ADD{source: Operand, dest: Operand},
}

#[derive(Debug)]
struct Machine {
    register : HashMap<Register, Value>,
    memory: HashMap<MemoryAddress, Value>,
}

impl Machine {
    pub fn new () -> Machine {
        Machine {
            register: HashMap::new(),
            memory: HashMap::new(),
        }
    }
}

impl Machine {
    pub fn execute_instruction(&mut self, i: &Instruction) {
        println!("-> Running {:?}", i);
        match i {
            Instruction::NOP{} => (),
            
            Instruction::ADD{
                source: Operand::Immediate(value),
                dest: Operand::Register(reg) 
            } => (),

            _ => (),
        }
    }
}

fn main() {
    let mut machine = Machine::new();
    println!("{:?}", machine);
    
    machine.execute_instruction(&Instruction::NOP{});
    println!("{:?}", machine);

    machine.execute_instruction(&Instruction::ADD{
        source: Operand::Immediate(3),
        dest: Operand::Register(Register::ZERO)
    });
    println!("{:?}", machine);
}
    