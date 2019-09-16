///OPCODES---------------------------------------------------------------------------------------
///  This module contains all of the opcode functions to prevent the parent module from being primarily full of them
///  An opcode function represents one of the 6502's opcodes. An opcode function is passed the
///  address mode to use and returns the number of extra cycles that address mode has taken

use super::MOS6502;
use super::StatusFlag;
use super::AddressModeFunction;
use super::OpcodeFunction;
use super::address_modes::AddressModeValue;

pub (crate) struct Opcode{
    function: OpcodeFunction,
    address_mode: AddressModeFunction,
    cycles: u8
}

//static OPCODE_TABLE: [Opcode; 256] = [
    //TODO: Create static opcode table
    //Opcode{ function: brk, address_mode: super::address_modes::, cycles: 7 } //0x00
//];



///ADC: Adds a value and the carry bit to the accumulator
fn adc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{

    let value;

    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        value = cpu.read(address)
    } else {
        panic!("ADC opcode called with invalid address mode!")
    }

    let result: u16;

    //Only run if the CPU is not built in NES mode
    //TODO: Make sure cpu is removed as dead code in nes builds
    if cfg!(not(nes)) && cpu.get_flag(StatusFlag::Decimal){
        let mut sum = cpu.accumulator.wrapping_add(value).wrapping_add(cpu.get_flag(StatusFlag::Carry) as u8);
        if (cpu.accumulator & 0x0f) + (value & 0x0f) + cpu.get_flag(StatusFlag::Carry) as u8 > 0x09{
            sum = sum.wrapping_add(0x06);
        }
        if (sum & 0xf0) > 0x90{
            sum = sum.wrapping_add(0x60);
            cpu.set_flag(StatusFlag::Carry, true);
        } else {
            cpu.set_flag(StatusFlag::Carry, false);
        }
        result = sum as u16;
    } else {
        result = cpu.accumulator as u16 + value as u16 + cpu.get_flag(StatusFlag::Carry) as u16;

        //Set the Carry flag for chain adding multi byte numbers
        cpu.set_flag(StatusFlag::Carry, result > u8::max_value() as u16);
    }
    //TODO: Verify that these flags are set correctly in decimal mode
    cpu.set_flag(StatusFlag::Zero, result as u8 == 0);

    //Set the Overflow flag if a signed overflow has occurred
    cpu.set_flag(StatusFlag::Overflow, (!(cpu.accumulator ^ value) & (cpu.accumulator ^ result as u8) & StatusFlag::Negative as u8) > 0);

    //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
    cpu.set_flag(StatusFlag::Negative, result as u8 & StatusFlag::Negative as u8 > 0);

    cpu.accumulator = result as u8;

    return 0; //Operation never adds any extra cycles
}

///AND: Performs a logical and with the accumulator and the addressed value, storing the result
///     in the accumulator
fn and(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    let (address, additional_cycles) = address_mode(cpu);
    let value = cpu.read(address);

    cpu.accumulator &= value;

    cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);

    //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
    cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);

    return additional_cycles;
}

///ASL: Performs a left bit shift on the addressed value
fn asl(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    let (address, additional_cycles) = address_mode(cpu);
    let mut value = cpu.read(address);

    //Store the 7th bit in the carry bit
    cpu.set_flag(StatusFlag::Carry, value >> 7 == 1);
    value <<= 1;
    cpu.write(address, value);

    return additional_cycles; //Should always be 0
}

///ASL A: Performs a left bit shift on the accumulator
fn asl_a(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    let mut value = cpu.accumulator;

    //Store the 7th bit in the carry bit
    cpu.set_flag(StatusFlag::Carry, value >> 7 == 1);
    value <<= 1;
    cpu.accumulator = value;

    return 0; //Should always be 0
}

///BCC: Branch if the carry bit is clear
fn bcc(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Carry), address_mode)
}

///BCC: Branch if the carry bit is set
fn bcs(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Carry), address_mode)
}

///BEQ: Branch if the zero bit is set (branch if equal)
fn beq(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Zero), address_mode)
}

///BIT: Uses the accumulator as a mask pattern to test the bits of a given memory location
fn bit(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///BMI: Branch if the negative bit is set (branch if negative)
fn bmi(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///BNE: Branch if the zero bit is clear (branch if not equal)
fn bne(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///BPL: Branch if the negative bit is clear (branch if positive)
fn bpl(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///BRK: Force an interrupt
fn brk(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///BVC: Branch if the overflow bit is clear
fn bvc(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///BVS: Branch if the overflow bit is set
fn bvs(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///CLC: Clear carry bit
fn clc(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///CLD: Clear decimal mode bit
fn cld(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///CLD: Clear interrupt disable bit
fn cli(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///CLD: Clear overflow bit
fn clv(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///CMP: Compare accumulator to a value in memory
fn cmp(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///CPX: Compare x register to a value in memory
fn cpx(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///CPY: Compare y register to a value in memory
fn cpy(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///DEC: Subtract one from the value at the given memory location
fn dec(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///DEC: Subtract one from the x register
fn dex(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///DEY: Subtract one from the y register
fn dey(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///EOR: Set accumulator to the result of an exclusive or operation with the accumulator and a value from memory
fn eor(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///INC: Add one to the value at the given memory location
fn inc(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///INX: Add one to the x register
fn inx(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///DEC: Add one to the y register
fn iny(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///JMP: Set the program counter to the given address
fn jmp(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///JSR: Puts the current program counter value on the stack and then jumps to the given address
fn jsr(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///LDA: Load a value into the accumulator from a memory address
fn lda(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///LDX: Load a value into the x register from a memory address
fn ldx(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///LDY: Load a value into the y register from a memory address
fn ldy(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///LSR: Performs a right bit shift on the given value
fn lsr(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///NOP: No operation
fn nop(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///ORA: The accumulator is set to the result of a inclusive or operation applied to the accumulator and a memory value
fn ora(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///PHA: Push the value of the accumulator onto the stack
fn pha(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///PHP: Push the value of the status byte onto the stack
fn php(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///PLA: Sets the accumulator to a value popped off the top of the stack
fn pla(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///PLP: Sets the status byte to a value popped off the top of the stack
fn plp(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///ROL: Rotate the bits of the given value to the left
fn rol(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///ROR: Rotate the bits of the given value to the right
fn ror(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///RTI: Returns from an interrupt, reversing the operations performed by the BRK instruction
fn rti(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///RTS: Returns from a subroutine, taking the value of the program counter from the stack
fn rts(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///SBC: Subtracts a value and the opposite of the carry bit from the accumulator
fn sbc(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///SEC: Sets the carry bit to one
fn sec(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///SED: Sets the decimal bit to one
fn sed(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///SEI: Sets the interrupt disable bit to one
fn sei(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///STA: Store the accumulator in the given memory address
fn sta(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///STX: Store the x register in the given memory address
fn stx(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///STY: Store the y register in the given memory address
fn sty(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///TAX: Transfer the accumulator into the x register
fn tax(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///TAY: Transfer the accumulator into the y register
fn tay(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///TXS: Transfer the x register into the stack pointer
fn txs(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
}

///TYA: Transfer the y register into the accumulator
fn tya(cpu: &mut MOS6502, address_mode: AddressModeFunction) -> u8{
    unimplemented!()
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
fn branch(cpu: &mut MOS6502, branch_condition: bool, address_mode: AddressModeFunction) -> u8{
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

//TESTS--------------------------------------------------------------------------------------------

#[cfg(test)]
mod test{

    use super::MOS6502;
    use super::StatusFlag;
    use super::super::address_modes::AddressModeValue;
    use super::*;

    #[test]
    fn test_adc(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x09,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x10,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{accumulator:0x1a,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Carry, false);

        adc(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_adc_zero_carry_flags(){
        let mut cpu_initial = MOS6502{
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x01,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);


        adc(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_adc_overflow_negative_flags(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x7f,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x01,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x80,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Overflow, true);
        cpu_expected.set_flag(StatusFlag::Negative, true);


        adc(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(not(nes))]
    fn test_adc_decimal_mode(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x09,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x09,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{accumulator:0x19,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Carry, false);

        adc(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(not(nes))]
    fn test_adc_decimal_mode_zero_carry_flags(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x98,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x01,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{accumulator:0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        adc(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(not(nes))]
    fn test_adc_decimal_mode_overflow_negative_flags(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x75,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x06,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);

        let mut cpu_expected = MOS6502{accumulator:0x81,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        adc(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }
}