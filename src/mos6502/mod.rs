mod opcodes;
mod address_modes;

use std::u8;

//Declare some type alias for clarity's sake
type AddressModeFunction = fn(&mut MOS6502) -> (u16, u8);
type OpcodeFunction = fn(&mut MOS6502, AddressModeFunction) -> u8;


#[derive(Debug)]
pub struct MOS6502{
    //Registers
    accumulator: u8 ,
    x_register: u8,
    y_register: u8,
    program_counter: u16,
    stack_pointer: u8,
    status_register: u8,
    //Callbacks
    read: fn(u16) -> u8,
    write: fn(u16, u8),
    //Other
    ///The number of cycles before the next opcode is run
    remaining_cycles: u8
}

impl MOS6502{

    ///Creates a new MOS6502 emulation
    pub fn new(read_fn: fn(u16) -> u8, write_fn: fn(u16, u8)) -> MOS6502{
        MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x34,
            read: read_fn,
            write: write_fn,
            remaining_cycles: 0
        }
    }

    ///Sets the function that will be called when the processor writes to an address
    pub fn set_write_callback(&mut self, callback :fn(u16, u8)){
        self.write = callback;
    }

    ///Sets the function that will be called when the processor reads from an address
    pub fn set_read_callback(&mut self, callback :fn(u16) -> u8){
        self.read = callback;
    }

    ///Runs a processor cycle
    pub fn cycle(&mut self){

        if self.remaining_cycles == 0 {
            //pre-increment program counter
            self.program_counter += 1;
        }
        self.remaining_cycles -= 1;
    }

    fn write(&self, address: u16, data: u8){
        (self.write)(address, data);
    }

    ///Wraps the read function provided passed at creation
    fn read(&self, address: u16) -> u8{
        return (self.read)(address);
    }

    ///Wrapper to return 16 bits from the read function instead of 8
    fn read_16(&self, address: u16) -> u16{
        //Remember little-endianness
        return ((self.read(address + 1) as u16) << 8) | self.read(address) as u16;
    }

    fn set_flag(&mut self, flag: StatusFlag, value: bool){
        //Clear flag
        self.status_register &= !(flag as u8);
        //TODO: Work out a branch free method of doing this, possibly converting flag values to bit index
        if value {
            self.status_register |= flag as u8
        }
    }

    fn get_flag(&self, flag: StatusFlag) -> bool{
        return (self.status_register & flag as u8) > 0;
    }

    pub fn reset(&mut self){
        self.accumulator = 0x00;
        self.x_register = 0x00;
        self.y_register = 0x00;
        self.program_counter = 0x0000;
        self.stack_pointer = 0xFD;
        self.status_register = 0x34;
    }
}

#[derive(Copy, Clone)]
enum StatusFlag {
    Carry= 0b00000001,
    Zero = 0b00000010,
    InterruptDisable = 0b00000100,
    Decimal = 0b00001000,
    Overflow = 0b01000000,
    Negative = 0b10000000
}