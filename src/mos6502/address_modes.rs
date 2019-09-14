//ADDRESS MODES---------------------------------------------------------------------------------
//  An address mode function is called by an opcode function, returning a memory address and the
//  number of extra cycles that may be required under specific circumstances (Typically crossing page boundaries)

use super::MOS6502;
use super::AddressModeFunction;
use super::OpcodeFunction;

///Absolute: Address mode returning a 16-bit absolute address
fn absolute(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let address: u16 = cpu.read_16(cpu.program_counter);
    cpu.program_counter += 2;
    return (AddressModeValue::AbsoluteAddress(address), 0);
}

///Absolute X: Address mode returning a 16-bit absolute address offset by the x register
fn absolute_x(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let address: u16 = cpu.read_16(cpu.program_counter);
    let offset_address: u16 = address + cpu.x_register as u16;
    let mut extra_cycles = 0;

    if (offset_address) & 0xff00 != address & 0xff00 {
        //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
        extra_cycles = 1;
    }

    cpu.program_counter += 2;
    return (AddressModeValue::AbsoluteAddress(offset_address), extra_cycles);
}

///Absolute Y: Address mode returning a 16-bit absolute address offset by the y register
fn absolute_y(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let address: u16 = cpu.read_16(cpu.program_counter);
    let offset_address: u16 = address + cpu.y_register as u16;
    let mut extra_cycles = 0;

    if (offset_address) & 0xff00 != address & 0xff00 {
        //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
        extra_cycles = 1;
    }

    cpu.program_counter += 2;
    return (AddressModeValue::AbsoluteAddress(offset_address), extra_cycles);
}

///Accumulator: Address mode which operates on the value in the accumulator instead of at a memory address
fn accumulator(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    return (AddressModeValue::Accumulator, 0);
}

///Immediate: Address mode using next byte as value
fn immediate(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    //Return the current location of the program counter
    let address = cpu.program_counter;
    cpu.program_counter += 1;
    return (AddressModeValue::AbsoluteAddress(address), 0);
}

///Implied: Address mode for opcodes that do not require a value or address
fn implied(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    return (AddressModeValue::Implied, 0);
}

///Indirect: Address mode that reads from the given address to get the actual address
fn indirect(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let indirect_address = cpu.read_16(cpu.program_counter);
    let address: u16;

    //Simulate bug at page edge
    if indirect_address & 0x00ff == 0x00ff{
        address = (cpu.read(indirect_address & 0xff00) as u16) << 8 | cpu.read(indirect_address) as u16;
    } else {
        address = cpu.read_16(indirect_address);
    }

    cpu.program_counter += 2;
    return (AddressModeValue::AbsoluteAddress(address), 0)
}

///Indirect X: Address mode that reads from the 8-bit given address offset by x to get the actual address
fn indirect_x(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let indirect_address = cpu.read(cpu.program_counter);
    let address = cpu.read_16(indirect_address as u16  + cpu.x_register as u16);

    cpu.program_counter += 1;
    return (AddressModeValue::AbsoluteAddress(address), 0);
}

///Indirect Y: Address mode that reads from the 8-bit given address to get the actual address and then offsets it by y
fn indirect_y(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let indirect_address = cpu.read(cpu.program_counter);
    let address= cpu.read_16(indirect_address as u16 + cpu.x_register as u16);
    let offset_address = address + cpu.y_register as u16;
    let mut extra_cycles = 0;

    if (offset_address) & 0xff00 != address & 0xff00 {
        //Offset crossed a page boundary, any opcode using this address mode will take an extra cycle
        extra_cycles = 1;
    }

    cpu.program_counter += 1;
    return (AddressModeValue::AbsoluteAddress(offset_address), extra_cycles);
}

///Relative: Address mode used by branch instructions that reads an 8-bit signed relative address to add to the program counter
fn relative(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let relative_address = cpu.read(cpu.program_counter);
    cpu.program_counter += 1;
    return (AddressModeValue::RelativeAddress(relative_address), 0);
}

///Zero-page: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__)
fn zero_page(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let address = cpu.read(cpu.program_counter) as u16;
    cpu.program_counter += 1;
    return(AddressModeValue::AbsoluteAddress(address), 0)
}

///Zero-page X: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__), offset by x
fn zero_page_x(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let address = cpu.read(cpu.program_counter) + cpu.x_register;
    cpu.program_counter += 1;
    return(AddressModeValue::AbsoluteAddress(address as u16), 0)
}

///Zero-page Y: Address mode that uses an 8-bit address to access memory on the 0 page (0x00__), offset by y
fn zero_page_y(cpu: &mut MOS6502) -> (AddressModeValue, u8){
    let address = cpu.read(cpu.program_counter + cpu.y_register as u16);
    cpu.program_counter += 1;
    return(AddressModeValue::AbsoluteAddress(address as u16), 0)
}

///Enum for the return type of Address modes
pub (crate) enum AddressModeValue {
    Implied,
    Accumulator,
    RelativeAddress(u8),
    AbsoluteAddress(u16)
}