mod opcodes;
mod address_modes;

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

///The struct representation of the MOS 6502 processor
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
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

    ///Creates a new MOS6502 emulation
    pub fn new() -> MOS6502{
        MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x34,
            remaining_cycles: 0,
            pending_nmi: false,
            pending_irq: false
        }
    }

    ///Runs a processor cycle, mutably borrows the reading and writing interface for the duration
    pub fn cycle(& mut self, interface: &mut (dyn Interface6502)){

        if self.remaining_cycles == 0 {
            if self.pending_nmi || (self.pending_irq && !self.get_flag(StatusFlag::InterruptDisable)) { //An interrupt will let the executing instruction complete
                self.push_stack_16(interface, self.program_counter);
                self.push_stack(interface, self.status_register);
                self.set_flag(StatusFlag::BreakIrq, true);
                self.set_flag(StatusFlag::InterruptDisable, true);

                if self.pending_nmi {
                    self.program_counter = interface.read_16(NMI_ADDRESS_LOCATION);
                    self.remaining_cycles = 8;
                } else {
                    self.program_counter = interface.read_16(IRQ_ADDRESS_LOCATION);
                    self.remaining_cycles = 7;
                }

                self.pending_nmi = false;
                self.pending_irq = false;

            } else { //Proceed normally
                let instruction = opcodes::OPCODE_TABLE[interface.read(self.program_counter) as usize];
                let (address_mode_value, mut extra_cycles) = instruction.find_address(self, interface);
                extra_cycles += instruction.execute_instruction(self, interface, address_mode_value);
                self.remaining_cycles += extra_cycles + instruction.get_cycles();
                self.program_counter += 1;
            }
        }
        self.remaining_cycles -= 1;

    }

    ///Pushes a byte onto the stack
    fn push_stack(&mut self, interface:&mut dyn Interface6502, data: u8){
        interface.write(STACK_PAGE + self.stack_pointer as u16, data);
        self.stack_pointer -= 1;
    }

    ///Pushes two bytes onto the stack
    fn push_stack_16(&mut self, interface:&mut dyn Interface6502, data: u16){
        self.push_stack(interface, (data >> 8) as u8);
        self.push_stack(interface, data as u8);
    }

    ///Pops a byte from the stack
    fn pop_stack(&mut self, interface:&mut dyn Interface6502) -> u8{
        self.stack_pointer += 1;
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
        self.program_counter = interface.read_16(RESET_ADDRESS_LOCATION);

        self.accumulator = 0x00;
        self.x_register = 0x00;
        self.y_register = 0x00;

        self.stack_pointer = 0xFD;
        self.status_register = 0x34;
        self.remaining_cycles = 8;
    }
}

///Trait for interfacing with the 6502
pub trait Interface6502{
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
    //TODO: Make changes so that these can be derived from the other two
    fn read_16(&mut self, address: u16) -> u16;
    fn write_16(&mut self, address: u16, data: u16);
}

#[derive(Copy, Clone)]
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

    fn read_16(&mut self, address: u16) -> u16{
        //Remember little-endianness
        return ((self.read(address + 1) as u16) << 8) | self.read(address) as u16;
    }

    fn write_16(&mut self, address: u16, data: u16){
        //Remember little-endianness
        self.write(address, (data >> 8) as u8);
        self.write(address, data as u8);
    }
}