///ILLEGAL OPCODES----------------------------------------------------------------------------------
/// This module contains all functions for the illegal opcodes that are not defined by official sources,
/// only as placeholders for now.

use super::super::MOS6502;
use super::super::StatusFlag;
use super::super::AddressModeFunction;
use super::super::OpcodeFunction;
use super::super::address_modes::*;

pub (super) fn slo(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn rla(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn sre(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn rra(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn sax(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn lax(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn dcp(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn isc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn anc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn alr(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn arr(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn xaa(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn axs(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn ahx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn shy(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn shx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn tas(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn las(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}

pub (super) fn kil(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    panic!("Illegal opcode function called")
}