use std::fmt::Debug;

use crate::Memory;
use crate::Registers;
use crate::instructions::InstructionMode;
use crate::instructions::Instructions;


#[macro_export]
macro_rules! enum_conv_gen {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<usize> for $name {
            type Error = ();

            fn try_from(v: usize) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as usize => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Parameter{
    pub value: u32,
}

impl Parameter{
    pub fn get_value(&self, _registers: &Registers, _memory: &Memory) -> u32{
        self.value
    }

    pub fn get_value_enc(&self, _registers: &Registers, _memory: &Memory) -> u32 {
        self.value
    }

    pub fn set_value(&self, _registers: &mut Registers, _memory: &mut Memory, _value: u32){
        panic!("Cannot set value parameter");
    }
}



pub fn decode_instructions(raw_instructions: &[u32]) -> Vec<(Instructions, InstructionMode, Vec<Parameter>)>{
    let mut instructions: Vec<(Instructions, InstructionMode, Vec<Parameter>)> = Vec::new();
    let mut is_extended = false;
    let mut vals: (Instructions, InstructionMode, Vec<Parameter>);

    for raw_instruction in raw_instructions{
        if is_extended{
            // We're extended, so we need to read the next 4 bytes as the value
            // we can also now continue as we've read the value
            let len = instructions.len();
            let value = *raw_instruction;
            instructions[len - 1].2.push(Parameter{value});
            is_extended = false;
            continue;
        }
        // Get the opcode by bit masking
        let opcode = (raw_instruction & 0xFF000000) >> 24;  // 0b1111_1111_0000_0000_0000_0000_0000_0000
        // Get the mode by bit masking
        let mode = (raw_instruction & 0x00F00000) >> 22;    // 0b0000_0000_1111_0000_0000_0000_0000_0000
        // Get the arguments by bit masking
        let arguments = raw_instruction & 0x000FFFFF;       // 0b0000_0000_0000_1111_1111_1111_1111_1111

        // Convert the opcode to an instruction
        let instruction = match Instructions::from_u8(opcode as u8){
            Some(x) => x,
            None => {
                eprintln!("Invalid instruction: {}", opcode);
                std::process::exit(1);
            }
        };

        // Convert the mode to an instruction mode
        let mode = match mode{
            0 => InstructionMode::Register,
            1 => InstructionMode::Immediate,
            2 => InstructionMode::RegisterIndirect,
            3 => InstructionMode::BaseOffset,
            _ => {
                eprintln!("Invalid mode: {}", mode);
                std::process::exit(1);
            }
        };

        vals = match mode {
            InstructionMode::Register => { // This is generally used for instructions that take a register as a value (eg: ADD R1, R2, R3) - R1 is the destination register, R2 and R3 are the source registers
                let destination_register = (arguments & 0xF000) >> 12;
                let src_1_register = (arguments & 0x0F00) >> 8;
                let src_2_register = (arguments & 0x00F0) >> 4;
                // Last 4 bits are unused

                (instruction, mode, vec![Parameter{value: destination_register}, Parameter{value: src_1_register}, Parameter{value: src_2_register}])
            },
            InstructionMode::Immediate => { // This is generally used for instructions that take a value as a value (eg: ADD R1, R2, 0x00000001) - R1 is the destination register, R2 is the source register, 0x00000001 is the value
                // The last bit of args is the extension flag
                // If it's set, then we read the next 4 bytes as the value
                // Otherwise, we read 0xF0 as the value
                let destination_register = (arguments & 0xF000) >> 12;
                let src_1_register = (arguments & 0x0F00) >> 8;
                let mut params = vec![Parameter{value: destination_register}, Parameter{value: src_1_register}];


                let extension_flag = arguments & 0x1;
                if extension_flag == 0x1{
                    is_extended = true; // This will be reset once the extension is read
                }else{
                    let value = (arguments & 0x00F0) >> 4;
                    params.push(Parameter{value});
                }

                (instruction, mode, params)                
            },
            InstructionMode::RegisterIndirect => {
                let destination_register = (arguments & 0xF000) >> 12;
                let src_1_register = (arguments & 0x0F00) >> 8;
                let src_2_register = (arguments & 0x00F0) >> 4;
                // Last 4 bits are unused

                (instruction, mode, vec![Parameter{value: destination_register}, Parameter{value: src_1_register}, Parameter{value: src_2_register}])
            },
            InstructionMode::BaseOffset => { // Used 
                let destination_register = (arguments & 0xF000) >> 12;
                let src_1_register = (arguments & 0x0F00) >> 8;
                let src_2_register = (arguments & 0x00F0) >> 4;
                let offset = (arguments & 0x000F) as u8;

                (instruction, mode, vec![Parameter{value: destination_register}, Parameter{value: src_1_register}, Parameter{value: src_2_register}, Parameter{value: offset as u32}])
            },
        };

        instructions.push(vals);
    }

    instructions
}