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
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        let value = cpu.read(address);
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
    } else {
        panic!("ADC opcode called with invalid address mode!")
    }
}

///AND: Performs a logical and with the accumulator and the addressed value, storing the result
///     in the accumulator
fn and(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        let value = cpu.read(address);
        cpu.accumulator &= value;
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
        return 0; //Operation never adds any extra cycles
    } else {
        panic!("AND opcode called with invalid address mode!")
    }
}

///ASL: Performs a left bit shift on the addressed value or accumulator
fn asl(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    //Wrapped local function to handle both cases
    fn asl_wrapped(cpu: &mut MOS6502, value: u8) -> u8{
        //Store the 7th bit in the carry bit
        cpu.set_flag(StatusFlag::Carry, value >> 7 == 1);
        let shifted_value = value << 1;
        cpu.set_flag(StatusFlag::Negative, shifted_value & StatusFlag::Negative as u8 > 0);
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        return shifted_value;
    }

    match address_mode_value{
        AddressModeValue::Accumulator => {
            cpu.accumulator = asl_wrapped(cpu, cpu.accumulator);
        },
        AddressModeValue::AbsoluteAddress(address) => {
            let value = asl_wrapped(cpu, cpu.read(address));
            cpu.write(address, value);
        },
        _ => panic!("ASL opcode called with invalid address mode!")
    }
    return 0; //Operation never adds any extra cycles
}

///BCC: Branch if the carry bit is clear
fn bcc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Carry), address_mode_value)
}

///BCC: Branch if the carry bit is set
fn bcs(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Carry), address_mode_value)
}

///BEQ: Branch if the zero bit is set (branch if equal)
fn beq(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Zero), address_mode_value)
}

///BIT: Uses the accumulator as a mask pattern to test the bits of a given memory location
fn bit(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        let value = cpu.accumulator & cpu.read(address);
        cpu.set_flag(StatusFlag::Zero, value == 0);
        cpu.set_flag(StatusFlag::Overflow, value & StatusFlag::Overflow as u8 > 0);
        cpu.set_flag(StatusFlag::Negative, value & StatusFlag::Negative as u8 > 0);
        return 0; //Operation never adds any extra cycles
    } else {
        panic!("BIT opcode called with invalid address mode!")
    }
}

///BMI: Branch if the negative bit is set (branch if negative)
fn bmi(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Negative), address_mode_value)
}

///BNE: Branch if the zero bit is clear (branch if not equal)
fn bne(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return branch(cpu, !cpu.get_flag(StatusFlag::Zero), address_mode_value)
}

///BPL: Branch if the negative bit is clear (branch if positive)
fn bpl(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return branch(cpu, !cpu.get_flag(StatusFlag::Negative), address_mode_value)
}

///BRK: Force an interrupt
fn brk(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///BVC: Branch if the overflow bit is clear
fn bvc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return branch(cpu, !cpu.get_flag(StatusFlag::Overflow), address_mode_value)
}

///BVS: Branch if the overflow bit is set
fn bvs(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return branch(cpu, cpu.get_flag(StatusFlag::Overflow), address_mode_value)
}

///CLC: Clear carry bit
fn clc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.set_flag(StatusFlag::Carry, false);
    return 0;
}

///CLD: Clear decimal mode bit
fn cld(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.set_flag(StatusFlag::Decimal, false);
    return 0;
}

///CLD: Clear interrupt disable bit
fn cli(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.set_flag(StatusFlag::InterruptDisable, false);
    return 0;
}

///CLD: Clear overflow bit
fn clv(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.set_flag(StatusFlag::Overflow, false);
    return 0;
}

///CMP: Compare accumulator to a value in memory
fn cmp(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    compare(cpu, cpu.accumulator, address_mode_value);
    return 0;
}

///CPX: Compare x register to a value in memory
fn cpx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    compare(cpu, cpu.x_register, address_mode_value);
    return 0;
}

///CPY: Compare y register to a value in memory
fn cpy(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    compare(cpu, cpu.y_register, address_mode_value);
    return 0;
}

///DEC: Subtract one from the value at the given memory location
fn dec(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) =  address_mode_value {
        let value = cpu.read(address).wrapping_sub(1);
        cpu.set_flag(StatusFlag::Zero, value == 0);
        cpu.set_flag(StatusFlag::Negative, value & StatusFlag::Negative as u8 > 0);
        cpu.write(address, value);
        return 0;
    } else {
        panic!("DEC opcode called with invalid address mode!")
    }
}

///DEC: Subtract one from the x register
fn dex(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::Implied =  address_mode_value {
        cpu.x_register = cpu.x_register.wrapping_sub(1);
        cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
        return 0;
    } else {
        panic!("DEX opcode called with invalid address mode!")
    }
}

///DEY: Subtract one from the y register
fn dey(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::Implied =  address_mode_value {
        cpu.y_register = cpu.y_register.wrapping_sub(1);
        cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
        return 0;
    } else {
        panic!("DEY opcode called with invalid address mode!")
    }
}

///EOR: Set accumulator to the result of an exclusive or operation with the accumulator and a value from memory
fn eor(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        let value = cpu.read(address);
        cpu.accumulator ^= value;
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
        return 0; //Operation never adds any extra cycles
    } else {
        panic!("EOR opcode called with invalid address mode!")
    }
}

///INC: Add one to the value at the given memory location
fn inc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) =  address_mode_value {
        let value = cpu.read(address).wrapping_add(1);
        cpu.set_flag(StatusFlag::Zero, value == 0);
        cpu.set_flag(StatusFlag::Negative, value & StatusFlag::Negative as u8 > 0);
        cpu.write(address, value);
        return 0;
    } else {
        panic!("INC opcode called with invalid address mode!")
    }
}

///INX: Add one to the x register
fn inx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::Implied =  address_mode_value {
        cpu.x_register = cpu.x_register.wrapping_add(1);
        cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
        return 0;
    } else {
        panic!("INX opcode called with invalid address mode!")
    }
}

///INX: Add one to the y register
fn iny(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::Implied =  address_mode_value {
        cpu.y_register = cpu.y_register.wrapping_add(1);
        cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
        return 0;
    } else {
        panic!("INY opcode called with invalid address mode!")
    }
}

///JMP: Set the program counter to the given address
fn jmp(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///JSR: Puts the current program counter value on the stack and then jumps to the given address
fn jsr(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///LDA: Load a value into the accumulator from a memory address
fn lda(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///LDX: Load a value into the x register from a memory address
fn ldx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///LDY: Load a value into the y register from a memory address
fn ldy(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///LSR: Performs a right bit shift on the given value
fn lsr(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///NOP: No operation
fn nop(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///ORA: The accumulator is set to the result of a inclusive or operation applied to the accumulator and a memory value
fn ora(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///PHA: Push the value of the accumulator onto the stack
fn pha(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///PHP: Push the value of the status byte onto the stack
fn php(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///PLA: Sets the accumulator to a value popped off the top of the stack
fn pla(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///PLP: Sets the status byte to a value popped off the top of the stack
fn plp(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///ROL: Rotate the bits of the given value to the left
fn rol(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///ROR: Rotate the bits of the given value to the right
fn ror(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///RTI: Returns from an interrupt, reversing the operations performed by the BRK instruction
fn rti(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///RTS: Returns from a subroutine, taking the value of the program counter from the stack
fn rts(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///SBC: Subtracts a value and the opposite of the carry bit from the accumulator
fn sbc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///SEC: Sets the carry bit to one
fn sec(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///SED: Sets the decimal bit to one
fn sed(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///SEI: Sets the interrupt disable bit to one
fn sei(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///STA: Store the accumulator in the given memory address
fn sta(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///STX: Store the x register in the given memory address
fn stx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///STY: Store the y register in the given memory address
fn sty(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///TAX: Transfer the accumulator into the x register
fn tax(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///TAY: Transfer the accumulator into the y register
fn tay(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///TXS: Transfer the x register into the stack pointer
fn txs(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    unimplemented!()
}

///TYA: Transfer the y register into the accumulator
fn tya(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
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
fn branch(cpu: &mut MOS6502, branch_condition: bool, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::RelativeAddress(relative_address) = address_mode_value{
        let address = signed_8_bit_to_16(relative_address).wrapping_add(cpu.program_counter);
        let extra_cycles;

        if branch_condition{
            if address & 0xff00 != cpu.program_counter & 0xff00{
                extra_cycles = 2;
            } else {
                extra_cycles = 1;
            }

            cpu.program_counter = address;
        } else {
            extra_cycles = 0;
        }

        return extra_cycles;
    } else {
        panic!("Branching opcode called with invalid address mode!")
    }
}

///General purpose function for compare opcodes
fn compare(cpu: &mut MOS6502, register: u8, address_mode_value: AddressModeValue){
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        let value = cpu.read(address);
        cpu.set_flag(StatusFlag::Carry, register >= value);
        cpu.set_flag(StatusFlag::Zero, register == value);
        cpu.set_flag(StatusFlag::Negative, (register.wrapping_sub(value)) & StatusFlag::Negative as u8 > 0);
        println!("{}", register.wrapping_sub(value))
    } else {
        panic!("Compare opcode called with invalid address mode!")
    }

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

    #[test]
    fn test_and(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x95,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x80,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x80,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);


        and(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_and_zero_flag(){
        let mut cpu_initial = MOS6502{
            accumulator: 0xf0,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x0f,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);


        and(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_asl(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xff,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0xff << 1);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Carry, true);
        cpu_expected.set_flag(StatusFlag::Negative, true);


        asl(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_asl_accumulator_zero_flag(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x80,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Carry, true);
        cpu_expected.set_flag(StatusFlag::Zero, true);


        asl(&mut cpu_initial, AddressModeValue::Accumulator);

        assert_eq!(cpu_initial, cpu_expected);
    }

    //Tests of the branch function are used in place of testing the individual branch conditions
    #[test]
    fn test_branch(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x00fb,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{program_counter:0x0100,..cpu_initial.clone()};

        assert_eq!(branch(&mut cpu_initial, true ,AddressModeValue::RelativeAddress(0x05)), 2);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_branch_backwards(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x000a,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{program_counter:0x0005,..cpu_initial.clone()};

        assert_eq!(branch(&mut cpu_initial, true ,AddressModeValue::RelativeAddress(0xfb)), 1);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_branch_fail(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x000a,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};

        assert_eq!(branch(&mut cpu_initial, false ,AddressModeValue::RelativeAddress(0xfb)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_bit(){
        let mut cpu_initial = MOS6502{
            accumulator: 0xf0,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x000a,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x0f,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        assert_eq!(bit(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_bit_negative_overflow_flags(){
        let mut cpu_initial = MOS6502{
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x000a,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xc0,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        assert_eq!(bit(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_compare(){
        let mut cpu_initial = MOS6502{
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xff,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        compare(&mut cpu_initial, 0xff,AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_compare_less(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x0f,
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

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        compare(&mut cpu_initial, 0x0f,AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dec(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
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
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x00);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        assert_eq!(dec(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dec_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x00,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0xff);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        assert_eq!(dec(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dex(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x01,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{x_register: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        assert_eq!(dex(&mut cpu_initial, AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dex_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{x_register: 0xff,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        assert_eq!(dex(&mut cpu_initial,AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dey(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x01,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{y_register: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        assert_eq!(dey(&mut cpu_initial, AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dey_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{y_register: 0xff,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        assert_eq!(dey(&mut cpu_initial,AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_eor(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x80,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x90,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);


        eor(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_eor_zero_flag(){
        let mut cpu_initial = MOS6502{
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xff,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);


        assert_eq!(eor(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_inc(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xff,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x00);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        assert_eq!(inc(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_inc_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x7f,
                    _ => panic!("Unintended Address Accessed")
                }
            },
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x80);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        assert_eq!(inc(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_inx(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0xff,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{x_register: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        assert_eq!(inx(&mut cpu_initial, AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_inx_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x7f,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{x_register: 0x80,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        assert_eq!(inx(&mut cpu_initial,AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_iny(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0xff,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{y_register: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        assert_eq!(iny(&mut cpu_initial, AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_iny_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x7f,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{y_register: 0x80,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        assert_eq!(iny(&mut cpu_initial,AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }
}