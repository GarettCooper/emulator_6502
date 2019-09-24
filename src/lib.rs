//!# 6502-emulator
//!Hello friends, prospective employers, and people who Googled "6502 emulator rust", you've found a small personal project I've been working on since early September of 2019 to use as a talking point during the interview process for my Winter 2020 co-op placement. The goal of the project is to demonstrate my ability to pick up a new programming language while developing a complex system.
//!
//!This is a general purpose Rust implementation of an [MOS 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502) emulator, capable of executing code in isolation or as part of one of the many systems the 6502 was used in, including the Commodore 64, Apple II, and Nintendo Entertainment System. To do so, the library provides the Interface6502 trait which allows the client to implement its own functions for reading and writing to memory addresses.
//!
//!### Defining an interface
//!
//!```rust
//!
//!struct BasicRam{
//!    ram: Box<[u8; u16::max_value() as usize + 1]> //The maximum address range of the 6502
//!}
//!
//!impl BasicRam {
//!    fn load_program(&mut self, start: usize, data: &mut Vec<u8>){
//!        self.ram[start..].clone_from_slice(data);
//!    }
//!}
//!
//!impl Interface6502 for BasicRam{
//!    fn read(&mut self, address: u16) -> u8{
//!        self.ram[address as usize]
//!    }
//!
//!    fn write(&mut self, address: u16, data: u8){
//!        self.ram[address as usize] = data
//!    }
//!}
//!
//!```
//!
//!In this example, the interface to be used with the emulator simply maps addresses to ram locations. The client is responsible for loading the 6502 binary program it wishes to run into an appropriate part of the address range. A more complex interface could map specific addresses to other emulated device components.
//!
//!For example, a NES implementation using this 6502 emulator would map reads and writes to addresses 0x2000-0x2007 to communication with the NES' picture processing unit, while a Commodore 64 implementation would map addresses 0xd000-0xd3ff for drawing to the screen.
//!
//!### Running a program
//!
//!```rust
//!
//!fn main() -> Result<()>{
//!  let mut ram = BasicRam{ ram: Box::new([0; u16::max_value() as usize + 1]) };
//!
//!  //Load a program into memory...
//!  let mut file = File::open("C:/some_6502_program.bin")?;
//!  let mut buffer = Vec::new();
//!  file.read_to_end(&mut buffer)?;
//!
//!  //Copy it into the BasicRam
//!  ram.load_program(0x0400, &mut buffer);
//!
//!  let mut cpu = MOS6502::new(); //Create a new emulator instance
//!  cpu.set_program_counter(0x0400); //Set the program counter to the first byte of the program in memory
//!  cpu.cycle(&mut ram); // The emulator can execute cycles individually, for systems that require precise timing...
//!  cpu.execute_instruction(&mut ram); // or instruction by instruction for a coarser approach
//!
//!  Ok(())
//!}
//!
//!```
//!Each cycle/instruction the processor borrows mutable ownership of the interface in order to read and write to it.
//!
//!NOTE: When an instruction is executed, the entire computation is carried out simultaneously before the processor simply waits for the
//!remaining number of cycles, meaning that timing of reads and writes is only accurate on an instruction-by-instruction basis, not cycle-by-cycle
//!
//!### Supported Features:
//!* Full implementation of documented instruction set
//!* Emulation of bugs that existed in the original 6502 hardware
//!* Binary Coded Decimal when the "binary_coded_decimal" compilation feature is enabled
//!* ... hopefully more if I get around to it
//!
//!Currently undocumented instructions are implemented as placeholder functions which cause the emulator to panic when called.


mod opcodes;
mod address_modes;

#[macro_use]
extern crate log;

use std::u8;
use address_modes::*;

//Declare some type alias for clarity's sake
type AddressModeFunction = fn(&mut MOS6502, &mut dyn Interface6502) -> (address_modes::AddressModeValue, u8);
type OpcodeFunction = fn(&mut MOS6502, &mut dyn Interface6502 , AddressModeValue) -> u8;

///The value that will be added to the stack pointer
const STACK_PAGE: u16 = 0x0100;
///The address that the program counter will be read from when a non-maskable interrupt request is made
const NMI_ADDRESS_LOCATION: u16 = 0xfffa;
///The address that the program counter will be read from when reset is called
const RESET_ADDRESS_LOCATION: u16 = 0xfffa;
///The address that the program counter will be read from when an interrupt request is made or BRK is called
const IRQ_ADDRESS_LOCATION: u16 = 0xfffe;

///Struct representation of the MOS 6502 processor
///
/// ### Usage Example
/// ```
/// fn main() -> Result<()>{
///  let mut ram = BasicRam{ ram: Box::new([0; u16::max_value() as usize + 1]) };
///
///  //Load a program into memory...
///  let mut file = File::open("C:/some_6502_program.bin")?;
///  let mut buffer = Vec::new();
///  file.read_to_end(&mut buffer)?;
///
///  //Copy it into the BasicRam
///  ram.load_program(0x0400, &mut buffer);
///
///  let mut cpu = MOS6502::new(); //Create a new emulator instance
///  cpu.set_program_counter(0x0400); //Set the program counter to the first byte of the program in memory
///  cpu.cycle(&mut ram); // The emulator can execute cycles individually, for systems that require precise timing...
///  cpu.execute_instruction(&mut ram); // or instruction by instruction for a coarser approach
///
///  Ok(())
///}
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct MOS6502{
    //Registers
    accumulator: u8 ,
    x_register: u8,
    y_register: u8,
    program_counter: u16,
    stack_pointer: u8,
    status_register: u8,
    //Other
    ///The number of cycles before the next opcode is run
    remaining_cycles: u8,
    //Tracking Booleans
    pending_nmi: bool,
    pending_irq: bool
}

impl MOS6502{

    ///Creates a new MOS6502 emulation with the program counter at 0x0400
    pub fn new() -> Self{
        MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0400,
            stack_pointer: 0xFD,
            status_register: 0x34,
            remaining_cycles: 0,
            pending_nmi: false,
            pending_irq: false
        }
    }

    ///Creates a new MOS6502 emulation with the program counter at the provided start address
    pub fn new_start(start: u16) -> Self{
        return MOS6502{ program_counter: start,..MOS6502::new() }
    }

    ///Force the program counter to a specific address
    pub fn set_program_counter(&mut self, program_counter: u16){
        self.program_counter = program_counter
    }

    ///Runs a processor cycle, mutably borrows the reading and writing interface for the duration
    pub fn cycle(& mut self, interface: &mut (dyn Interface6502)){

        if self.remaining_cycles == 0 {
            if self.pending_nmi || (self.pending_irq && !self.get_flag(StatusFlag::InterruptDisable)) { //An interrupt will let the executing instruction complete
                //Increase program counter by 1 so it returns to the correct place
                self.push_stack_16(interface, self.program_counter + 1);
                self.set_flag(StatusFlag::BreakIrq, true);
                self.push_stack(interface, self.status_register);
                self.set_flag(StatusFlag::InterruptDisable, true);

                if self.pending_nmi {
                    self.program_counter = read_16(interface, NMI_ADDRESS_LOCATION);
                    self.remaining_cycles = 8;
                } else {
                    self.program_counter = read_16(interface, IRQ_ADDRESS_LOCATION);
                    self.remaining_cycles = 7;
                }

                self.pending_nmi = false;
                self.pending_irq = false;

            } else {
                //Proceed normally
                let instruction = opcodes::OPCODE_TABLE[interface.read(self.program_counter) as usize];
                let log_program_counter = self.program_counter;
                self.program_counter += 1;
                let (address_mode_value, mut extra_cycles) = instruction.find_address(self, interface);

                info!("0x{:04X} {} {:?}", log_program_counter, instruction.get_name(), address_mode_value);

                extra_cycles += instruction.execute_instruction(self, interface, address_mode_value);
                self.remaining_cycles += extra_cycles + instruction.get_cycles();
            }
        }
        self.remaining_cycles -= 1;
    }

    ///Runs as many processor cycles as it takes to complete the instruction at the program counter
    pub fn execute_instruction(& mut self, interface: &mut (dyn Interface6502)){
        self.cycle(interface); //No do-while loops in Rust
        while self.remaining_cycles != 0 {
            self.cycle(interface)
        }
    }

    ///Pushes a byte onto the stack
    fn push_stack(&mut self, interface:&mut dyn Interface6502, data: u8){
        interface.write(STACK_PAGE + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    ///Pushes two bytes onto the stack
    fn push_stack_16(&mut self, interface:&mut dyn Interface6502, data: u16){
        self.push_stack(interface, (data >> 8) as u8);
        self.push_stack(interface, data as u8);
    }

    ///Pops a byte from the stack
    fn pop_stack(&mut self, interface:&mut dyn Interface6502) -> u8{
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        return interface.read(STACK_PAGE + self.stack_pointer as u16);
    }

    ///Pops two bytes from the stack
    fn pop_stack_16(&mut self, interface:&mut dyn Interface6502) -> u16{
        let lo = self.pop_stack(interface) as u16;
        let hi = self.pop_stack(interface) as u16;
        return (hi << 8) | lo;
    }

    ///Sets a status flag to the given boolean value
    fn set_flag(&mut self, flag: StatusFlag, value: bool){
        //Clear flag
        self.status_register &= !(flag as u8);
        //TODO: Work out a branch free method of doing this, possibly converting flag values to bit index
        if value {
            self.status_register |= flag as u8
        }
    }

    ///Returns the value of a flag in the status register as a boolean
    fn get_flag(&self, flag: StatusFlag) -> bool{
        return (self.status_register & flag as u8) > 0;
    }

    ///Request that an interrupt occurs after the current instruction completes
    pub fn interrupt_request(&mut self){
        self.pending_irq = true;
    }

    ///Request that an interrupt occurs after the current instruction completes, even if the interrupt disabled flag is set
    pub fn non_maskable_interrupt_request(&mut self){
        self.pending_nmi = true;
    }

    ///Resets the 6502 to a known state
    pub fn reset(&mut self, interface: &mut dyn Interface6502){
        self.program_counter = read_16(interface, RESET_ADDRESS_LOCATION);

        self.accumulator = 0x00;
        self.x_register = 0x00;
        self.y_register = 0x00;

        self.stack_pointer = 0xFD;
        self.status_register = 0x34;
        self.remaining_cycles = 8;
    }
}

///Wrapper function for reading 16 bits at a time
fn read_16(bus: &mut dyn Interface6502, address: u16) -> u16{
    let lo = bus.read(address) as u16;
    let hi = bus.read(address + 1) as u16;
    return (hi << 8) | lo;
}

///Wrapper function for writing 16 bits at a time
fn write_16(bus: &mut dyn Interface6502, address: u16, data: u16){
    bus.write(address, data as u8);
    bus.write(address + 1, (data >> 8) as u8);
}

///Trait for interfacing with the 6502
///
/// ### Declaration Example
/// ```
/// struct BasicRam{
///    ram: Box<[u8; u16::max_value() as usize + 1]> //The maximum address range of the 6502
///}
///
///impl BasicRam {
///    fn load_program(&mut self, start: usize, data: &mut Vec<u8>){
///        self.ram[start..].clone_from_slice(data);
///    }
///}
///
///impl Interface6502 for BasicRam{
///    fn read(&mut self, address: u16) -> u8{
///        self.ram[address as usize]
///    }
///
///    fn write(&mut self, address: u16, data: u8){
///        self.ram[address as usize] = data
///    }
///}
/// ```
pub trait Interface6502{
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

#[derive(Debug, Copy, Clone)]
enum StatusFlag {
    Carry= 0b00000001,
    Zero = 0b00000010,
    InterruptDisable = 0b00000100,
    Decimal = 0b00001000,
    Break = 0b00110000,
    BreakIrq = 0b00100000,
    Overflow = 0b01000000,
    Negative = 0b10000000
}

#[cfg(test)] //This default implementation is only to make testing easier, and should not be exposed
impl Default for MOS6502{
    fn default() -> Self{
        MOS6502::new()
    }
}

#[cfg(test)]
pub (crate) struct StubInterface6502{
    read: fn(u16) -> u8,
    write: fn(u16, u8)
}

#[cfg(test)]
impl StubInterface6502{
    pub (crate) fn new(read_fn: fn(u16) -> u8, write_fn: fn(u16, u8)) -> Self{
        StubInterface6502{ read: read_fn, write: write_fn }
    }
}

#[cfg(test)]
impl Interface6502 for StubInterface6502{
    fn read(&mut self, address: u16) -> u8{
        (self.read)(address)
    }

    fn write(&mut self, address: u16, data: u8){
        (self.write)(address, data)
    }
}