const MEMORY_SIZE: usize = 0xFFFFFF;

pub struct Memory{
    memory: Vec<u8>,
}

impl Memory{
    pub fn new() -> Self{
        Memory{
            memory: vec![0x1; MEMORY_SIZE],
        }
    }

    pub fn get_memory(&self, address: usize) -> u32{
        let mut value: u32 = 0;
        for i in 0..4{
            value |= (self.memory[address + i] as u32) << (i * 8);
        }
        value
    }

    pub fn get_memory_u16(&self, address: usize) -> u32{
        let mut value: u16 = 0;
        for i in 0..2{
            // Little Endian
            value |= (self.memory[address + i] as u16) << (i * 8);
        }

        value as u32
    }

    pub fn get_memory_u8(&self, address: usize) -> u32{
        self.memory[address] as u32
    }

    pub fn get_memory_u16_signed(&self, address: usize) -> u32{
        // We don't read it as signed, but we return it as signed
        // We do this by extending the sign bit to 32 bits
        let mut value: u32 = 0;
        for i in 0..2{
            // Little Endian
            value |= (self.memory[address + i] as u32) << (i * 8);
        }

        let sign_bit = (value & 0x8000) >> 15;
        value |= sign_bit << 16;

        value as u32
    }

    pub fn get_memory_u8_signed(&self, address: usize) -> u32{
        // We don't read it as signed, but we return it as signed
        // We do this by extending the sign bit to 32 bits
        let mut value = self.memory[address] as u32;

        let sign_bit = (value & 0x80) >> 7;
        value |= sign_bit << 8;

        value as u32
    }

    pub fn set_memory(&mut self, address: usize, value: u32){
        for i in 0..4{
            self.memory[address + i] = ((value >> (i * 8)) & 0xFF) as u8;
        }
    }

    pub fn set_memory_u16(&mut self, address: usize, value: u32){
        for i in 0..2{
            // Little Endian
            self.memory[address + i] = ((value >> (i * 8)) & 0xFF) as u8;
        }
    }

    pub fn set_memory_u8(&mut self, address: usize, value: u32){
        self.memory[address] = (value & 0xFF) as u8;
    }
}


pub fn test_memory() -> bool{
    // Test setting, loading, and signed loading
    
    let mut memory = Memory::new();

    // Setting
    memory.set_memory(0x000000, 0x12345678);
    memory.set_memory(0x000004, 0x87654321);
    memory.set_memory(0x000008, 0x00000000);
    memory.set_memory(0x00000C, 0xFFFFFFFF);

    // 16 bit setting
    memory.set_memory_u16(0x000010, 0x1234);
    memory.set_memory_u16(0x000012, 0x5678);
    memory.set_memory_u16(0x000014, 0x0000);
    memory.set_memory_u16(0x000016, 0xFFFF);

    // 8 bit setting
    memory.set_memory_u8(0x000018, 0x12);
    memory.set_memory_u8(0x000019, 0x34);
    memory.set_memory_u8(0x00001A, 0x56);
    memory.set_memory_u8(0x00001B, 0x78);

    // Loading
    assert_eq!(memory.get_memory(0x000000), 0x12345678);
    assert_eq!(memory.get_memory(0x000004), 0x87654321);
    assert_eq!(memory.get_memory(0x000008), 0x00000000);
    assert_eq!(memory.get_memory(0x00000C), 0xFFFFFFFF);

    // 16 bit loading
    assert_eq!(memory.get_memory_u16(0x000010), 0x1234);
    assert_eq!(memory.get_memory_u16(0x000012), 0x5678);
    assert_eq!(memory.get_memory_u16(0x000014), 0x0000);
    assert_eq!(memory.get_memory_u16(0x000016), 0xFFFF);

    // 8 bit loading
    assert_eq!(memory.get_memory_u8(0x000018), 0x12);
    assert_eq!(memory.get_memory_u8(0x000019), 0x34);
    assert_eq!(memory.get_memory_u8(0x00001A), 0x56);
    assert_eq!(memory.get_memory_u8(0x00001B), 0x78);

    // Set signed values. Then load and check they're properly extended
    memory.set_memory_u16(0x00001C, 0x8000); // 0b1000000000000000 -> should be -32768
    memory.set_memory_u16(0x00001E, 0x7FFF); // 0b0111111111111111 -> should be 32767
    memory.set_memory_u8(0x000020, 0x80); // 0b10000000 -> should be -128
    memory.set_memory_u8(0x000021, 0x7F); // 0b01111111 -> should be 127

    // Load signed values - load them first, then check they're properly extended. 
    // Convert to i32 to check the sign bit
    assert_eq!(memory.get_memory_u16_signed(0x00001C) as u16 as i16, -32768);
    assert_eq!(memory.get_memory_u16_signed(0x00001E) as u16 as i16, 32767);
    assert_eq!(memory.get_memory_u8_signed(0x000020) as u8 as i8, -128);
    assert_eq!(memory.get_memory_u8_signed(0x000021) as u8 as i8, 127);

    true
}