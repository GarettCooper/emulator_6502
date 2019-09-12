use std::u8;

//Declare some type alias for clarity's sake
type AddressMode = fn(&mut MOS6502) -> (u16,u8);
type OpCode = fn(&mut MOS6502, AddressMode) -> u8;


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
    write: fn(u16, u8),
    read: fn(u16) -> u8,
    //Other
    ///The number of cycles before the next opcode is run
    remaining_cycles: u8
}

impl Default for MOS6502{
    fn default() -> Self {
        MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x34,
            write: |address, data | {println!("Write callback not set!")},
            read: |data| {println!("Read callback not set!"); return Default::default()},
            remaining_cycles: 0
        }
    }
}


impl MOS6502{

    ///Creates a new MOS6502 emulation
    pub fn new() -> MOS6502{
        Default::default()
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
    fn abs(this: &mut Self) -> (u16, u8){
        let address: u16 = this.read_16(this.program_counter);
        this.program_counter += 2;
        return (address, 0);
    }

    ///Absolute X: Address mode returning a 16-bit absolute address offset by the x register
    fn abx(this: &mut Self) -> (u16, u8){
        let address: u16 = this.read_16(this.program_counter);
        let offset_address: u16 = address + this.x_register as u16;
        let mut extra_cycles = 0;

        if (offset_address) & 0xff00 != address & 0xff00 {
            //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
            extra_cycles = 1;
        }

        this.program_counter += 2;
        return (offset_address, extra_cycles);
    }

    ///Absolute Y: Address mode returning a 16-bit absolute address offset by the y register
    fn aby(this: &mut Self) -> (u16, u8){
        let address: u16 = this.read_16(this.program_counter);
        let offset_address: u16 = address + this.y_register as u16;
        let mut extra_cycles = 0;

        if (offset_address) & 0xff00 != address & 0xff00 {
            //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
            extra_cycles = 1;
        }

        this.program_counter += 2;
        return (offset_address, extra_cycles);
    }

    ///Accumulator: Address mode which operates on the value in the accumulator instead of at a memory address
    ///This is only here for completeness, it should never be called
    // DO NOT USE THIS MODE, instead use multiple implementations of opcode
    // TODO: Investigate a better solution
    fn acc(this: &mut Self) -> (u16, u8){
        //Return default values that should never be used
        panic!("Accumulator Address mode was called");
        return (Default::default(), Default::default());
    }

    ///Immediate: Address mode using next byte as value
    fn imm(this: &mut Self) -> (u16, u8){
        //Return the current location of the program counter
        let address = this.program_counter;
        this.program_counter += 1;
        return (address, 0);
    }

    ///Implied: Address mode for opcodes that do not require a value or address
    fn imp(this: &mut Self) -> (u16, u8){
        //Return default values that should never be used
        return (Default::default(), Default::default());
    }

    ///Indirect: Address mode that reads from the given address to get the actual address
    fn ind(this: &mut Self) -> (u16, u8){
        let (indirect_address, _) = Self::abs(this);
        let address: u16;

        //Simulate bug at page edge
        if indirect_address & 0x00ff == 0x00ff{
            address = (this.read(indirect_address & 0xff00) as u16) << 8 | this.read(indirect_address) as u16;
        } else {
            address = this.read_16(indirect_address);
        }

        this.program_counter += 2;
        return (address, 0)
    }

    ///Indirect X: Address mode that reads from the 8-bit given address offset by x to get the actual address
    fn izx(this: &mut Self) -> (u16, u8){
        let indirect_address = this.read(this.program_counter);
        let address= this.read_16(indirect_address as u16);

        this.program_counter += 1;
        return (address, 0);
    }

    ///Indirect Y: Address mode that reads from the 8-bit given address to get the actual address and then offsets it by y
    fn izy(this: &mut Self) -> (u16, u8){
        let indirect_address = this.read(this.program_counter);
        let address= this.read_16(indirect_address as u16 + this.x_register as u16);
        let offset_address = address + this.y_register as u16;
        let mut extra_cycles = 0;

        if (offset_address) & 0xff00 != address & 0xff00 {
            //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
            extra_cycles = 1;
        }

        this.program_counter += 1;
        return (offset_address, extra_cycles);
    }

    ///Relative: Address mode used by branch instructions that reads an 8-bit signed relative address to add to the program counter
    fn rel(this: &mut Self) -> (u16, u8){
        //This is the same as the immediate address mode, just return the program counter and let the opcode function deal with it
        return Self::imm(this);
    }

    ///Zero-page: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__)
    fn zp0(this: &mut Self) -> (u16, u8){
        let address = this.read(this.program_counter) as u16;
        this.program_counter += 1;
        return(address, 0)
    }

    ///Zero-page X: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__), offset by x
    fn zpx(this: &mut Self) -> (u16, u8){
        let address = this.read(this.program_counter + this.x_register as u16);
        this.program_counter += 1;
        return(address as u16, 0)
    }

    ///Zero-page Y: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__), offset by y
    fn zpy(this: &mut Self) -> (u16, u8){
        let address = this.read(this.program_counter + this.y_register as u16);
        this.program_counter += 1;
        return(address as u16, 0)
    }

    //OPCODES---------------------------------------------------------------------------------------
    //  An opcode function represents one of the 6502's opcodes. An opcode function is passed the
    //  address mode to use and returns the number of extra cycles that address mode has taken

    ///ADC: Adds a value and the carry bit to the accumulator, returns the number of additional cycles
    ///     the operation will take
    fn adc(this: &mut Self, address_mode: AddressMode) -> u8{

        let (address, additional_cycles) = address_mode(this);
        let value = this.read(address);

        let result: u16;

        //Only run if the CPU is not built in NES mode
        //TODO: Make sure this is removed as dead code in nes builds
        if cfg!(not(nes)) && this.get_flag(StatusFlag::Decimal){
            let mut sum = this.accumulator.wrapping_add(value);
            if (this.accumulator & 0x0f) + (value & 0x0f) > 0x09{
                sum = sum.wrapping_add(0x06);
            }
            if (sum & 0xf0) > 0x90{
                sum = sum.wrapping_add(0x60);
                this.set_flag(StatusFlag::Carry, true);
            }
            result = sum as u16;
        } else {
            result = this.accumulator as u16 + value as u16 + this.get_flag(StatusFlag::Carry) as u16;

            //Set the Carry flag for chain adding multi byte numbers
            this.set_flag(StatusFlag::Carry, result > u8::max_value() as u16);
        }
        //TODO: Verify that these flags are set correctly in decimal mode
        this.set_flag(StatusFlag::Zero, result == 0);

        //Set the Overflow flag if a signed overflow has occurred
        this.set_flag(StatusFlag::Overflow, (!(this.accumulator ^ value) & (this.accumulator ^ result as u8) & StatusFlag::Overflow as u8) > 0);

        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        this.set_flag(StatusFlag::Negative, result as u8 & StatusFlag::Negative as u8 > 0);

        this.accumulator = result as u8;

        return additional_cycles;
    }

    ///AND: Performs a logical and with the accumulator and the addressed value, storing the result
    ///     in the accumulator
    fn and(this: &mut Self, address_mode: AddressMode) -> u8{
        let (address, additional_cycles) = address_mode(this);
        let value = this.read(address);

        this.accumulator &= value;

        this.set_flag(StatusFlag::Zero, this.accumulator == 0);

        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        this.set_flag(StatusFlag::Negative, this.accumulator & StatusFlag::Negative as u8 > 0);

        return additional_cycles;
    }

    ///ASL: Performs a left bit shift on the addressed value
    fn asl(this: &mut Self, address_mode: AddressMode) -> u8{
        let (address, additional_cycles) = address_mode(this);
        let mut value = this.read(address);

        //Store the 7th bit in the carry bit
        this.set_flag(StatusFlag::Carry, value >> 7 == 1);
        value <<= 1;
        this.write(address, value);

        return additional_cycles; //Should always be 0
    }

    ///ASL A: Performs a left bit shift on the accumulator
    fn asl_a(this: &mut Self, address_mode: AddressMode) -> u8{
        let mut value = this.accumulator;

        //Store the 7th bit in the carry bit
        this.set_flag(StatusFlag::Carry, value >> 7 == 1);
        value <<= 1;
        this.accumulator = value;

        return 0; //Should always be 0
    }

    ///BCC: Branch if the carry bit is clear
    fn bcc(this: &mut Self, address_mode: AddressMode) -> u8{
        let (relative_address, additional_cycles) = address_mode(this);
        let address = signed_8_bit_to_16(this.read(relative_address)) + this.program_counter;
        let mut extra_cycles = 0;

        if !this.get_flag(StatusFlag::Carry){
            if address & 0xff00 != this.program_counter & 0xff00{
                extra_cycles += 2;
            } else {
                extra_cycles += 1;
            }

            this.program_counter = address;
        }

        return extra_cycles;
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
pub enum StatusFlag {
    Carry= 0b00000001,
    Zero = 0b00000010,
    InterruptDisable = 0b00000100,
    Decimal = 0b00001000,
    Overflow = 0b01000000,
    Negative = 0b10000000
}