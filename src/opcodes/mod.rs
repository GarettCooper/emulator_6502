//!  ### OPCODES
//!  This module contains all of the opcode functions to prevent the parent module from being primarily full of them
//!  An opcode function represents one of the 6502's opcodes. An opcode function is passed the
//!  address mode to use and returns the number of extra cycles that address mode has taken
mod illegal;

use super::address_modes::*;
use super::{AddressModeFunction, Interface6502, OpcodeFunction, StatusFlag, MOS6502};
use illegal::*;

#[derive(Clone, Copy)]
/// Type used to represent an Opcode, a single instruction on the 6502
pub(super) struct Instruction<'a> {
    name: &'a str, //For logging
    function: OpcodeFunction,
    address_mode: AddressModeFunction,
    cycles: u8,
}

impl Instruction<'_> {
    /// Gets the name of the opcode for debug purposes
    pub(super) fn get_name(&self) -> &str {
        return self.name;
    }

    /// Executes the instruction by calling the wrapped function
    pub(super) fn execute_instruction(&self, cpu: &mut MOS6502, interface: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
        (self.function)(cpu, interface, address_mode_value);
    }

    /// Calls the address mode function to retrieve the addressed location to be used in the instruction
    pub(super) fn find_address(&self, cpu: &mut MOS6502, interface: &mut dyn Interface6502) -> AddressModeValue {
        return (self.address_mode)(cpu, interface);
    }

    /// Gets the number of cycles that executing this instruction will take
    pub(super) fn get_cycles(&self) -> u8 {
        return self.cycles;
    }
}

//TODO: Replace this with a macro-generated match statement so that it can be better evaluated at compile time
/// The table that is used to map instructions to the appropriate function and addressing mode for
/// executing them.
pub (super) static OPCODE_TABLE: [Instruction; 256] = [
    Instruction { name:"brk", function: brk, address_mode: implied, cycles: 7 },		//0x0
    Instruction { name:"ora", function: ora, address_mode: indirect_x, cycles: 6 },		//0x1
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x2
    Instruction { name:"slo", function: slo, address_mode: indirect_x, cycles: 8 },		//0x3
    Instruction { name:"nop", function: nop, address_mode: zero_page, cycles: 3 },		//0x4
    Instruction { name:"ora", function: ora, address_mode: zero_page, cycles: 3 },		//0x5
    Instruction { name:"asl", function: asl, address_mode: zero_page, cycles: 5 },		//0x6
    Instruction { name:"slo", function: slo, address_mode: zero_page, cycles: 5 },		//0x7
    Instruction { name:"php", function: php, address_mode: implied, cycles: 3 },		//0x8
    Instruction { name:"ora", function: ora, address_mode: immediate, cycles: 2 },		//0x9
    Instruction { name:"asl", function: asl, address_mode: implied, cycles: 2 },		//0xa
    Instruction { name:"anc", function: anc, address_mode: immediate, cycles: 2 },		//0xb
    Instruction { name:"nop", function: nop, address_mode: absolute, cycles: 4 },		//0xc
    Instruction { name:"ora", function: ora, address_mode: absolute, cycles: 4 },		//0xd
    Instruction { name:"asl", function: asl, address_mode: absolute, cycles: 6 },		//0xe
    Instruction { name:"slo", function: slo, address_mode: absolute, cycles: 6 },		//0xf
    Instruction { name:"bpl", function: bpl, address_mode: relative, cycles: 2 },		//0x10
    Instruction { name:"ora", function: ora, address_mode: indirect_y, cycles: 5 },		//0x11
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x12
    Instruction { name:"slo", function: slo, address_mode: indirect_y_const, cycles: 8 },		//0x13
    Instruction { name:"nop", function: nop, address_mode: zero_page_x, cycles: 4 },		//0x14
    Instruction { name:"ora", function: ora, address_mode: zero_page_x, cycles: 4 },		//0x15
    Instruction { name:"asl", function: asl, address_mode: zero_page_x, cycles: 6 },		//0x16
    Instruction { name:"slo", function: slo, address_mode: zero_page_x, cycles: 6 },		//0x17
    Instruction { name:"clc", function: clc, address_mode: implied, cycles: 2 },		//0x18
    Instruction { name:"ora", function: ora, address_mode: absolute_y, cycles: 4 },		//0x19
    Instruction { name:"nop", function: nop, address_mode: implied, cycles: 2 },		//0x1a
    Instruction { name:"slo", function: slo, address_mode: absolute_y_const, cycles: 7 },		//0x1b
    Instruction { name:"nop", function: nop, address_mode: absolute_x, cycles: 4 },		//0x1c
    Instruction { name:"ora", function: ora, address_mode: absolute_x, cycles: 4 },		//0x1d
    Instruction { name:"asl", function: asl, address_mode: absolute_x_const, cycles: 7 },		//0x1e
    Instruction { name:"slo", function: slo, address_mode: absolute_x_const, cycles: 7 },		//0x1f
    Instruction { name:"jsr", function: jsr, address_mode: absolute, cycles: 6 },		//0x20
    Instruction { name:"and", function: and, address_mode: indirect_x, cycles: 6 },		//0x21
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x22
    Instruction { name:"rla", function: rla, address_mode: indirect_x, cycles: 8 },		//0x23
    Instruction { name:"bit", function: bit, address_mode: zero_page, cycles: 3 },		//0x24
    Instruction { name:"and", function: and, address_mode: zero_page, cycles: 3 },		//0x25
    Instruction { name:"rol", function: rol, address_mode: zero_page, cycles: 5 },		//0x26
    Instruction { name:"rla", function: rla, address_mode: zero_page, cycles: 5 },		//0x27
    Instruction { name:"plp", function: plp, address_mode: implied, cycles: 4 },		//0x28
    Instruction { name:"and", function: and, address_mode: immediate, cycles: 2 },		//0x29
    Instruction { name:"rol", function: rol, address_mode: implied, cycles: 2 },		//0x2a
    Instruction { name:"anc", function: anc, address_mode: immediate, cycles: 2 },		//0x2b
    Instruction { name:"bit", function: bit, address_mode: absolute, cycles: 4 },		//0x2c
    Instruction { name:"and", function: and, address_mode: absolute, cycles: 4 },		//0x2d
    Instruction { name:"rol", function: rol, address_mode: absolute, cycles: 6 },		//0x2e
    Instruction { name:"rla", function: rla, address_mode: absolute, cycles: 6 },		//0x2f
    Instruction { name:"bmi", function: bmi, address_mode: relative, cycles: 2 },		//0x30
    Instruction { name:"and", function: and, address_mode: indirect_y, cycles: 5 },		//0x31
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x32
    Instruction { name:"rla", function: rla, address_mode: indirect_y_const, cycles: 8 },		//0x33
    Instruction { name:"nop", function: nop, address_mode: zero_page_x, cycles: 4 },		//0x34
    Instruction { name:"and", function: and, address_mode: zero_page_x, cycles: 4 },		//0x35
    Instruction { name:"rol", function: rol, address_mode: zero_page_x, cycles: 6 },		//0x36
    Instruction { name:"rla", function: rla, address_mode: zero_page_x, cycles: 6 },		//0x37
    Instruction { name:"sec", function: sec, address_mode: implied, cycles: 2 },		//0x38
    Instruction { name:"and", function: and, address_mode: absolute_y, cycles: 4 },		//0x39
    Instruction { name:"nop", function: nop, address_mode: implied, cycles: 2 },		//0x3a
    Instruction { name:"rla", function: rla, address_mode: absolute_y_const, cycles: 7 },		//0x3b
    Instruction { name:"nop", function: nop, address_mode: absolute_x, cycles: 4 },		//0x3c
    Instruction { name:"and", function: and, address_mode: absolute_x, cycles: 4 },		//0x3d
    Instruction { name:"rol", function: rol, address_mode: absolute_x_const, cycles: 7 },		//0x3e
    Instruction { name:"rla", function: rla, address_mode: absolute_x_const, cycles: 7 },		//0x3f
    Instruction { name:"rti", function: rti, address_mode: implied, cycles: 6 },		//0x40
    Instruction { name:"eor", function: eor, address_mode: indirect_x, cycles: 6 },		//0x41
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x42
    Instruction { name:"sre", function: sre, address_mode: indirect_x, cycles: 8 },		//0x43
    Instruction { name:"nop", function: nop, address_mode: zero_page, cycles: 3 },		//0x44
    Instruction { name:"eor", function: eor, address_mode: zero_page, cycles: 3 },		//0x45
    Instruction { name:"lsr", function: lsr, address_mode: zero_page, cycles: 5 },		//0x46
    Instruction { name:"sre", function: sre, address_mode: zero_page, cycles: 5 },		//0x47
    Instruction { name:"pha", function: pha, address_mode: implied, cycles: 3 },		//0x48
    Instruction { name:"eor", function: eor, address_mode: immediate, cycles: 2 },		//0x49
    Instruction { name:"lsr", function: lsr, address_mode: implied, cycles: 2 },		//0x4a
    Instruction { name:"alr", function: alr, address_mode: immediate, cycles: 2 },		//0x4b
    Instruction { name:"jmp", function: jmp, address_mode: absolute, cycles: 3 },		//0x4c
    Instruction { name:"eor", function: eor, address_mode: absolute, cycles: 4 },		//0x4d
    Instruction { name:"lsr", function: lsr, address_mode: absolute, cycles: 6 },		//0x4e
    Instruction { name:"sre", function: sre, address_mode: absolute, cycles: 6 },		//0x4f
    Instruction { name:"bvc", function: bvc, address_mode: relative, cycles: 2 },		//0x50
    Instruction { name:"eor", function: eor, address_mode: indirect_y, cycles: 5 },		//0x51
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x52
    Instruction { name:"sre", function: sre, address_mode: indirect_y_const, cycles: 8 },		//0x53
    Instruction { name:"nop", function: nop, address_mode: zero_page_x, cycles: 4 },		//0x54
    Instruction { name:"eor", function: eor, address_mode: zero_page_x, cycles: 4 },		//0x55
    Instruction { name:"lsr", function: lsr, address_mode: zero_page_x, cycles: 6 },		//0x56
    Instruction { name:"sre", function: sre, address_mode: zero_page_x, cycles: 6 },		//0x57
    Instruction { name:"cli", function: cli, address_mode: implied, cycles: 2 },		//0x58
    Instruction { name:"eor", function: eor, address_mode: absolute_y, cycles: 4 },		//0x59
    Instruction { name:"nop", function: nop, address_mode: implied, cycles: 2 },		//0x5a
    Instruction { name:"sre", function: sre, address_mode: absolute_y_const, cycles: 7 },		//0x5b
    Instruction { name:"nop", function: nop, address_mode: absolute_x, cycles: 4 },		//0x5c
    Instruction { name:"eor", function: eor, address_mode: absolute_x, cycles: 4 },		//0x5d
    Instruction { name:"lsr", function: lsr, address_mode: absolute_x_const, cycles: 7 },		//0x5e
    Instruction { name:"sre", function: sre, address_mode: absolute_x_const, cycles: 7 },		//0x5f
    Instruction { name:"rts", function: rts, address_mode: implied, cycles: 6 },		//0x60
    Instruction { name:"adc", function: adc, address_mode: indirect_x, cycles: 6 },		//0x61
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x62
    Instruction { name:"rra", function: rra, address_mode: indirect_x, cycles: 8 },		//0x63
    Instruction { name:"nop", function: nop, address_mode: zero_page, cycles: 3 },		//0x64
    Instruction { name:"adc", function: adc, address_mode: zero_page, cycles: 3 },		//0x65
    Instruction { name:"ror", function: ror, address_mode: zero_page, cycles: 5 },		//0x66
    Instruction { name:"rra", function: rra, address_mode: zero_page, cycles: 5 },		//0x67
    Instruction { name:"pla", function: pla, address_mode: implied, cycles: 4 },		//0x68
    Instruction { name:"adc", function: adc, address_mode: immediate, cycles: 2 },		//0x69
    Instruction { name:"ror", function: ror, address_mode: implied, cycles: 2 },		//0x6a
    Instruction { name:"arr", function: arr, address_mode: immediate, cycles: 2 },		//0x6b
    Instruction { name:"jmp", function: jmp, address_mode: indirect, cycles: 5 },		//0x6c
    Instruction { name:"adc", function: adc, address_mode: absolute, cycles: 4 },		//0x6d
    Instruction { name:"ror", function: ror, address_mode: absolute, cycles: 6 },		//0x6e
    Instruction { name:"rra", function: rra, address_mode: absolute, cycles: 6 },		//0x6f
    Instruction { name:"bvs", function: bvs, address_mode: relative, cycles: 2 },		//0x70
    Instruction { name:"adc", function: adc, address_mode: indirect_y, cycles: 5 },		//0x71
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x72
    Instruction { name:"rra", function: rra, address_mode: indirect_y_const, cycles: 8 },		//0x73
    Instruction { name:"nop", function: nop, address_mode: zero_page_x, cycles: 4 },		//0x74
    Instruction { name:"adc", function: adc, address_mode: zero_page_x, cycles: 4 },		//0x75
    Instruction { name:"ror", function: ror, address_mode: zero_page_x, cycles: 6 },		//0x76
    Instruction { name:"rra", function: rra, address_mode: zero_page_x, cycles: 6 },		//0x77
    Instruction { name:"sei", function: sei, address_mode: implied, cycles: 2 },		//0x78
    Instruction { name:"adc", function: adc, address_mode: absolute_y, cycles: 4 },		//0x79
    Instruction { name:"nop", function: nop, address_mode: implied, cycles: 2 },		//0x7a
    Instruction { name:"rra", function: rra, address_mode: absolute_y_const, cycles: 7 },		//0x7b
    Instruction { name:"nop", function: nop, address_mode: absolute_x, cycles: 4 },		//0x7c
    Instruction { name:"adc", function: adc, address_mode: absolute_x, cycles: 4 },		//0x7d
    Instruction { name:"ror", function: ror, address_mode: absolute_x_const, cycles: 7 },		//0x7e
    Instruction { name:"rra", function: rra, address_mode: absolute_x_const, cycles: 7 },		//0x7f
    Instruction { name:"nop", function: nop, address_mode: immediate, cycles: 2 },		//0x80
    Instruction { name:"sta", function: sta, address_mode: indirect_x, cycles: 6 },		//0x81
    Instruction { name:"nop", function: nop, address_mode: immediate, cycles: 2 },		//0x82
    Instruction { name:"sax", function: sax, address_mode: indirect_x, cycles: 6 },		//0x83
    Instruction { name:"sty", function: sty, address_mode: zero_page, cycles: 3 },		//0x84
    Instruction { name:"sta", function: sta, address_mode: zero_page, cycles: 3 },		//0x85
    Instruction { name:"stx", function: stx, address_mode: zero_page, cycles: 3 },		//0x86
    Instruction { name:"sax", function: sax, address_mode: zero_page, cycles: 3 },		//0x87
    Instruction { name:"dey", function: dey, address_mode: implied, cycles: 2 },		//0x88
    Instruction { name:"nop", function: nop, address_mode: immediate, cycles: 2 },		//0x89
    Instruction { name:"txa", function: txa, address_mode: implied, cycles: 2 },		//0x8a
    Instruction { name:"xaa", function: xaa, address_mode: immediate, cycles: 2 },		//0x8b
    Instruction { name:"sty", function: sty, address_mode: absolute, cycles: 4 },		//0x8c
    Instruction { name:"sta", function: sta, address_mode: absolute, cycles: 4 },		//0x8d
    Instruction { name:"stx", function: stx, address_mode: absolute, cycles: 4 },		//0x8e
    Instruction { name:"sax", function: sax, address_mode: absolute, cycles: 4 },		//0x8f
    Instruction { name:"bcc", function: bcc, address_mode: relative, cycles: 2 },		//0x90
    Instruction { name:"sta", function: sta, address_mode: indirect_y, cycles: 6 },		//0x91
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0x92
    Instruction { name:"ahx", function: ahx, address_mode: indirect_y, cycles: 6 },		//0x93
    Instruction { name:"sty", function: sty, address_mode: zero_page_x, cycles: 4 },		//0x94
    Instruction { name:"sta", function: sta, address_mode: zero_page_x, cycles: 4 },		//0x95
    Instruction { name:"stx", function: stx, address_mode: zero_page_y, cycles: 4 },		//0x96
    Instruction { name:"sax", function: sax, address_mode: zero_page_y, cycles: 4 },		//0x97
    Instruction { name:"tya", function: tya, address_mode: implied, cycles: 2 },		//0x98
    Instruction { name:"sta", function: sta, address_mode: absolute_y_const, cycles: 5 },		//0x99
    Instruction { name:"txs", function: txs, address_mode: implied, cycles: 2 },		//0x9a
    Instruction { name:"tas", function: tas, address_mode: absolute_y, cycles: 5 },		//0x9b
    Instruction { name:"shy", function: shy, address_mode: absolute_x, cycles: 5 },		//0x9c
    Instruction { name:"sta", function: sta, address_mode: absolute_x_const, cycles: 5 },		//0x9d
    Instruction { name:"shx", function: shx, address_mode: absolute_y, cycles: 5 },		//0x9e
    Instruction { name:"ahx", function: ahx, address_mode: absolute_y, cycles: 5 },		//0x9f
    Instruction { name:"ldy", function: ldy, address_mode: immediate, cycles: 2 },		//0xa0
    Instruction { name:"lda", function: lda, address_mode: indirect_x, cycles: 6 },		//0xa1
    Instruction { name:"ldx", function: ldx, address_mode: immediate, cycles: 2 },		//0xa2
    Instruction { name:"lax", function: lax, address_mode: indirect_x, cycles: 6 },		//0xa3
    Instruction { name:"ldy", function: ldy, address_mode: zero_page, cycles: 3 },		//0xa4
    Instruction { name:"lda", function: lda, address_mode: zero_page, cycles: 3 },		//0xa5
    Instruction { name:"ldx", function: ldx, address_mode: zero_page, cycles: 3 },		//0xa6
    Instruction { name:"lax", function: lax, address_mode: zero_page, cycles: 3 },		//0xa7
    Instruction { name:"tay", function: tay, address_mode: implied, cycles: 2 },		//0xa8
    Instruction { name:"lda", function: lda, address_mode: immediate, cycles: 2 },		//0xa9
    Instruction { name:"tax", function: tax, address_mode: implied, cycles: 2 },		//0xaa
    Instruction { name:"lax", function: lax, address_mode: immediate, cycles: 2 },		//0xab
    Instruction { name:"ldy", function: ldy, address_mode: absolute, cycles: 4 },		//0xac
    Instruction { name:"lda", function: lda, address_mode: absolute, cycles: 4 },		//0xad
    Instruction { name:"ldx", function: ldx, address_mode: absolute, cycles: 4 },		//0xae
    Instruction { name:"lax", function: lax, address_mode: absolute, cycles: 4 },		//0xaf
    Instruction { name:"bcs", function: bcs, address_mode: relative, cycles: 2 },		//0xb0
    Instruction { name:"lda", function: lda, address_mode: indirect_y, cycles: 5 },		//0xb1
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0xb2
    Instruction { name:"lax", function: lax, address_mode: indirect_y, cycles: 5 },		//0xb3
    Instruction { name:"ldy", function: ldy, address_mode: zero_page_x, cycles: 4 },		//0xb4
    Instruction { name:"lda", function: lda, address_mode: zero_page_x, cycles: 4 },		//0xb5
    Instruction { name:"ldx", function: ldx, address_mode: zero_page_y, cycles: 4 },		//0xb6
    Instruction { name:"lax", function: lax, address_mode: zero_page_y, cycles: 4 },		//0xb7
    Instruction { name:"clv", function: clv, address_mode: implied, cycles: 2 },		//0xb8
    Instruction { name:"lda", function: lda, address_mode: absolute_y, cycles: 4 },		//0xb9
    Instruction { name:"tsx", function: tsx, address_mode: implied, cycles: 2 },		//0xba
    Instruction { name:"las", function: las, address_mode: absolute_y, cycles: 4 },		//0xbb
    Instruction { name:"ldy", function: ldy, address_mode: absolute_x, cycles: 4 },		//0xbc
    Instruction { name:"lda", function: lda, address_mode: absolute_x, cycles: 4 },		//0xbd
    Instruction { name:"ldx", function: ldx, address_mode: absolute_y, cycles: 4 },		//0xbe
    Instruction { name:"lax", function: lax, address_mode: absolute_y, cycles: 4 },		//0xbf
    Instruction { name:"cpy", function: cpy, address_mode: immediate, cycles: 2 },		//0xc0
    Instruction { name:"cmp", function: cmp, address_mode: indirect_x, cycles: 6 },		//0xc1
    Instruction { name:"nop", function: nop, address_mode: immediate, cycles: 2 },		//0xc2
    Instruction { name:"dcp", function: dcp, address_mode: indirect_x, cycles: 8 },		//0xc3
    Instruction { name:"cpy", function: cpy, address_mode: zero_page, cycles: 3 },		//0xc4
    Instruction { name:"cmp", function: cmp, address_mode: zero_page, cycles: 3 },		//0xc5
    Instruction { name:"dec", function: dec, address_mode: zero_page, cycles: 5 },		//0xc6
    Instruction { name:"dcp", function: dcp, address_mode: zero_page, cycles: 5 },		//0xc7
    Instruction { name:"iny", function: iny, address_mode: implied, cycles: 2 },		//0xc8
    Instruction { name:"cmp", function: cmp, address_mode: immediate, cycles: 2 },		//0xc9
    Instruction { name:"dex", function: dex, address_mode: implied, cycles: 2 },		//0xca
    Instruction { name:"axs", function: axs, address_mode: immediate, cycles: 2 },		//0xcb
    Instruction { name:"cpy", function: cpy, address_mode: absolute, cycles: 4 },		//0xcc
    Instruction { name:"cmp", function: cmp, address_mode: absolute, cycles: 4 },		//0xcd
    Instruction { name:"dec", function: dec, address_mode: absolute, cycles: 6 },		//0xce
    Instruction { name:"dcp", function: dcp, address_mode: absolute, cycles: 6 },		//0xcf
    Instruction { name:"bne", function: bne, address_mode: relative, cycles: 2 },		//0xd0
    Instruction { name:"cmp", function: cmp, address_mode: indirect_y, cycles: 5 },		//0xd1
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0xd2
    Instruction { name:"dcp", function: dcp, address_mode: indirect_y_const, cycles: 8 },		//0xd3
    Instruction { name:"nop", function: nop, address_mode: zero_page_x, cycles: 4 },		//0xd4
    Instruction { name:"cmp", function: cmp, address_mode: zero_page_x, cycles: 4 },		//0xd5
    Instruction { name:"dec", function: dec, address_mode: zero_page_x, cycles: 6 },		//0xd6
    Instruction { name:"dcp", function: dcp, address_mode: zero_page_x, cycles: 6 },		//0xd7
    Instruction { name:"cld", function: cld, address_mode: implied, cycles: 2 },		//0xd8
    Instruction { name:"cmp", function: cmp, address_mode: absolute_y, cycles: 4 },		//0xd9
    Instruction { name:"nop", function: nop, address_mode: implied, cycles: 2 },		//0xda
    Instruction { name:"dcp", function: dcp, address_mode: absolute_y_const, cycles: 7 },		//0xdb
    Instruction { name:"nop", function: nop, address_mode: absolute_x, cycles: 4 },		//0xdc
    Instruction { name:"cmp", function: cmp, address_mode: absolute_x, cycles: 4 },		//0xdd
    Instruction { name:"dec", function: dec, address_mode: absolute_x_const, cycles: 7 },		//0xde
    Instruction { name:"dcp", function: dcp, address_mode: absolute_x_const, cycles: 7 },		//0xdf
    Instruction { name:"cpx", function: cpx, address_mode: immediate, cycles: 2 },		//0xe0
    Instruction { name:"sbc", function: sbc, address_mode: indirect_x, cycles: 6 },		//0xe1
    Instruction { name:"nop", function: nop, address_mode: immediate, cycles: 2 },		//0xe2
    Instruction { name:"isc", function: isc, address_mode: indirect_x, cycles: 8 },		//0xe3
    Instruction { name:"cpx", function: cpx, address_mode: zero_page, cycles: 3 },		//0xe4
    Instruction { name:"sbc", function: sbc, address_mode: zero_page, cycles: 3 },		//0xe5
    Instruction { name:"inc", function: inc, address_mode: zero_page, cycles: 5 },		//0xe6
    Instruction { name:"isc", function: isc, address_mode: zero_page, cycles: 5 },		//0xe7
    Instruction { name:"inx", function: inx, address_mode: implied, cycles: 2 },		//0xe8
    Instruction { name:"sbc", function: sbc, address_mode: immediate, cycles: 2 },		//0xe9
    Instruction { name:"nop", function: nop, address_mode: implied, cycles: 2 },		//0xea
    Instruction { name:"sbc", function: sbc, address_mode: immediate, cycles: 2 },		//0xeb
    Instruction { name:"cpx", function: cpx, address_mode: absolute, cycles: 4 },		//0xec
    Instruction { name:"sbc", function: sbc, address_mode: absolute, cycles: 4 },		//0xed
    Instruction { name:"inc", function: inc, address_mode: absolute, cycles: 6 },		//0xee
    Instruction { name:"isc", function: isc, address_mode: absolute, cycles: 6 },		//0xef
    Instruction { name:"beq", function: beq, address_mode: relative, cycles: 2 },		//0xf0
    Instruction { name:"sbc", function: sbc, address_mode: indirect_y, cycles: 5 },		//0xf1
    Instruction { name:"kil", function: kil, address_mode: implied, cycles: 0 },		//0xf2
    Instruction { name:"isc", function: isc, address_mode: indirect_y_const, cycles: 8 },		//0xf3
    Instruction { name:"nop", function: nop, address_mode: zero_page_x, cycles: 4 },		//0xf4
    Instruction { name:"sbc", function: sbc, address_mode: zero_page_x, cycles: 4 },		//0xf5
    Instruction { name:"inc", function: inc, address_mode: zero_page_x, cycles: 6 },		//0xf6
    Instruction { name:"isc", function: isc, address_mode: zero_page_x, cycles: 6 },		//0xf7
    Instruction { name:"sed", function: sed, address_mode: implied, cycles: 2 },		//0xf8
    Instruction { name:"sbc", function: sbc, address_mode: absolute_y, cycles: 4 },		//0xf9
    Instruction { name:"nop", function: nop, address_mode: implied, cycles: 2 },		//0xfa
    Instruction { name:"isc", function: isc, address_mode: absolute_y_const, cycles: 7 },		//0xfb
    Instruction { name:"nop", function: nop, address_mode: absolute_x, cycles: 4 },		//0xfc
    Instruction { name:"sbc", function: sbc, address_mode: absolute_x, cycles: 4 },		//0xfd
    Instruction { name:"inc", function: inc, address_mode: absolute_x_const, cycles: 7 },		//0xfe
    Instruction { name:"isc", function: isc, address_mode: absolute_x_const, cycles: 7 },		//0xff
];

/// ADC: Adds a value and the carry bit to the accumulator
fn adc(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = bus.read(address);
        let result: u16;

        //Only run if the CPU is not built in NES mode
        //TODO: Make sure cpu is removed as dead code in nes builds
        if cfg!(feature = "binary_coded_decimal") && cpu.get_flag(StatusFlag::Decimal) {
            let mut sum = cpu.accumulator.wrapping_add(value).wrapping_add(cpu.get_flag(StatusFlag::Carry) as u8);
            if (cpu.accumulator & 0x0f) + (value & 0x0f) + cpu.get_flag(StatusFlag::Carry) as u8 > 0x09 {
                sum = sum.wrapping_add(0x06);
            }
            if (sum & 0xf0) > 0x90 {
                sum = sum.wrapping_add(0x60);
                cpu.set_flag(StatusFlag::Carry, true);
            } else {
                cpu.set_flag(StatusFlag::Carry, false);
            }
            result = u16::from(sum);
        } else {
            result = u16::from(cpu.accumulator) + u16::from(value) + cpu.get_flag(StatusFlag::Carry) as u16;
            //Set the Carry flag for chain adding multi byte numbers
            cpu.set_flag(StatusFlag::Carry, result > u16::from(u8::max_value()));
        }
        cpu.set_flag(StatusFlag::Zero, result as u8 == 0);
        //Set the Overflow flag if a signed overflow has occurred
        cpu.set_flag(StatusFlag::Overflow, (!(cpu.accumulator ^ value) & (cpu.accumulator ^ result as u8) & StatusFlag::Negative as u8) > 0);
        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, result as u8 & StatusFlag::Negative as u8 > 0);
        cpu.accumulator = result as u8;
    } else {
        panic!("ADC opcode called with invalid address mode!")
    }
}

/// AND: Performs a logical and with the accumulator and the addressed value, storing the result
/// in the accumulator
fn and(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = bus.read(address);
        cpu.accumulator &= value;
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("AND opcode called with invalid address mode!")
    }
}

/// ASL: Performs a left bit shift on the addressed value or accumulator
fn asl(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    //Wrapped local function to handle both cases
    fn asl_wrapped(cpu: &mut MOS6502, value: u8) -> u8 {
        //Store the 7th bit in the carry bit
        cpu.set_flag(StatusFlag::Carry, value >> 7 == 1);
        let shifted_value = value << 1;
        cpu.set_flag(StatusFlag::Negative, shifted_value & StatusFlag::Negative as u8 > 0);
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        return shifted_value;
    }

    match address_mode_value {
        AddressModeValue::Implied => {
            cpu.accumulator = asl_wrapped(cpu, cpu.accumulator);
        }
        AddressModeValue::AbsoluteAddress(address) => {
            let value = asl_wrapped(cpu, bus.read(address));
            bus.write(address, value);
        }
        _ => panic!("ASL opcode called with invalid address mode!"),
    }
}

/// BCC: Branch if the carry bit is clear
fn bcc(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    return branch(cpu, !cpu.get_flag(StatusFlag::Carry), address_mode_value);
}

/// BCC: Branch if the carry bit is set
fn bcs(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    return branch(cpu, cpu.get_flag(StatusFlag::Carry), address_mode_value);
}

/// BEQ: Branch if the zero bit is set (branch if equal)
fn beq(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    return branch(cpu, cpu.get_flag(StatusFlag::Zero), address_mode_value);
}

/// BIT: Uses the accumulator as a mask pattern to test the bits of a given memory location
fn bit(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = bus.read(address);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator & value == 0);
        cpu.set_flag(StatusFlag::Overflow, value & StatusFlag::Overflow as u8 > 0);
        cpu.set_flag(StatusFlag::Negative, value & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("BIT opcode called with invalid address mode!")
    }
}

/// BMI: Branch if the negative bit is set (branch if negative)
fn bmi(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    return branch(cpu, cpu.get_flag(StatusFlag::Negative), address_mode_value);
}

/// BNE: Branch if the zero bit is clear (branch if not equal)
fn bne(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    return branch(cpu, !cpu.get_flag(StatusFlag::Zero), address_mode_value);
}

/// BPL: Branch if the negative bit is clear (branch if positive)
fn bpl(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    return branch(cpu, !cpu.get_flag(StatusFlag::Negative), address_mode_value);
}

/// BRK: Force an interrupt
fn brk(cpu: &mut MOS6502, bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    //Increase program counter by 1 so it returns to the correct place
    cpu.push_stack_16(bus, cpu.program_counter + 1);
    cpu.set_flag(StatusFlag::Break, true);
    cpu.push_stack(bus, cpu.status_register);
    cpu.set_flag(StatusFlag::InterruptDisable, true);
    cpu.program_counter = super::read_16(bus, super::IRQ_ADDRESS_LOCATION);
}

/// BVC: Branch if the overflow bit is clear
fn bvc(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    return branch(cpu, !cpu.get_flag(StatusFlag::Overflow), address_mode_value);
}

/// BVS: Branch if the overflow bit is set
fn bvs(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    return branch(cpu, cpu.get_flag(StatusFlag::Overflow), address_mode_value);
}

/// CLC: Clear carry bit
fn clc(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.set_flag(StatusFlag::Carry, false);
}

/// CLD: Clear decimal mode bit
fn cld(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.set_flag(StatusFlag::Decimal, false);
}

/// CLD: Clear interrupt disable bit
fn cli(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.set_flag(StatusFlag::InterruptDisable, false);
}

/// CLD: Clear overflow bit
fn clv(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.set_flag(StatusFlag::Overflow, false);
}

/// CMP: Compare accumulator to a value in memory
fn cmp(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    compare(cpu, bus, cpu.accumulator, address_mode_value);
}

/// CPX: Compare x register to a value in memory
fn cpx(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    compare(cpu, bus, cpu.x_register, address_mode_value);
}

/// CPY: Compare y register to a value in memory
fn cpy(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    compare(cpu, bus, cpu.y_register, address_mode_value);
}

/// DEC: Subtract one from the value at the given memory location
fn dec(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = bus.read(address).wrapping_sub(1);
        cpu.set_flag(StatusFlag::Zero, value == 0);
        cpu.set_flag(StatusFlag::Negative, value & StatusFlag::Negative as u8 > 0);
        bus.write(address, value);
    } else {
        panic!("DEC opcode called with invalid address mode!")
    }
}

/// DEX: Subtract one from the x register
fn dex(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::Implied = address_mode_value {
        cpu.x_register = cpu.x_register.wrapping_sub(1);
        cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("DEX opcode called with invalid address mode!")
    }
}

/// DEY: Subtract one from the y register
fn dey(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::Implied = address_mode_value {
        cpu.y_register = cpu.y_register.wrapping_sub(1);
        cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("DEY opcode called with invalid address mode!")
    }
}

/// EOR: Set accumulator to the result of an exclusive or operation with the accumulator and a value from memory
fn eor(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = bus.read(address);
        cpu.accumulator ^= value;
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("EOR opcode called with invalid address mode!")
    }
}

/// INC: Add one to the value at the given memory location
fn inc(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = bus.read(address).wrapping_add(1);
        cpu.set_flag(StatusFlag::Zero, value == 0);
        cpu.set_flag(StatusFlag::Negative, value & StatusFlag::Negative as u8 > 0);
        bus.write(address, value);
    } else {
        panic!("INC opcode called with invalid address mode!")
    }
}

/// INX: Add one to the x register
fn inx(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::Implied = address_mode_value {
        cpu.x_register = cpu.x_register.wrapping_add(1);
        cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("INX opcode called with invalid address mode!")
    }
}

/// INX: Add one to the y register
fn iny(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::Implied = address_mode_value {
        cpu.y_register = cpu.y_register.wrapping_add(1);
        cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("INY opcode called with invalid address mode!")
    }
}

/// JMP: Set the program counter to the given address
fn jmp(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        cpu.program_counter = address;
    } else {
        panic!("JMP opcode called with invalid address mode!")
    }
}

/// JSR: Puts the current program counter value on the stack and then jumps to the given address
fn jsr(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        cpu.push_stack_16(bus, cpu.program_counter - 1);
        cpu.program_counter = address;
    } else {
        panic!("JSR opcode called with invalid address mode!")
    }
}

/// LDA: Load a value into the accumulator from a memory address
//TODO: Come up with a way of sharing code across load opcodes
fn lda(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        cpu.accumulator = bus.read(address);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("LDA opcode called with invalid address mode!")
    }
}

/// LDX: Load a value into the x register from a memory address
fn ldx(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        cpu.x_register = bus.read(address);
        cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("LDX opcode called with invalid address mode!")
    }
}

/// LDY: Load a value into the y register from a memory address
fn ldy(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        cpu.y_register = bus.read(address);
        cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("LDY opcode called with invalid address mode!")
    }
}

/// LSR: Performs a right bit shift on the given value
fn lsr(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    //Wrapped local function to handle both cases
    fn lsr_wrapped(cpu: &mut MOS6502, value: u8) -> u8 {
        //Store the 0th bit in the carry bit
        cpu.set_flag(StatusFlag::Carry, value & 1 == 1);
        let shifted_value = value >> 1;
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        cpu.set_flag(StatusFlag::Negative, false); //The result can never have bit 7 set
        return shifted_value;
    }

    match address_mode_value {
        AddressModeValue::Implied => {
            cpu.accumulator = lsr_wrapped(cpu, cpu.accumulator);
        }
        AddressModeValue::AbsoluteAddress(address) => {
            let value = lsr_wrapped(cpu, bus.read(address));
            bus.write(address, value);
        }
        _ => panic!("LSR opcode called with invalid address mode!"),
    }
}

/// NOP: No operation
fn nop(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
}

/// ORA: The accumulator is set to the result of a inclusive or operation applied to the accumulator and a memory value
fn ora(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = bus.read(address);
        cpu.accumulator |= value;
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
    } else {
        panic!("ORA opcode called with invalid address mode!")
    }
}

/// PHA: Push the value of the accumulator onto the stack
fn pha(cpu: &mut MOS6502, bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.push_stack(bus, cpu.accumulator);
}

/// PHP: Push the value of the status byte onto the stack
fn php(cpu: &mut MOS6502, bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    let status = cpu.status_register;
    // PHP sets both Break flags, but only to the version of the flags pushed onto the stack
    cpu.push_stack(bus, status | StatusFlag::Break as u8);
}

/// PLA: Sets the accumulator to a value popped off the top of the stack
fn pla(cpu: &mut MOS6502, bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.accumulator = cpu.pop_stack(bus);
    cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
}

/// PLP: Sets the status byte to a value popped off the top of the stack
fn plp(cpu: &mut MOS6502, bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    let status = cpu.pop_stack(bus);
    // Set all the flags except the Break flags, which remain as they were
    cpu.status_register = (cpu.status_register & (StatusFlag::Break as u8)) | (status & !(StatusFlag::Break as u8));
}

/// ROL: Rotate the bits of the given value to the left
fn rol(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    //Wrapped local function to handle both cases
    fn rol_wrapped(cpu: &mut MOS6502, value: u8) -> u8 {
        //Store the 7th bit in the carry bit
        let carry = cpu.get_flag(StatusFlag::Carry) as u8;
        cpu.set_flag(StatusFlag::Carry, value >> 7 == 1);
        let shifted_value = (value << 1) + carry;
        cpu.set_flag(StatusFlag::Negative, shifted_value & StatusFlag::Negative as u8 > 0);
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        return shifted_value;
    }

    match address_mode_value {
        AddressModeValue::Implied => {
            cpu.accumulator = rol_wrapped(cpu, cpu.accumulator);
        }
        AddressModeValue::AbsoluteAddress(address) => {
            let value = rol_wrapped(cpu, bus.read(address));
            bus.write(address, value);
        }
        _ => panic!("ROL opcode called with invalid address mode!"),
    }
}

/// ROR: Rotate the bits of the given value to the right
fn ror(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    //Wrapped local function to handle both cases
    fn ror_wrapped(cpu: &mut MOS6502, value: u8) -> u8 {
        //Store the 7th bit in the carry bit
        let carry = cpu.get_flag(StatusFlag::Carry) as u8;
        cpu.set_flag(StatusFlag::Carry, value & 1 == 1);
        let shifted_value = (value >> 1) + (carry << 7);
        cpu.set_flag(StatusFlag::Negative, shifted_value & StatusFlag::Negative as u8 > 0);
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        return shifted_value;
    }

    match address_mode_value {
        AddressModeValue::Implied => {
            cpu.accumulator = ror_wrapped(cpu, cpu.accumulator);
        }
        AddressModeValue::AbsoluteAddress(address) => {
            let value = ror_wrapped(cpu, bus.read(address));
            bus.write(address, value);
        }
        _ => panic!("ROR opcode called with invalid address mode!"),
    }
}

/// RTI: Returns from an interrupt, reversing the operations performed by the BRK instruction
fn rti(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    // Reuse other paths
    plp(cpu, bus, address_mode_value);
    cpu.program_counter = cpu.pop_stack_16(bus);
}

/// RTS: Returns from a subroutine, taking the value of the program counter from the stack
fn rts(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::Implied = address_mode_value {
        cpu.program_counter = cpu.pop_stack_16(bus) + 1;
    } else {
        panic!("RTS opcode called with invalid address mode!")
    }
}

/// SBC: Subtracts a value and the opposite of the carry bit from the accumulator.
/// The carry flag is expected to be set for one off subtraction.
// TODO: Investigate how to reuse more of the adc code
fn sbc(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = !bus.read(address);
        let result: u16;

        // Only run if the CPU is not built in NES mode
        // TODO: Make sure cpu is removed as dead code in nes builds
        if cfg!(feature = "binary_coded_decimal") && cpu.get_flag(StatusFlag::Decimal) {
            let mut sum = cpu.accumulator.wrapping_add(value).wrapping_add(cpu.get_flag(StatusFlag::Carry) as u8);
            if (cpu.accumulator & 0x0f) + (value & 0x0f) + cpu.get_flag(StatusFlag::Carry) as u8 > 0x09 {
                sum = sum.wrapping_sub(0x06);
            }
            if (sum & 0xf0) > 0x90 {
                sum = sum.wrapping_sub(0x60);
                cpu.set_flag(StatusFlag::Carry, false);
            } else {
                cpu.set_flag(StatusFlag::Carry, true);
            }
            result = u16::from(sum);
        } else {
            result = u16::from(cpu.accumulator) + u16::from(value) + cpu.get_flag(StatusFlag::Carry) as u16;
            // Set the Carry flag for chain adding multi byte numbers
            cpu.set_flag(StatusFlag::Carry, result > u16::from(u8::max_value()));
        }
        cpu.set_flag(StatusFlag::Zero, result as u8 == 0);
        // Set the Overflow flag if a signed overflow has occurred
        cpu.set_flag(StatusFlag::Overflow, (!(cpu.accumulator ^ value) & (cpu.accumulator ^ result as u8) & StatusFlag::Negative as u8) > 0);
        // Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, result as u8 & StatusFlag::Negative as u8 > 0);
        cpu.accumulator = result as u8;
    } else {
        panic!("SBC opcode called with invalid address mode!")
    }
}

/// SEC: Sets the carry bit to one
fn sec(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.set_flag(StatusFlag::Carry, true);
}

/// SED: Sets the decimal bit to one
fn sed(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.set_flag(StatusFlag::Decimal, true);
}

/// SEI: Sets the interrupt disable bit to one
fn sei(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.set_flag(StatusFlag::InterruptDisable, true);
}

/// STA: Store the accumulator in the given memory address
fn sta(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        bus.write(address, cpu.accumulator);
    } else {
        panic!("STA opcode called with invalid address mode!")
    }
}

/// STX: Store the x register in the given memory address
fn stx(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        bus.write(address, cpu.x_register);
    } else {
        panic!("STX opcode called with invalid address mode!")
    }
}

/// STY: Store the y register in the given memory address
fn sty(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        bus.write(address, cpu.y_register);
    } else {
        panic!("STY opcode called with invalid address mode!")
    }
}

/// TAX: Transfer the accumulator into the x register
fn tax(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.x_register = cpu.accumulator;
    cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
}

/// TAY: Transfer the accumulator into the y register
fn tay(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.y_register = cpu.accumulator;
    cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
}

/// TSS: Transfer the stack pointer into the x register
fn tsx(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.x_register = cpu.stack_pointer;
    cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
}

/// TXA: Transfer the x register into the accumulator
fn txa(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.accumulator = cpu.x_register;
    cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
}

/// TXS: Transfer the x register into the stack pointer
fn txs(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.stack_pointer = cpu.x_register;
}

/// TYA: Transfer the y register into the accumulator
fn tya(cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    cpu.accumulator = cpu.y_register;
    cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
}

//HELPERS------------------------------------------------------------------------------------------

/// Function to convert a byte to a u16 when the value is signed
fn signed_8_bit_to_16(value: u8) -> u16 {
    let mut value = u16::from(value);
    if value & 0x80 > 0 {
        value |= 0xff00;
    }
    return value;
}

/// General purpose function for branch opcodes
fn branch(cpu: &mut MOS6502, branch_condition: bool, address_mode_value: AddressModeValue) {
    if let AddressModeValue::RelativeAddress(relative_address) = address_mode_value {
        let address = signed_8_bit_to_16(relative_address).wrapping_add(cpu.program_counter);

        if branch_condition {
            if address & 0xff00 != cpu.program_counter & 0xff00 {
                cpu.remaining_cycles += 2;
            } else {
                cpu.remaining_cycles += 1;
            }
            cpu.program_counter = address;
        }
    } else {
        panic!("Branching opcode called with invalid address mode!")
    }
}

/// General purpose function for comparison opcodes
fn compare(cpu: &mut MOS6502, bus: &mut dyn Interface6502, register: u8, address_mode_value: AddressModeValue) -> u8 {
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        let value = bus.read(address);
        cpu.set_flag(StatusFlag::Carry, register >= value);
        cpu.set_flag(StatusFlag::Zero, register == value);
        let register = register.wrapping_sub(value);
        cpu.set_flag(StatusFlag::Negative, register & StatusFlag::Negative as u8 > 0);
        return  register;
    } else {
        panic!("Compare opcode called with invalid address mode!")
    }
}

// TESTS--------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    #![allow(unused_variables, unused_mut)] //Allow some warnings for test code

    use super::*;
    use crate::{StatusFlag, MOS6502};
    use crate::address_modes::AddressModeValue;
    use crate::test_utilities::StubInterface6502;

    #[test]
    fn test_adc() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x09,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x10,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x1a,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);

        adc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_adc_zero_carry_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        adc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_adc_overflow_negative_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x7f,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x80,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Overflow, true);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        adc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_adc_decimal_mode() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x09,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x09,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x19,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);

        adc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_adc_decimal_mode_zero_carry_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x98,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        adc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_adc_decimal_mode_overflow_negative_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x75,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x06,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x81,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        adc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    // TODO: Add tests for these decimal mode conditions from http://www.oxyron.de/html/opcodes02.html
    /*
        $00+$0F=$15 (an easy way to convert a hex-digit into BCD...)
        $00+$1F=$25 (can be claimed as being "ok" since 10+$0F=25)
        $10+$1F=$35 ("ok")
        $05+$1F=$2A (a non-BCD result, still somewhat "ok" since 5+10+$0F=20+$0A)
        $0F+$0A=$1F ("ok", since $0F+$0A=$0F+10)
        $0F+$0B=$10 (now, this is plain bullshit!)
    */

    #[test]
    fn test_and() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x95,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x80,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x80,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        and(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_and_zero_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xf0,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        and(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_asl() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x4f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x4f << 1);
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        asl(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_asl_accumulator_zero_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x80,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Carry, true);
        cpu_expected.set_flag(StatusFlag::Zero, true);

        asl(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_brk() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x80,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x4000,
            stack_pointer: 0xfd,
            status_register: 0x81,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0xfffe => 0x01,
                0xffff => 0x80,
                _ => panic!("Unintended Address Accessed {:4X}", address),
            },
            write: |address, data, write_count| match address {
                0x01fd => assert_eq!(data, 0x40),
                0x01fc => assert_eq!(data, 0x01),
                0x01fb => assert_eq!(data, 0x81 | StatusFlag::Break as u8),
                _ => panic!("Unintended Address Accessed {:4X}", address),
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            program_counter: 0x8001,
            stack_pointer: 0xfa,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Break, true);
        cpu_expected.set_flag(StatusFlag::InterruptDisable, true);

        brk(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    //Tests of the branch function are used in place of testing the individual branch conditions
    #[test]
    fn test_branch() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x00fb,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            program_counter: 0x0100,
            remaining_cycles: 2,
            ..cpu_initial.clone()
        };

        branch(&mut cpu_initial, true, AddressModeValue::RelativeAddress(0x05));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_branch_backwards() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x000a,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            program_counter: 0x0005,
            remaining_cycles: 1,
            ..cpu_initial.clone()
        };

        branch(&mut cpu_initial, true, AddressModeValue::RelativeAddress(0xfb));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_branch_fail() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x000a,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };

        branch(&mut cpu_initial, false, AddressModeValue::RelativeAddress(0xfb));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_bit() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xf0,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x000a,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        bit(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_bit_negative_overflow_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x0f,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x000a,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0xc0,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        bit(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_compare() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0xff,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        compare(&mut cpu_initial, &mut stub_bus, 0xff, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_compare_less() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x0f,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x10,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        compare(&mut cpu_initial, &mut stub_bus, 0x0f, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dec() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x00);
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        dec(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dec_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x00,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0xff);
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        dec(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dex() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x01,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        dex(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dex_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0xff,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        dex(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dey() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x01,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            y_register: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        dey(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dey_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            y_register: 0xff,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        dey(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_eor() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x80,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x90,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        eor(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_eor_zero_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0xff,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        eor(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_inc() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0xff,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x00);
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        inc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_inc_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x7f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x80);
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        inc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_inx() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0xff,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        inx(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_inx_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x7f,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0x80,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        inx(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_iny() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0xff,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            y_register: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        iny(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_iny_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x7f,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            y_register: 0x80,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        iny(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_jmp() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            program_counter: 0x00ff,
            ..cpu_initial.clone()
        };

        jmp(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_jsr() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x00bb,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| match address {
                0x01fd => assert_eq!(data, 0x00),
                0x01fc => assert_eq!(data, 0xba),
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            program_counter: 0x00ff,
            stack_pointer: 0xfb,
            ..cpu_initial.clone()
        };

        jsr(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lda() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0xff,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0xff,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        lda(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lda_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x00,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        lda(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ldx() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0xff,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0xff,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        ldx(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ldx_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0xff,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x00,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        ldx(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ldy() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0xff,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            y_register: 0xff,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        ldy(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ldy_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0xff,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x00,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            y_register: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        ldy(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lsr() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0xff,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0xff >> 1);
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Carry, true);

        lsr(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lsr_accumulator_zero_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x01,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        lsr(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ora() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x80,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x90,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        ora(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ora_zero_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x00,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        ora(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_pha() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| match address {
                0x01fd => assert_eq!(data, 0x10),
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            stack_pointer: 0xfc,
            ..cpu_initial.clone()
        };

        pha(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_php() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| match address {
                0x01fd => assert_eq!(data, StatusFlag::Break as u8),
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            stack_pointer: 0xfc,
            ..cpu_initial.clone()
        };
        //cpu_expected.set_flag(StatusFlag::Break, true);

        php(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_pla() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfc,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x01fd => 0xff,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            accumulator: 0xff,
            stack_pointer: 0xfd,
            status_register: 0x80,
            ..cpu_initial.clone()
        };

        pla(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_plp() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfc,
            status_register: 0x24,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x01fd => 0xb1,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            status_register: 0xa1,
            stack_pointer: 0xfd,
            ..cpu_initial.clone()
        };

        plp(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rol() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x41,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x83);
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Carry, false);

        rol(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rol_accumulator() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x80,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        rol(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ror() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x02,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x81);
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial.clone() };
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Carry, false);

        ror(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ror_accumulator() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x01,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        ror(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rti() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x01,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfa,
            status_register: 0x20,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x01fb => 0xf1,
                0x01fc => 0x01,
                0x01fd => 0x40,
                _ => panic!("Unintended Address Accessed: {:4X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            program_counter: 0x4001,
            status_register: 0xe1,
            stack_pointer: 0xfd,
            ..cpu_initial.clone()
        };

        rti(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rts() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x00bb,
            stack_pointer: 0xfb,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x01fc => 0x00,
                0x01fd => 0x10,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            program_counter: 0x1001,
            stack_pointer: 0xfd,
            ..cpu_initial.clone()
        };

        rts(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sbc() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x08,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            accumulator: 0x08,
            ..cpu_initial.clone()
        };

        sbc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sbc_overflow() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x81,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x02,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x7f,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        sbc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sbc_zero() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x10,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        sbc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sbc_carry_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x11,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0xff,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        sbc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_sbc_decimal() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x12,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x06,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            accumulator: 0x06,
            ..cpu_initial.clone()
        };

        sbc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_sbc_decimal_carry_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x12,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x18,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x94,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        sbc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sta() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x01,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 { ..cpu_initial.clone() };

        sta(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_stx() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x01,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 { ..cpu_initial.clone() };

        stx(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sty() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x01,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 { ..cpu_initial.clone() };

        sty(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tax() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x01,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        tax(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tay() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x01,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            y_register: 0x00,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        tay(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tsx() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0xfd,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        tsx(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_txa() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x80,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x80,
            ..cpu_initial.clone()
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        txa(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_txs() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            stack_pointer: 0x00,
            ..cpu_initial.clone()
        };

        txs(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tya() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x01,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            }, ..Default::default()
        };

        let cpu_expected = MOS6502 {
            accumulator: 0x01,
            ..cpu_initial.clone()
        };

        tya(&mut cpu_initial, &mut stub_bus, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }
}
