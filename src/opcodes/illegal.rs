///ILLEGAL OPCODES----------------------------------------------------------------------------------
/// This module contains all functions for the illegal opcodes that are not defined by official sources,
/// only as placeholders for now.
use super::super::{Interface6502, MOS6502};
//use super::super::StatusFlag; //Commented for later when these are implemented
use super::super::address_modes::*;

pub(super) fn slo(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn rla(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn sre(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn rra(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn sax(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn lax(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn dcp(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn isc(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn anc(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn alr(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn arr(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn xaa(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn axs(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn ahx(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn shy(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn shx(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn tas(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn las(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}

pub(super) fn kil(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) -> u8 {
    panic!("Illegal opcode function called")
}
