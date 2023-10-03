#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register{
    pub register: usize,
    pub value: u32,
}

impl Register{
    pub fn from_u8(v: u32) -> Self {
        Self{
            register: v as usize,
            value: 0,
        }
    }
}

pub struct Registers{
    pub registers: [Register; 16], // 16 32-bit general purpose registers

    // Program Counter
    pc: usize, // Points to the current instruction

    // Stack Pointer
    sp: usize, // Points to the top of the stack



    // Flags - NOTE - These are not the actual flags, 
    // but rather the bitmasks to see what has been set
    // In a real CPU, these would be set in a register

    // Compare Flag
    cmp_flag: u8, // Bitmask to see what has been set
    // 0x0000 = ==
    // 0x0001 = !=
    // 0x0002 = >
    // 0x0003 = <
    // 0x0004 = >=
    // 0x0005 = <=
    // 0x0006 = undefined
    // 0x0007 = undefined

    // Arithmetic Flag
    arith_flag: u8, // Bitmask to see what has been set
    // 0x0000 = Negative
    // 0x0001 = Zero
    // 0x0002 = Carry
    // 0x0003 = Overflow
    // 0x0004 = undefined
    // 0x0005 = undefined
    // 0x0006 = undefined
    // 0x0007 = undefined

    // Interrupt Flag
    interrupt_flag: u8, // Bitmask to see what has been set
    // 0x0000 = undefined
    // 0x0001 = undefined
    // 0x0002 = undefined
    // 0x0003 = undefined
    // 0x0004 = undefined
    // 0x0005 = undefined
    // 0x0006 = undefined
    // 0x0007 = undefined
}

impl Registers{
    pub fn new() -> Self{
        let registers = [Register::from_u8(0); 16];

        Registers{
            registers,
            pc: 0,
            sp: 0,
            cmp_flag: 0,
            arith_flag: 0,
            interrupt_flag: 0,
        }
    }

    pub fn set_register(&mut self, register: usize, value: u32){
        self.registers[register].value = value;
    }

    pub fn get_register(&self, register: usize) -> u32{
        self.registers[register].value
    }

    pub fn set_pc(&mut self, value: usize){
        self.pc = value;
    }

    pub fn get_pc(&self) -> usize{
        self.pc
    }

    pub fn set_sp(&mut self, value: usize){
        self.sp = value;
    }

    pub fn get_sp(&self) -> usize{
        self.sp
    }

    pub fn set_cmp_flag(&mut self, value: u8){
        self.cmp_flag = value;
    }

    pub fn get_cmp_flag(&self) -> u8{
        self.cmp_flag
    }
}
