///OPCODES---------------------------------------------------------------------------------------
///  This module contains all of the opcode functions to prevent the parent module from being primarily full of them
///  An opcode function represents one of the 6502's opcodes. An opcode function is passed the
///  address mode to use and returns the number of extra cycles that address mode has taken

use super::MOS6502;
use super::StatusFlag;
use super::AddressMode;


//TODO: Create static opcode table

///ADC: Adds a value and the carry bit to the accumulator, returns the number of additional cycles
///     the operation will take
fn adc(cpu: &mut MOS6502, address_mode: AddressMode) -> u8{

    let (address, additional_cycles) = address_mode(cpu);
    let value = cpu.read(address);

    let result: u16;

    //Only run if the CPU is not built in NES mode
    //TODO: Make sure cpu is removed as dead code in nes builds
    if cfg!(not(nes)) && cpu.get_flag(StatusFlag::Decimal){
        let mut sum = cpu.accumulator.wrapping_add(value);
        if (cpu.accumulator & 0x0f) + (value & 0x0f) > 0x09{
            sum = sum.wrapping_add(0x06);
        }
        if (sum & 0xf0) > 0x90{
            sum = sum.wrapping_add(0x60);
            cpu.set_flag(StatusFlag::Carry, true);
        }
        result = sum as u16;
    } else {
        result = cpu.accumulator as u16 + value as u16 + cpu.get_flag(StatusFlag::Carry) as u16;

        //Set the Carry flag for chain adding multi byte numbers
        cpu.set_flag(StatusFlag::Carry, result > u8::max_value() as u16);
    }
    //TODO: Verify that these flags are set correctly in decimal mode
    cpu.set_flag(StatusFlag::Zero, result == 0);

    //Set the Overflow flag if a signed overflow has occurred
    cpu.set_flag(StatusFlag::Overflow, (!(cpu.accumulator ^ value) & (cpu.accumulator ^ result as u8) & StatusFlag::Overflow as u8) > 0);

    //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
    cpu.set_flag(StatusFlag::Negative, result as u8 & StatusFlag::Negative as u8 > 0);

    cpu.accumulator = result as u8;

    return additional_cycles;
}

///AND: Performs a logical and with the accumulator and the addressed value, storing the result
///     in the accumulator
fn and(cpu: &mut MOS6502, address_mode: AddressMode) -> u8{
    let (address, additional_cycles) = address_mode(cpu);
    let value = cpu.read(address);

    cpu.accumulator &= value;

    cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);

    //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
    cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);

    return additional_cycles;
}

///ASL: Performs a left bit shift on the addressed value
fn asl(cpu: &mut MOS6502, address_mode: AddressMode) -> u8{
    let (address, additional_cycles) = address_mode(cpu);
    let mut value = cpu.read(address);

    //Store the 7th bit in the carry bit
    cpu.set_flag(StatusFlag::Carry, value >> 7 == 1);
    value <<= 1;
    cpu.write(address, value);

    return additional_cycles; //Should always be 0
}

///ASL A: Performs a left bit shift on the accumulator
fn asl_a(cpu: &mut MOS6502, address_mode: AddressMode) -> u8{
    let mut value = cpu.accumulator;

    //Store the 7th bit in the carry bit
    cpu.set_flag(StatusFlag::Carry, value >> 7 == 1);
    value <<= 1;
    cpu.accumulator = value;

    return 0; //Should always be 0
}

///BCC: Branch if the carry bit is clear
fn bcc(cpu: &mut MOS6502, address_mode: AddressMode) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Carry), address_mode)
}

///BCC: Branch if the carry bit is set
fn bcs(cpu: &mut MOS6502, address_mode: AddressMode) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Carry), address_mode)
}

///BEQ: Branch if the zero bit is set
fn beq(cpu: &mut MOS6502, address_mode: AddressMode) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Zero), address_mode)
}

//HELPERS------------------------------------------------------------------------------------------

///Function to convert a byte to a u16 when the value is signed
fn signed_8_bit_to_16(value: u8) -> u16{
    let mut value = value as u16;
    if value & 0x80 > 0{
        value |= 0xff00;
    }
    return value;
}

///General purpose function for branch opcodes
fn branch(cpu: &mut MOS6502, branch_condition: bool, address_mode: AddressMode) -> u8{
    let (relative_address, additional_cycles) = address_mode(cpu);
    let address = signed_8_bit_to_16(cpu.read(relative_address)) + cpu.program_counter;
    let mut extra_cycles = 0;

    if branch_condition{
        if address & 0xff00 != cpu.program_counter & 0xff00{
            extra_cycles += 2;
        } else {
            extra_cycles += 1;
        }

        cpu.program_counter = address;
    }

    return extra_cycles;
}