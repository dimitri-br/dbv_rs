use crate::{enum_conv_gen, utils::Parameter, registers::Registers, memory::Memory};

// Instruction Design:
//
// First 8 bits are the instruction (Opcode)
// Next four bits are the mode (Register, Value, Memory)
// The rest of the bits are the arguments
// | 0000_0000 | 0000 | 0000_0000_0000_0000_0000 | -> 32 bits
// |  Opcode   | Mode |         Arguments        |
//
// We get the opcode, mode and arguemnts through bit masking
// Opcode: 0xFF000000
// Mode: 0x00F00000
// Arguments: 0x000FFFFF
//
// It's then up to the instruction to parse the arguments
// and do what it needs to do


enum_conv_gen! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Instructions {
        HLT = 0x0,// Halts the program
        
        PSH,      // Push val to stack. Depending on the mode, val can be a register or a value
        POP,      // Remove from stack and depending on the mode, store in a register or discard
        
        SET,      // Set a register to either a value, register or memory address
        MOV,      // Moves value from register a to register b, or from the memory address defined by register a to register b 
                
        ADD,      // Adds
        SUB,      // Subtracts - based on the mode, subtracts two registers, or a register and a value, or a register and the value at a memory address, storing the result in a register
        MUL,      // Multiplies two registers, or a register and a value, or a register and the value at a memory address, storing the result in a register
        DIV,      // Divides two registers, or a register and a value, or a register and the value at a memory address, storing the result in a register
        
        AND,      // Performs an AND operation
        OR,       // Performs an OR operation
        XOR,      // Performs an XOR operation
        NOT,      // Performs an NOT operation
        SL,       // Performs an SHIFT LEFT operation
        SR,       // Performs an SHIFT RIGHT operation

        MOD,      // Performs an MOD operation (modulo)

        SD,      // Store data into a memory location (32-bit)
        LD,      // Loads data from a memory location (32-bit)
        
        // 16-bit memory operations
        SD16,    // Store data into a memory location (16-bit)
        LD16,    // Loads data from a memory location (16-bit)
        
        // 8-bit memory operations
        SD8,     // Store data into a memory location (8-bit)
        LD8,     // Loads data from a memory location (8-bit)
        
        // Sign-extended load operations
        LD16S,   // Loads 16-bit data from memory and sign-extends to 32-bit
        LD8S,    // Loads 8-bit data from memory and sign-extends to 32-bit    
        
        CMP,      // Compares two registers, setting the flags accordingly
        IF,       // If cmp flag is 0x0001 (==), goto addr 
        IFN,      // If cmp flag is 0x0000 (!=), goto addr 
        IFG,      // If cmp flag is 0x0002 (>), goto addr 
        IFL,      // If cmp flag is 0x0003 (<), goto addr 
        IFE,      // If cmp flag is 0x0004 (>=), goto addr
        IFNE,     // If cmp flag is 0x0005 (<=), goto addr

        JMP,      // Jump to addr 
        CALL,     // Call a function at addr (Must be followed by RET, or undefined behavior)
        
        RET,      // Return from a jump
    }
}

impl Instructions {
    pub fn from_u8(v: u8) -> Option<Self> {
        Self::try_from(v as usize).ok()
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstructionMode {
    Register, // Register - this mode is used for instructions that take a register as a value (eg: ADD R1, R2, R3) - R1 is the destination register, R2 and R3 are the source registers
    Immediate, // Value - this mode is used for instructions that take a value as a value (eg: ADD R1, R2, 0x00000001) - R1 is the destination register, R2 is the source register, 0x00000001 is the value
    RegisterIndirect, // Memory - this mode is used for instructions that take a memory address as a value (eg: ADD R1, R2, [R3]) - R1 is the destination register, R2 is the source register, R3 is the register that contains the memory address
    BaseOffset,
}
