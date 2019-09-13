mod opcodes;

use std::u8;

//Declare some type alias for clarity's sake
pub (crate) type AddressMode = fn(&mut MOS6502) -> (u16,u8);
pub (crate) type OpCode = fn(&mut MOS6502, AddressMode) -> u8;


#[derive(Debug)]
pub struct MOS6502{
                //Registers
    pub (crate) accumulator: u8 ,
    pub (crate) x_register: u8,
    pub (crate) y_register: u8,
    pub (crate) program_counter: u16,
    pub (crate) stack_pointer: u8,
    pub (crate) status_register: u8,
                 //Callbacks
    pub (crate) read: fn(u16) -> u8,
    pub (crate) write: fn(u16, u8),
                //Other
                ///The number of cycles before the next opcode is run
    pub (crate) remaining_cycles: u8
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

    //ADDRESS MODES---------------------------------------------------------------------------------
    //  An address mode function is called by an opcode function, returning a memory address and the
    //  number of extra cycles that may be required under specific circumstances (Typically crossing page boundaries)

    ///Absolute: Address mode returning a 16-bit absolute address
    fn abs(cpu: &mut Self) -> (u16, u8){
        let address: u16 = cpu.read_16(cpu.program_counter);
        cpu.program_counter += 2;
        return (address, 0);
    }

    ///Absolute X: Address mode returning a 16-bit absolute address offset by the x register
    fn abx(cpu: &mut Self) -> (u16, u8){
        let address: u16 = cpu.read_16(cpu.program_counter);
        let offset_address: u16 = address + cpu.x_register as u16;
        let mut extra_cycles = 0;

        if (offset_address) & 0xff00 != address & 0xff00 {
            //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
            extra_cycles = 1;
        }

        cpu.program_counter += 2;
        return (offset_address, extra_cycles);
    }

    ///Absolute Y: Address mode returning a 16-bit absolute address offset by the y register
    fn aby(cpu: &mut Self) -> (u16, u8){
        let address: u16 = cpu.read_16(cpu.program_counter);
        let offset_address: u16 = address + cpu.y_register as u16;
        let mut extra_cycles = 0;

        if (offset_address) & 0xff00 != address & 0xff00 {
            //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
            extra_cycles = 1;
        }

        cpu.program_counter += 2;
        return (offset_address, extra_cycles);
    }

    ///Accumulator: Address mode which operates on the value in the accumulator instead of at a memory address
    ///This is only here for completeness, it should never be called
    // DO NOT USE THIS MODE, instead use multiple implementations of opcode
    // TODO: Investigate a better solution
    fn acc(cpu: &mut Self) -> (u16, u8){
        //Return default values that should never be used
        panic!("Accumulator Address mode was called");
        return (Default::default(), Default::default());
    }

    ///Immediate: Address mode using next byte as value
    fn imm(cpu: &mut Self) -> (u16, u8){
        //Return the current location of the program counter
        let address = cpu.program_counter;
        cpu.program_counter += 1;
        return (address, 0);
    }

    ///Implied: Address mode for opcodes that do not require a value or address
    fn imp(cpu: &mut Self) -> (u16, u8){
        //Return default values that should never be used
        return (Default::default(), Default::default());
    }

    ///Indirect: Address mode that reads from the given address to get the actual address
    fn ind(cpu: &mut Self) -> (u16, u8){
        let (indirect_address, _) = Self::abs(cpu);
        let address: u16;

        //Simulate bug at page edge
        if indirect_address & 0x00ff == 0x00ff{
            address = (cpu.read(indirect_address & 0xff00) as u16) << 8 | cpu.read(indirect_address) as u16;
        } else {
            address = cpu.read_16(indirect_address);
        }

        cpu.program_counter += 2;
        return (address, 0)
    }

    ///Indirect X: Address mode that reads from the 8-bit given address offset by x to get the actual address
    fn izx(cpu: &mut Self) -> (u16, u8){
        let indirect_address = cpu.read(cpu.program_counter);
        let address= cpu.read_16(indirect_address as u16);

        cpu.program_counter += 1;
        return (address, 0);
    }

    ///Indirect Y: Address mode that reads from the 8-bit given address to get the actual address and then offsets it by y
    fn izy(cpu: &mut Self) -> (u16, u8){
        let indirect_address = cpu.read(cpu.program_counter);
        let address= cpu.read_16(indirect_address as u16 + cpu.x_register as u16);
        let offset_address = address + cpu.y_register as u16;
        let mut extra_cycles = 0;

        if (offset_address) & 0xff00 != address & 0xff00 {
            //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
            extra_cycles = 1;
        }

        cpu.program_counter += 1;
        return (offset_address, extra_cycles);
    }

    ///Relative: Address mode used by branch instructions that reads an 8-bit signed relative address to add to the program counter
    fn rel(cpu: &mut Self) -> (u16, u8){
        //This is the same as the immediate address mode, just return the program counter and let the opcode function deal with it
        return Self::imm(cpu);
    }

    ///Zero-page: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__)
    fn zp0(cpu: &mut Self) -> (u16, u8){
        let address = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;
        return(address, 0)
    }

    ///Zero-page X: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__), offset by x
    fn zpx(cpu: &mut Self) -> (u16, u8){
        let address = cpu.read(cpu.program_counter + cpu.x_register as u16);
        cpu.program_counter += 1;
        return(address as u16, 0)
    }

    ///Zero-page Y: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__), offset by y
    fn zpy(cpu: &mut Self) -> (u16, u8){
        let address = cpu.read(cpu.program_counter + cpu.y_register as u16);
        cpu.program_counter += 1;
        return(address as u16, 0)
    }
}

//HELPERS------------------------------------------------------------------------------------------

fn signed_8_bit_to_16(value: u8) -> u16{
    let mut value = value as u16;
    if value & 0x80 > 0{
        value |= 0xff00;
    }
    return value;
}

#[cfg(not(nes))] //These are unneeded in nes mode
fn decimal_add(x: u8, y:u8) -> u8{
    let mut sum = x.wrapping_add(y);
    if (x & 0x0f) + (y & 0x0f) > 0x09{
        sum = sum.wrapping_add(0x06);
    }
    if (sum & 0xf0) > 0x90{
        sum = sum.wrapping_add(0x60);
    }
    return sum
}

#[cfg(not(nes))] //These are unneeded in nes mode
fn decimal_subtract(x: u8, y: u8) -> u8{
    let mut diff = x.wrapping_sub(y);
    if (x & 0x0f) < (y & 0x0f){
        diff = diff.wrapping_sub(0x06);
    }
    if (x & 0xf0) < (y & 0xf0){
        diff = diff.wrapping_sub(0x60);
    }
    return diff
}

#[derive(Copy, Clone)]
pub (crate) enum StatusFlag {
    Carry= 0b00000001,
    Zero = 0b00000010,
    InterruptDisable = 0b00000100,
    Decimal = 0b00001000,
    Overflow = 0b01000000,
    Negative = 0b10000000
}