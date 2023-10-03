mod instructions;
mod memory;
mod registers;
mod utils;
mod vm;

use instructions::*;
use memory::*;
use registers::*;
use utils::*;
use vm::*;

fn main() {  
    let mut virtual_machine = VirtualMachine::new();

    // Load the program
    virtual_machine.load_program("main.dbv").unwrap();

    match virtual_machine.run(){
        Ok(_) => println!("Program exited successfully"),
        Err(e) => println!("Program exited with error: {:?}", e),
    }

    // Print the registers
    virtual_machine.dump();
}
