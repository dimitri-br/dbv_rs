use std::path::Path;
use std::io::Read;
use std::fs::File;

use crate::instructions::{InstructionMode, Instructions};
use crate::registers::Registers;
use crate::memory::Memory;
use crate::utils::{Parameter, decode_instructions};

pub struct VirtualMachine{
    pub registers: Registers,
    pub memory: Memory,

    program: Vec<(Instructions, InstructionMode, Vec<Parameter>)>, // The program is stored as a vector of u32, as that's the size of a FULL instruction

    // Runtime Flags
    has_jumped: bool,
}

impl VirtualMachine{
    pub fn new() -> Self{
        VirtualMachine{
            registers: Registers::new(),
            memory: Memory::new(),
            program: Vec::new(),

            has_jumped: false,
        }
    }

    pub fn dump(&self){
        // print out register state
        println!("Registers:");
        println!("PC: 0x{:04X}", self.registers.get_pc());
        println!("SP: 0x{:04X}", self.registers.get_sp());
        println!("CMP: 0x{:02X}", self.registers.get_cmp_flag());
        println!("");
        for i in 0..self.registers.registers.len(){
            println!("R{}: 0x{:08X}", i, self.registers.get_register(i));
        }

        println!("");
        // output the result of the data stored in reg 3
        println!("Data:");
        println!("{:?}", self.memory.get_memory(0x2000));
    }

    pub fn load_program<T>(&mut self, file_path: &T) -> Result<(), &'static str> where T: AsRef<Path> + ?Sized{
        let mut file = File::open(file_path).unwrap();
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer).unwrap();

        let mut program = Vec::new();
        // Read the file in 32 bit chunks, stored big endian
        for i in 0..file_buffer.len() / 4{
            let mut instruction: u32 = 0;
            for j in 0..4{
                instruction |= (file_buffer[i * 4 + j] as u32) << (24 - j * 8);
            }
            program.push(instruction);
        }

        let program = decode_instructions(&program);

        self.program = program;

        for i in 0..self.program.len(){
            println!("{:?}", self.program[i]);
        }

        Ok(())
    }

    fn fetch(&self) -> (Instructions, InstructionMode, Vec<Parameter>){
        let (instruction, mode, args) = &self.program[self.registers.get_pc()];
        // clone and return
        (instruction.clone(), mode.clone(), args.clone())
    }

    pub fn run(&mut self) -> Result<(), &'static str>{

        // Run the program
        'running: loop {
            // Get the instruction
            let (instruction, mode, args) = self.fetch();
            if self.execute(instruction, mode, args){
                break 'running;
            }

            // Increment the program counter
            if !self.has_jumped{
                self.registers.set_pc(self.registers.get_pc() + 1);
            }else{
                self.has_jumped = false;
            }
        }

        Ok(())
    }


    fn execute(&mut self, opcode: Instructions, mode: InstructionMode, args: Vec<Parameter>) -> bool{
        match opcode{
            Instructions::HLT => return true,

            Instructions::PSH => {}
            Instructions::POP => {}
            Instructions::SET => {
                match mode{
                    InstructionMode::Register => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);

                        // Get the value from the register
                        let source_register = args[1].get_value(&self.registers, &self.memory);
                        let value = self.registers.get_register(source_register as usize);

                        self.registers.set_register(destination_register as usize, value);
                    },
                    InstructionMode::Immediate => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);

                        // The value just is the value
                        let value = args[2].get_value(&self.registers, &self.memory);

                        self.registers.set_register(destination_register as usize, value);
                    },
                    InstructionMode::RegisterIndirect => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);
                        
                        let source_register = args[1].get_value(&self.registers, &self.memory);
                        // Get the address from the register
                        let address = self.registers.get_register(source_register as usize);
                        // Load the value from memory
                        let value = self.memory.get_memory(address as usize);

                        self.registers.set_register(destination_register as usize, value);
                    },
                    InstructionMode::BaseOffset => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);

                        let source_register = args[1].get_value(&self.registers, &self.memory);
                        let address = self.registers.get_register(source_register as usize);
                        let offset = args[2].get_value(&self.registers, &self.memory);

                        let address = address + offset;

                        let value = self.memory.get_memory(address as usize);

                        self.registers.set_register(destination_register as usize, value);
                    },
                }
            }
            Instructions::MOV => {}

            Instructions::ADD => {}
            Instructions::SUB => {
                match mode{
                    InstructionMode::Register => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);

                        // Get the value from the register
                        let a_register = args[1].get_value(&self.registers, &self.memory);
                        let a_value = self.registers.get_register(a_register as usize);

                        let b_register = args[2].get_value(&self.registers, &self.memory);
                        let b_value = self.registers.get_register(b_register as usize);

                        self.registers.set_register(destination_register as usize, a_value - b_value);
                    },
                    InstructionMode::Immediate => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);
                        
                        let a_register = args[1].get_value(&self.registers, &self.memory);
                        let a_value = self.registers.get_register(a_register as usize);

                        let b_value = args[2].get_value(&self.registers, &self.memory);

                        self.registers.set_register(destination_register as usize, a_value - b_value);
                    },
                    InstructionMode::RegisterIndirect => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);
                        
                        let a_register = args[1].get_value(&self.registers, &self.memory);
                        let a_value = self.registers.get_register(a_register as usize);

                        let b_register = args[2].get_value(&self.registers, &self.memory);
                        let b_address = self.registers.get_register(b_register as usize);

                        let b_value = self.memory.get_memory(b_address as usize);

                        self.registers.set_register(destination_register as usize, a_value - b_value);
                    },
                    InstructionMode::BaseOffset => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);

                        let a_register = args[1].get_value(&self.registers, &self.memory);
                        let a_value = self.registers.get_register(a_register as usize);

                        let b_register = args[2].get_value(&self.registers, &self.memory);
                        let b_address = self.registers.get_register(b_register as usize);

                        let offset = args[3].get_value(&self.registers, &self.memory);

                        let b_address = b_address + offset;
                        let b_value = self.memory.get_memory(b_address as usize);

                        self.registers.set_register(destination_register as usize, a_value - b_value);
                    }
                }
            }
            Instructions::MUL => {
                match mode{
                    InstructionMode::Register => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);

                        // Get the value from the register
                        let a_register = args[1].get_value(&self.registers, &self.memory);
                        let a_value = self.registers.get_register(a_register as usize);

                        let b_register = args[2].get_value(&self.registers, &self.memory);
                        let b_value = self.registers.get_register(b_register as usize);

                        self.registers.set_register(destination_register as usize, a_value * b_value);
                    },
                    InstructionMode::Immediate => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);
                        
                        let a_register = args[1].get_value(&self.registers, &self.memory);
                        let a_value = self.registers.get_register(a_register as usize);

                        let b_value = args[2].get_value(&self.registers, &self.memory);

                        self.registers.set_register(destination_register as usize, a_value * b_value);
                    },
                    InstructionMode::RegisterIndirect => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);
                        
                        let a_register = args[1].get_value(&self.registers, &self.memory);
                        let a_value = self.registers.get_register(a_register as usize);

                        let b_register = args[2].get_value(&self.registers, &self.memory);
                        let b_address = self.registers.get_register(b_register as usize);

                        let b_value = self.memory.get_memory(b_address as usize);

                        self.registers.set_register(destination_register as usize, a_value * b_value);
                    },
                    InstructionMode::BaseOffset => {
                        let destination_register = args[0].get_value(&self.registers, &self.memory);

                        let a_register = args[1].get_value(&self.registers, &self.memory);
                        let a_value = self.registers.get_register(a_register as usize);

                        let b_register = args[2].get_value(&self.registers, &self.memory);
                        let b_address = self.registers.get_register(b_register as usize);

                        let offset = args[3].get_value(&self.registers, &self.memory);

                        let b_address = b_address + offset;
                        let b_value = self.memory.get_memory(b_address as usize);

                        self.registers.set_register(destination_register as usize, a_value * b_value);                        
                    },
                }
            }
            Instructions::DIV => {}

            Instructions::AND => {}
            Instructions::OR => {}
            Instructions::XOR => {}
            Instructions::NOT => {}

            Instructions::SL => {}
            Instructions::SR => {}

            Instructions::SD => {
                match mode{
                    InstructionMode::Register => {
                        println!("Invalid mode for SD");
                        let source_register = args[0].get_value(&self.registers, &self.memory);
                        let source_value = self.registers.get_register(source_register as usize);

                        // Get the value from the register
                        let destination_register = args[1].get_value(&self.registers, &self.memory);
                        let destination_value = self.registers.get_register(destination_register as usize);

                        self.memory.set_memory(destination_value as usize, source_value);
                    },
                    InstructionMode::Immediate => {
                        let source_register = args[0].get_value(&self.registers, &self.memory);
                        let source_value = self.registers.get_register(source_register as usize);

                        // The value just is the value
                        let destination_value = args[1].get_value(&self.registers, &self.memory);

                        self.memory.set_memory(destination_value as usize, source_value);
                    }, 
                    InstructionMode::RegisterIndirect => {
                        let source_register = args[0].get_value(&self.registers, &self.memory);
                        let source_value = self.registers.get_register(source_register as usize);

                        let destination_register = args[1].get_value(&self.registers, &self.memory);
                        let destination_address = self.registers.get_register(destination_register as usize);
                        let destination_value = self.memory.get_memory(destination_address as usize);

                        self.memory.set_memory(destination_value as usize, source_value);
                    },
                    InstructionMode::BaseOffset => {
                        let source_register = args[0].get_value(&self.registers, &self.memory);
                        let source_value = self.registers.get_register(source_register as usize);

                        let destination_register = args[1].get_value(&self.registers, &self.memory);
                        let destination_address = self.registers.get_register(destination_register as usize);

                        let offset = args[2].get_value(&self.registers, &self.memory);

                        let destination_address = destination_address + offset;

                        let destination_value = self.memory.get_memory(destination_address as usize);

                        self.memory.set_memory(destination_value as usize, source_value);
                    }                    
                }
            }
            Instructions::LD => {}
            Instructions::SD16 => {}
            Instructions::LD16 => {}
            Instructions::SD8 => {}
            Instructions::LD8 => {}
            Instructions::LD16S => {}
            Instructions::LD8S => {}

            Instructions::CMP => {
                let register_a = args[0].get_value(&self.registers, &self.memory);
                let register_b = args[1].get_value(&self.registers, &self.memory);

                let register_a = self.registers.get_register(register_a as usize);
                let register_b = self.registers.get_register(register_b as usize);

                if register_a == register_b{
                    self.registers.set_cmp_flag(0x0000);
                }else if register_a != register_b{
                    self.registers.set_cmp_flag(0x0001);
                }else if register_a > register_b{
                    self.registers.set_cmp_flag(0x0002);
                }else if register_a < register_b{
                    self.registers.set_cmp_flag(0x0003);
                }else if register_a >= register_b{
                    self.registers.set_cmp_flag(0x0004);
                }else if register_a <= register_b{
                    self.registers.set_cmp_flag(0x0005);
                }
            }
            Instructions::IF => {}
            Instructions::IFN => {
                if self.registers.get_cmp_flag() == 0x0000{
                    let address = args[2].get_value(&self.registers, &self.memory);

                    // Something to note: we need to divide this number by 4, 
                    // as the program is stored as u8, but we've compacted each instruction into a u32

                    let address = (address / 4);

                    self.registers.set_pc(address as usize);

                    self.has_jumped = true;
                }
            }
            Instructions::IFG => {}
            Instructions::IFL => {}
            Instructions::IFE => {}
            Instructions::IFNE => {}

            Instructions::JMP => {
                let address = args[2].get_value(&self.registers, &self.memory);

                // Something to note: we need to divide this number by 4,
                // as the program is stored as u8, but we've compacted each instruction into a u32
                let address = (address / 4);

                self.registers.set_pc(address as usize);

                self.has_jumped = true;
            }
            Instructions::CALL => {}
            Instructions::RET => {}
        };

        return false;
    }
}