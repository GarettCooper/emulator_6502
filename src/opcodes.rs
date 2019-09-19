///OPCODES---------------------------------------------------------------------------------------
///  This module contains all of the opcode functions to prevent the parent module from being primarily full of them
///  An opcode function represents one of the 6502's opcodes. An opcode function is passed the
///  address mode to use and returns the number of extra cycles that address mode has taken

mod illegal;

use super::MOS6502;
use super::StatusFlag;
use super::AddressModeFunction;
use super::OpcodeFunction;
use super::address_modes::*;
use illegal::*;

#[derive(Clone, Copy)]
pub (super) struct Opcode{
    function: OpcodeFunction,
    address_mode: AddressModeFunction,
    cycles: u8
}

pub (super) static OPCODE_TABLE: [Opcode; 256] = [
    Opcode{ function: brk, address_mode: implied, cycles: 7 },		//0x0
    Opcode{ function: ora, address_mode: indirect_x, cycles: 6 },		//0x1
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x2
    Opcode{ function: slo, address_mode: indirect_x, cycles: 8 },		//0x3
    Opcode{ function: nop, address_mode: zero_page, cycles: 3 },		//0x4
    Opcode{ function: ora, address_mode: zero_page, cycles: 3 },		//0x5
    Opcode{ function: asl, address_mode: zero_page, cycles: 5 },		//0x6
    Opcode{ function: slo, address_mode: zero_page, cycles: 5 },		//0x7
    Opcode{ function: php, address_mode: implied, cycles: 3 },		//0x8
    Opcode{ function: ora, address_mode: immediate, cycles: 2 },		//0x9
    Opcode{ function: asl, address_mode: implied, cycles: 2 },		//0xa
    Opcode{ function: anc, address_mode: immediate, cycles: 2 },		//0xb
    Opcode{ function: nop, address_mode: absolute, cycles: 4 },		//0xc
    Opcode{ function: ora, address_mode: absolute, cycles: 4 },		//0xd
    Opcode{ function: asl, address_mode: absolute, cycles: 6 },		//0xe
    Opcode{ function: slo, address_mode: absolute, cycles: 6 },		//0xf
    Opcode{ function: bpl, address_mode: relative, cycles: 2 },		//0x10
    Opcode{ function: ora, address_mode: indirect_y, cycles: 5 },		//0x11
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x12
    Opcode{ function: slo, address_mode: indirect_y, cycles: 8 },		//0x13
    Opcode{ function: nop, address_mode: zero_page_x, cycles: 4 },		//0x14
    Opcode{ function: ora, address_mode: zero_page_x, cycles: 4 },		//0x15
    Opcode{ function: asl, address_mode: zero_page_x, cycles: 6 },		//0x16
    Opcode{ function: slo, address_mode: zero_page_x, cycles: 6 },		//0x17
    Opcode{ function: clc, address_mode: implied, cycles: 2 },		//0x18
    Opcode{ function: ora, address_mode: absolute_y, cycles: 4 },		//0x19
    Opcode{ function: nop, address_mode: implied, cycles: 2 },		//0x1a
    Opcode{ function: slo, address_mode: absolute_y, cycles: 7 },		//0x1b
    Opcode{ function: nop, address_mode: absolute_x, cycles: 4 },		//0x1c
    Opcode{ function: ora, address_mode: absolute_x, cycles: 4 },		//0x1d
    Opcode{ function: asl, address_mode: absolute_x, cycles: 7 },		//0x1e
    Opcode{ function: slo, address_mode: absolute_x, cycles: 7 },		//0x1f
    Opcode{ function: jsr, address_mode: absolute, cycles: 6 },		//0x20
    Opcode{ function: and, address_mode: indirect_x, cycles: 6 },		//0x21
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x22
    Opcode{ function: rla, address_mode: indirect_x, cycles: 8 },		//0x23
    Opcode{ function: bit, address_mode: zero_page, cycles: 3 },		//0x24
    Opcode{ function: and, address_mode: zero_page, cycles: 3 },		//0x25
    Opcode{ function: rol, address_mode: zero_page, cycles: 5 },		//0x26
    Opcode{ function: rla, address_mode: zero_page, cycles: 5 },		//0x27
    Opcode{ function: plp, address_mode: implied, cycles: 4 },		//0x28
    Opcode{ function: and, address_mode: immediate, cycles: 2 },		//0x29
    Opcode{ function: rol, address_mode: implied, cycles: 2 },		//0x2a
    Opcode{ function: anc, address_mode: immediate, cycles: 2 },		//0x2b
    Opcode{ function: bit, address_mode: absolute, cycles: 4 },		//0x2c
    Opcode{ function: and, address_mode: absolute, cycles: 4 },		//0x2d
    Opcode{ function: rol, address_mode: absolute, cycles: 6 },		//0x2e
    Opcode{ function: rla, address_mode: absolute, cycles: 6 },		//0x2f
    Opcode{ function: bmi, address_mode: relative, cycles: 2 },		//0x30
    Opcode{ function: and, address_mode: indirect_y, cycles: 5 },		//0x31
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x32
    Opcode{ function: rla, address_mode: indirect_y, cycles: 8 },		//0x33
    Opcode{ function: nop, address_mode: zero_page_x, cycles: 4 },		//0x34
    Opcode{ function: and, address_mode: zero_page_x, cycles: 4 },		//0x35
    Opcode{ function: rol, address_mode: zero_page_x, cycles: 6 },		//0x36
    Opcode{ function: rla, address_mode: zero_page_x, cycles: 6 },		//0x37
    Opcode{ function: sec, address_mode: implied, cycles: 2 },		//0x38
    Opcode{ function: and, address_mode: absolute_y, cycles: 4 },		//0x39
    Opcode{ function: nop, address_mode: implied, cycles: 2 },		//0x3a
    Opcode{ function: rla, address_mode: absolute_y, cycles: 7 },		//0x3b
    Opcode{ function: nop, address_mode: absolute_x, cycles: 4 },		//0x3c
    Opcode{ function: and, address_mode: absolute_x, cycles: 4 },		//0x3d
    Opcode{ function: rol, address_mode: absolute_x, cycles: 7 },		//0x3e
    Opcode{ function: rla, address_mode: absolute_x, cycles: 7 },		//0x3f
    Opcode{ function: rti, address_mode: implied, cycles: 6 },		//0x40
    Opcode{ function: eor, address_mode: indirect_x, cycles: 6 },		//0x41
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x42
    Opcode{ function: sre, address_mode: indirect_x, cycles: 8 },		//0x43
    Opcode{ function: nop, address_mode: zero_page, cycles: 3 },		//0x44
    Opcode{ function: eor, address_mode: zero_page, cycles: 3 },		//0x45
    Opcode{ function: lsr, address_mode: zero_page, cycles: 5 },		//0x46
    Opcode{ function: sre, address_mode: zero_page, cycles: 5 },		//0x47
    Opcode{ function: pha, address_mode: implied, cycles: 3 },		//0x48
    Opcode{ function: eor, address_mode: immediate, cycles: 2 },		//0x49
    Opcode{ function: lsr, address_mode: implied, cycles: 2 },		//0x4a
    Opcode{ function: alr, address_mode: immediate, cycles: 2 },		//0x4b
    Opcode{ function: jmp, address_mode: absolute, cycles: 3 },		//0x4c
    Opcode{ function: eor, address_mode: absolute, cycles: 4 },		//0x4d
    Opcode{ function: lsr, address_mode: absolute, cycles: 6 },		//0x4e
    Opcode{ function: sre, address_mode: absolute, cycles: 6 },		//0x4f
    Opcode{ function: bvc, address_mode: relative, cycles: 2 },		//0x50
    Opcode{ function: eor, address_mode: indirect_y, cycles: 5 },		//0x51
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x52
    Opcode{ function: sre, address_mode: indirect_y, cycles: 8 },		//0x53
    Opcode{ function: nop, address_mode: zero_page_x, cycles: 4 },		//0x54
    Opcode{ function: eor, address_mode: zero_page_x, cycles: 4 },		//0x55
    Opcode{ function: lsr, address_mode: zero_page_x, cycles: 6 },		//0x56
    Opcode{ function: sre, address_mode: zero_page_x, cycles: 6 },		//0x57
    Opcode{ function: cli, address_mode: implied, cycles: 2 },		//0x58
    Opcode{ function: eor, address_mode: absolute_y, cycles: 4 },		//0x59
    Opcode{ function: nop, address_mode: implied, cycles: 2 },		//0x5a
    Opcode{ function: sre, address_mode: absolute_y, cycles: 7 },		//0x5b
    Opcode{ function: nop, address_mode: absolute_x, cycles: 4 },		//0x5c
    Opcode{ function: eor, address_mode: absolute_x, cycles: 4 },		//0x5d
    Opcode{ function: lsr, address_mode: absolute_x, cycles: 7 },		//0x5e
    Opcode{ function: sre, address_mode: absolute_x, cycles: 7 },		//0x5f
    Opcode{ function: rts, address_mode: implied, cycles: 6 },		//0x60
    Opcode{ function: adc, address_mode: indirect_x, cycles: 6 },		//0x61
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x62
    Opcode{ function: rra, address_mode: indirect_x, cycles: 8 },		//0x63
    Opcode{ function: nop, address_mode: zero_page, cycles: 3 },		//0x64
    Opcode{ function: adc, address_mode: zero_page, cycles: 3 },		//0x65
    Opcode{ function: ror, address_mode: zero_page, cycles: 5 },		//0x66
    Opcode{ function: rra, address_mode: zero_page, cycles: 5 },		//0x67
    Opcode{ function: pla, address_mode: implied, cycles: 4 },		//0x68
    Opcode{ function: adc, address_mode: immediate, cycles: 2 },		//0x69
    Opcode{ function: ror, address_mode: implied, cycles: 2 },		//0x6a
    Opcode{ function: arr, address_mode: immediate, cycles: 2 },		//0x6b
    Opcode{ function: jmp, address_mode: indirect, cycles: 5 },		//0x6c
    Opcode{ function: adc, address_mode: absolute, cycles: 4 },		//0x6d
    Opcode{ function: ror, address_mode: absolute, cycles: 6 },		//0x6e
    Opcode{ function: rra, address_mode: absolute, cycles: 6 },		//0x6f
    Opcode{ function: bvs, address_mode: relative, cycles: 2 },		//0x70
    Opcode{ function: adc, address_mode: indirect_y, cycles: 5 },		//0x71
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x72
    Opcode{ function: rra, address_mode: indirect_y, cycles: 8 },		//0x73
    Opcode{ function: nop, address_mode: zero_page_x, cycles: 4 },		//0x74
    Opcode{ function: adc, address_mode: zero_page_x, cycles: 4 },		//0x75
    Opcode{ function: ror, address_mode: zero_page_x, cycles: 6 },		//0x76
    Opcode{ function: rra, address_mode: zero_page_x, cycles: 6 },		//0x77
    Opcode{ function: sei, address_mode: implied, cycles: 2 },		//0x78
    Opcode{ function: adc, address_mode: absolute_y, cycles: 4 },		//0x79
    Opcode{ function: nop, address_mode: implied, cycles: 2 },		//0x7a
    Opcode{ function: rra, address_mode: absolute_y, cycles: 7 },		//0x7b
    Opcode{ function: nop, address_mode: absolute_x, cycles: 4 },		//0x7c
    Opcode{ function: adc, address_mode: absolute_x, cycles: 4 },		//0x7d
    Opcode{ function: ror, address_mode: absolute_x, cycles: 7 },		//0x7e
    Opcode{ function: rra, address_mode: absolute_x, cycles: 7 },		//0x7f
    Opcode{ function: nop, address_mode: immediate, cycles: 2 },		//0x80
    Opcode{ function: sta, address_mode: indirect_x, cycles: 6 },		//0x81
    Opcode{ function: nop, address_mode: immediate, cycles: 2 },		//0x82
    Opcode{ function: sax, address_mode: indirect_x, cycles: 6 },		//0x83
    Opcode{ function: sty, address_mode: zero_page, cycles: 3 },		//0x84
    Opcode{ function: sta, address_mode: zero_page, cycles: 3 },		//0x85
    Opcode{ function: stx, address_mode: zero_page, cycles: 3 },		//0x86
    Opcode{ function: sax, address_mode: zero_page, cycles: 3 },		//0x87
    Opcode{ function: dey, address_mode: implied, cycles: 2 },		//0x88
    Opcode{ function: nop, address_mode: immediate, cycles: 2 },		//0x89
    Opcode{ function: txa, address_mode: implied, cycles: 2 },		//0x8a
    Opcode{ function: xaa, address_mode: immediate, cycles: 2 },		//0x8b
    Opcode{ function: sty, address_mode: absolute, cycles: 4 },		//0x8c
    Opcode{ function: sta, address_mode: absolute, cycles: 4 },		//0x8d
    Opcode{ function: stx, address_mode: absolute, cycles: 4 },		//0x8e
    Opcode{ function: sax, address_mode: absolute, cycles: 4 },		//0x8f
    Opcode{ function: bcc, address_mode: relative, cycles: 2 },		//0x90
    Opcode{ function: sta, address_mode: indirect_y, cycles: 6 },		//0x91
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0x92
    Opcode{ function: ahx, address_mode: indirect_y, cycles: 6 },		//0x93
    Opcode{ function: sty, address_mode: zero_page_x, cycles: 4 },		//0x94
    Opcode{ function: sta, address_mode: zero_page_x, cycles: 4 },		//0x95
    Opcode{ function: stx, address_mode: zero_page_y, cycles: 4 },		//0x96
    Opcode{ function: sax, address_mode: zero_page_y, cycles: 4 },		//0x97
    Opcode{ function: tya, address_mode: implied, cycles: 2 },		//0x98
    Opcode{ function: sta, address_mode: absolute_y, cycles: 5 },		//0x99
    Opcode{ function: txs, address_mode: implied, cycles: 2 },		//0x9a
    Opcode{ function: tas, address_mode: absolute_y, cycles: 5 },		//0x9b
    Opcode{ function: shy, address_mode: absolute_x, cycles: 5 },		//0x9c
    Opcode{ function: sta, address_mode: absolute_x, cycles: 5 },		//0x9d
    Opcode{ function: shx, address_mode: absolute_y, cycles: 5 },		//0x9e
    Opcode{ function: ahx, address_mode: absolute_y, cycles: 5 },		//0x9f
    Opcode{ function: ldy, address_mode: immediate, cycles: 2 },		//0xa0
    Opcode{ function: lda, address_mode: indirect_x, cycles: 6 },		//0xa1
    Opcode{ function: ldx, address_mode: immediate, cycles: 2 },		//0xa2
    Opcode{ function: lax, address_mode: indirect_x, cycles: 6 },		//0xa3
    Opcode{ function: ldy, address_mode: zero_page, cycles: 3 },		//0xa4
    Opcode{ function: lda, address_mode: zero_page, cycles: 3 },		//0xa5
    Opcode{ function: ldx, address_mode: zero_page, cycles: 3 },		//0xa6
    Opcode{ function: lax, address_mode: zero_page, cycles: 3 },		//0xa7
    Opcode{ function: tay, address_mode: implied, cycles: 2 },		//0xa8
    Opcode{ function: lda, address_mode: immediate, cycles: 2 },		//0xa9
    Opcode{ function: tax, address_mode: implied, cycles: 2 },		//0xaa
    Opcode{ function: lax, address_mode: immediate, cycles: 2 },		//0xab
    Opcode{ function: ldy, address_mode: absolute, cycles: 4 },		//0xac
    Opcode{ function: lda, address_mode: absolute, cycles: 4 },		//0xad
    Opcode{ function: ldx, address_mode: absolute, cycles: 4 },		//0xae
    Opcode{ function: lax, address_mode: absolute, cycles: 4 },		//0xaf
    Opcode{ function: bcs, address_mode: relative, cycles: 2 },		//0xb0
    Opcode{ function: lda, address_mode: indirect_y, cycles: 5 },		//0xb1
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0xb2
    Opcode{ function: lax, address_mode: indirect_y, cycles: 5 },		//0xb3
    Opcode{ function: ldy, address_mode: zero_page_x, cycles: 4 },		//0xb4
    Opcode{ function: lda, address_mode: zero_page_x, cycles: 4 },		//0xb5
    Opcode{ function: ldx, address_mode: zero_page_y, cycles: 4 },		//0xb6
    Opcode{ function: lax, address_mode: zero_page_y, cycles: 4 },		//0xb7
    Opcode{ function: clv, address_mode: implied, cycles: 2 },		//0xb8
    Opcode{ function: lda, address_mode: absolute_y, cycles: 4 },		//0xb9
    Opcode{ function: tsx, address_mode: implied, cycles: 2 },		//0xba
    Opcode{ function: las, address_mode: absolute_y, cycles: 4 },		//0xbb
    Opcode{ function: ldy, address_mode: absolute_x, cycles: 4 },		//0xbc
    Opcode{ function: lda, address_mode: absolute_x, cycles: 4 },		//0xbd
    Opcode{ function: ldx, address_mode: absolute_y, cycles: 4 },		//0xbe
    Opcode{ function: lax, address_mode: absolute_y, cycles: 4 },		//0xbf
    Opcode{ function: cpy, address_mode: immediate, cycles: 2 },		//0xc0
    Opcode{ function: cmp, address_mode: indirect_x, cycles: 6 },		//0xc1
    Opcode{ function: nop, address_mode: immediate, cycles: 2 },		//0xc2
    Opcode{ function: dcp, address_mode: indirect_x, cycles: 8 },		//0xc3
    Opcode{ function: cpy, address_mode: zero_page, cycles: 3 },		//0xc4
    Opcode{ function: cmp, address_mode: zero_page, cycles: 3 },		//0xc5
    Opcode{ function: dec, address_mode: zero_page, cycles: 5 },		//0xc6
    Opcode{ function: dcp, address_mode: zero_page, cycles: 5 },		//0xc7
    Opcode{ function: iny, address_mode: implied, cycles: 2 },		//0xc8
    Opcode{ function: cmp, address_mode: immediate, cycles: 2 },		//0xc9
    Opcode{ function: dex, address_mode: implied, cycles: 2 },		//0xca
    Opcode{ function: axs, address_mode: immediate, cycles: 2 },		//0xcb
    Opcode{ function: cpy, address_mode: absolute, cycles: 4 },		//0xcc
    Opcode{ function: cmp, address_mode: absolute, cycles: 4 },		//0xcd
    Opcode{ function: dec, address_mode: absolute, cycles: 6 },		//0xce
    Opcode{ function: dcp, address_mode: absolute, cycles: 6 },		//0xcf
    Opcode{ function: bne, address_mode: relative, cycles: 2 },		//0xd0
    Opcode{ function: cmp, address_mode: indirect_y, cycles: 5 },		//0xd1
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0xd2
    Opcode{ function: dcp, address_mode: indirect_y, cycles: 8 },		//0xd3
    Opcode{ function: nop, address_mode: zero_page_x, cycles: 4 },		//0xd4
    Opcode{ function: cmp, address_mode: zero_page_x, cycles: 4 },		//0xd5
    Opcode{ function: dec, address_mode: zero_page_x, cycles: 6 },		//0xd6
    Opcode{ function: dcp, address_mode: zero_page_x, cycles: 6 },		//0xd7
    Opcode{ function: cld, address_mode: implied, cycles: 2 },		//0xd8
    Opcode{ function: cmp, address_mode: absolute_y, cycles: 4 },		//0xd9
    Opcode{ function: nop, address_mode: implied, cycles: 2 },		//0xda
    Opcode{ function: dcp, address_mode: absolute_y, cycles: 7 },		//0xdb
    Opcode{ function: nop, address_mode: absolute_x, cycles: 4 },		//0xdc
    Opcode{ function: cmp, address_mode: absolute_x, cycles: 4 },		//0xdd
    Opcode{ function: dec, address_mode: absolute_x, cycles: 7 },		//0xde
    Opcode{ function: dcp, address_mode: absolute_x, cycles: 7 },		//0xdf
    Opcode{ function: cpx, address_mode: immediate, cycles: 2 },		//0xe0
    Opcode{ function: sbc, address_mode: indirect_x, cycles: 6 },		//0xe1
    Opcode{ function: nop, address_mode: immediate, cycles: 2 },		//0xe2
    Opcode{ function: isc, address_mode: indirect_x, cycles: 8 },		//0xe3
    Opcode{ function: cpx, address_mode: zero_page, cycles: 3 },		//0xe4
    Opcode{ function: sbc, address_mode: zero_page, cycles: 3 },		//0xe5
    Opcode{ function: inc, address_mode: zero_page, cycles: 5 },		//0xe6
    Opcode{ function: isc, address_mode: zero_page, cycles: 5 },		//0xe7
    Opcode{ function: inx, address_mode: implied, cycles: 2 },		//0xe8
    Opcode{ function: sbc, address_mode: immediate, cycles: 2 },		//0xe9
    Opcode{ function: nop, address_mode: implied, cycles: 2 },		//0xea
    Opcode{ function: sbc, address_mode: immediate, cycles: 2 },		//0xeb
    Opcode{ function: cpx, address_mode: absolute, cycles: 4 },		//0xec
    Opcode{ function: sbc, address_mode: absolute, cycles: 4 },		//0xed
    Opcode{ function: inc, address_mode: absolute, cycles: 6 },		//0xee
    Opcode{ function: isc, address_mode: absolute, cycles: 6 },		//0xef
    Opcode{ function: beq, address_mode: relative, cycles: 2 },		//0xf0
    Opcode{ function: sbc, address_mode: indirect_y, cycles: 5 },		//0xf1
    Opcode{ function: kil, address_mode: implied, cycles: 0 },		//0xf2
    Opcode{ function: isc, address_mode: indirect_y, cycles: 8 },		//0xf3
    Opcode{ function: nop, address_mode: zero_page_x, cycles: 4 },		//0xf4
    Opcode{ function: sbc, address_mode: zero_page_x, cycles: 4 },		//0xf5
    Opcode{ function: inc, address_mode: zero_page_x, cycles: 6 },		//0xf6
    Opcode{ function: isc, address_mode: zero_page_x, cycles: 6 },		//0xf7
    Opcode{ function: sed, address_mode: implied, cycles: 2 },		//0xf8
    Opcode{ function: sbc, address_mode: absolute_y, cycles: 4 },		//0xf9
    Opcode{ function: nop, address_mode: implied, cycles: 2 },		//0xfa
    Opcode{ function: isc, address_mode: absolute_y, cycles: 7 },		//0xfb
    Opcode{ function: nop, address_mode: absolute_x, cycles: 4 },		//0xfc
    Opcode{ function: sbc, address_mode: absolute_x, cycles: 4 },		//0xfd
    Opcode{ function: inc, address_mode: absolute_x, cycles: 7 },		//0xfe
    Opcode{ function: isc, address_mode: absolute_x, cycles: 7 },		//0xff
];



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
    cpu.program_counter += 1; //Increase program counter by 1 so it returns to the correct place
    cpu.push_stack_16(cpu.program_counter);
    cpu.push_stack(cpu.status_register);
    cpu.set_flag(StatusFlag::Break, true);
    cpu.program_counter = cpu.read_16(super::IRQ_ADDRESS_LOCATION);
    return 0;
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
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        cpu.program_counter = address;
        return 0;
    } else {
        panic!("JMP opcode called with invalid address mode!")
    }
}

///JSR: Puts the current program counter value on the stack and then jumps to the given address
fn jsr(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        cpu.push_stack_16(cpu.program_counter - 1);
        cpu.program_counter = address;
        return 0
    } else {
        panic!("JSR opcode called with invalid address mode!")
    }
}

///LDA: Load a value into the accumulator from a memory address
//TODO: Come up with a way of sharing code across load opcodes
fn lda(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        cpu.accumulator = cpu.read(address);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
        return 0
    } else {
        panic!("LDA opcode called with invalid address mode!")
    }
}

///LDX: Load a value into the x register from a memory address
fn ldx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        cpu.x_register = cpu.read(address);
        cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
        return 0
    } else {
        panic!("LDX opcode called with invalid address mode!")
    }
}

///LDY: Load a value into the y register from a memory address
fn ldy(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        cpu.y_register = cpu.read(address);
        cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
        return 0
    } else {
        panic!("LDY opcode called with invalid address mode!")
    }
}

///LSR: Performs a right bit shift on the given value
fn lsr(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    //Wrapped local function to handle both cases
    fn lsr_wrapped(cpu: &mut MOS6502, value: u8) -> u8{
        //Store the 7th bit in the carry bit
        cpu.set_flag(StatusFlag::Carry, value & 1 == 1);
        let shifted_value = value >> 1;
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        return shifted_value;
    }

    match address_mode_value{
        AddressModeValue::Accumulator => {
            cpu.accumulator = lsr_wrapped(cpu, cpu.accumulator);
        },
        AddressModeValue::AbsoluteAddress(address) => {
            let value = lsr_wrapped(cpu, cpu.read(address));
            cpu.write(address, value);
        },
        _ => panic!("LSR opcode called with invalid address mode!")
    }
    return 0; //Operation never adds any extra cycles
}

///NOP: No operation
fn nop(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    return 0;
}

///ORA: The accumulator is set to the result of a inclusive or operation applied to the accumulator and a memory value
fn ora(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        let value = cpu.read(address);
        cpu.accumulator |= value;
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
        return 0; //Operation never adds any extra cycles
    } else {
        panic!("ORA opcode called with invalid address mode!")
    }
}

///PHA: Push the value of the accumulator onto the stack
fn pha(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.push_stack(cpu.accumulator);
    return 0
}

///PHP: Push the value of the status byte onto the stack
fn php(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.push_stack(cpu.status_register);
    return 0
}

///PLA: Sets the accumulator to a value popped off the top of the stack
fn pla(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.accumulator = cpu.pop_stack();
    return 0;
}

///PLP: Sets the status byte to a value popped off the top of the stack
fn plp(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.status_register = cpu.pop_stack();
    return 0;
}

///ROL: Rotate the bits of the given value to the left
fn rol(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    //Wrapped local function to handle both cases
    fn rol_wrapped(cpu: &mut MOS6502, value: u8) -> u8{
        //Store the 7th bit in the carry bit
        let carry = cpu.get_flag(StatusFlag::Carry) as u8;
        cpu.set_flag(StatusFlag::Carry, value >> 7 == 1);
        let shifted_value = (value << 1) + carry;
        cpu.set_flag(StatusFlag::Negative, shifted_value & StatusFlag::Negative as u8 > 0);
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        return shifted_value;
    }

    match address_mode_value{
        AddressModeValue::Accumulator => {
            cpu.accumulator = rol_wrapped(cpu, cpu.accumulator);
        },
        AddressModeValue::AbsoluteAddress(address) => {
            let value = rol_wrapped(cpu, cpu.read(address));
            cpu.write(address, value);
        },
        _ => panic!("ROL opcode called with invalid address mode!")
    }
    return 0; //Operation never adds any extra cycles
}

///ROR: Rotate the bits of the given value to the right
fn ror(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    //Wrapped local function to handle both cases
    fn ror_wrapped(cpu: &mut MOS6502, value: u8) -> u8{
        //Store the 7th bit in the carry bit
        let carry = cpu.get_flag(StatusFlag::Carry) as u8;
        cpu.set_flag(StatusFlag::Carry, value & 1 == 1);
        let shifted_value = (value >> 1) + (carry << 7);
        cpu.set_flag(StatusFlag::Negative, shifted_value & StatusFlag::Negative as u8 > 0);
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        return shifted_value;
    }

    match address_mode_value{
        AddressModeValue::Accumulator => {
            cpu.accumulator = ror_wrapped(cpu, cpu.accumulator);
        },
        AddressModeValue::AbsoluteAddress(address) => {
            let value = ror_wrapped(cpu, cpu.read(address));
            cpu.write(address, value);
        },
        _ => panic!("ROR opcode called with invalid address mode!")
    }
    return 0; //Operation never adds any extra cycles
}

///RTI: Returns from an interrupt, reversing the operations performed by the BRK instruction
fn rti(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.status_register = cpu.pop_stack();
    cpu.program_counter = cpu.pop_stack_16();
    return 0;
}

///RTS: Returns from a subroutine, taking the value of the program counter from the stack
fn rts(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::Implied = address_mode_value{
        cpu.program_counter = cpu.pop_stack_16() + 1;
        return 0
    } else {
        panic!("RTS opcode called with invalid address mode!")
    }
}

///SBC: Subtracts a value and the opposite of the carry bit from the accumulator
/// CARRY FLAG IS EXPECTED TO BE SET FOR ONE OFF SUBTRACTION
//TODO: Investigate how to reuse more of the adc code
fn sbc(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value{
        let value = !cpu.read(address);
        let result: u16;

        //Only run if the CPU is not built in NES mode
        //TODO: Make sure cpu is removed as dead code in nes builds
        if cfg!(not(nes)) && cpu.get_flag(StatusFlag::Decimal){
            let mut sum = cpu.accumulator.wrapping_add(value).wrapping_add(cpu.get_flag(StatusFlag::Carry) as u8);
            if (cpu.accumulator & 0x0f) + (value & 0x0f) + cpu.get_flag(StatusFlag::Carry) as u8 > 0x09{
                sum = sum.wrapping_sub(0x06);
            }
            if (sum & 0xf0) > 0x90{
                sum = sum.wrapping_sub(0x60);
                cpu.set_flag(StatusFlag::Carry, false);
            } else {
                cpu.set_flag(StatusFlag::Carry, true);
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
        println!("cpu.accumulator ^ value:0x{:2X}, cpu.accumulator ^ result:0x{:2X}", (cpu.accumulator ^ value), (cpu.accumulator ^ result as u8));
        //Negative flag is in bit 7, so it can be used to test if the result is negative, because a negative value will also have a 1 in bit 7
        cpu.set_flag(StatusFlag::Negative, result as u8 & StatusFlag::Negative as u8 > 0);
        cpu.accumulator = result as u8;
        return 0; //Operation never adds any extra cycles
    } else {
        panic!("ADC opcode called with invalid address mode!")
    }
}

///SEC: Sets the carry bit to one
fn sec(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.set_flag(StatusFlag::Carry, true);
    return 0;
}

///SED: Sets the decimal bit to one
fn sed(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.set_flag(StatusFlag::Decimal, true);
    return 0;
}

///SEI: Sets the interrupt disable bit to one
fn sei(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.set_flag(StatusFlag::InterruptDisable, true);
    return 0;
}

///STA: Store the accumulator in the given memory address
fn sta(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        cpu.write(address, cpu.accumulator);
        return 0;
    } else {
        panic!("STA opcode called with invalid address mode!")
    }
}

///STX: Store the x register in the given memory address
fn stx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        cpu.write(address, cpu.x_register);
        return 0;
    } else {
        panic!("STX opcode called with invalid address mode!")
    }
}

///STY: Store the y register in the given memory address
fn sty(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
        cpu.write(address, cpu.y_register);
        return 0;
    } else {
        panic!("STY opcode called with invalid address mode!")
    }
}

///TAX: Transfer the accumulator into the x register
fn tax(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.x_register = cpu.accumulator;
    cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
    return 0;
}

///TAY: Transfer the accumulator into the y register
fn tay(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.y_register = cpu.accumulator;
    cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
    return 0;
}

///TSS: Transfer the stack pointer into the x register
fn tsx(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.x_register = cpu.stack_pointer;
    cpu.set_flag(StatusFlag::Negative, cpu.x_register & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.x_register == 0);
    return 0;
}

///TXA: Transfer the x register into the accumulator
fn txa(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.accumulator = cpu.x_register;
    cpu.set_flag(StatusFlag::Negative, cpu.accumulator & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
    return 0;
}

///TXS: Transfer the x register into the stack pointer
fn txs(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.stack_pointer = cpu.x_register;
    return 0;
}

///TYA: Transfer the y register into the accumulator
fn tya(cpu: &mut MOS6502, address_mode_value: AddressModeValue) -> u8{
    cpu.accumulator = cpu.y_register;
    cpu.set_flag(StatusFlag::Negative, cpu.y_register & StatusFlag::Negative as u8 > 0);
    cpu.set_flag(StatusFlag::Zero, cpu.y_register == 0);
    return 0;
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x10,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x01,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x01,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x09,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x01,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x06,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
    fn test_and(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x95,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x80,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x0f,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x4f,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x4f << 1);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
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
            stack_pointer: 0xfd,
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

    #[test]
    fn test_brk(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x80,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x4000,
            stack_pointer: 0xfd,
            status_register: 0x81,
            read: |_address| {
                match _address {
                    0xfffe => 0x01,
                    0xffff => 0x80,
                    _ => panic!("Unintended Address Accessed {:4X}", _address)
                }
            },
            write: |_address, _data| {
                match _address {
                    0x01fd => assert_eq!(_data, 0x40),
                    0x01fc => assert_eq!(_data, 0x01),
                    0x01fb => assert_eq!(_data, 0x81),
                    _ => panic!("Unintended Address Accessed {:4X}", _address)
                }
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{program_counter:0x8001, stack_pointer:0xfa,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Break, true);

        brk(&mut cpu_initial, AddressModeValue::Implied);

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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x0f,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xc0,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xff,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x10,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x01,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x00,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x80,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xff,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xff,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x7f,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
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
            stack_pointer: 0xfd,
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

    #[test]
    fn test_jmp(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{program_counter: 0x00ff,..cpu_initial.clone()};

        assert_eq!(jmp(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_jsr(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x00bb,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {
                match address {
                    0x01fd => assert_eq!(data, 0x00),
                    0x01fc => assert_eq!(data, 0xba),
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{program_counter: 0x00ff, stack_pointer: 0xfb,..cpu_initial.clone()};

        assert_eq!(jsr(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lda(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {0xff},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator: 0xff,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        lda(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lda_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0xff,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {0x00},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        lda(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ldx(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {0xff},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{x_register: 0xff,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        ldx(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ldx_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0xff,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {0x00},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{x_register: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        ldx(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ldy(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {0xff},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{y_register: 0xff,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        ldy(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ldy_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0xff,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {0x00},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{y_register: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        ldy(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lsr(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0xff,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0xff >> 1);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Carry, true);


        lsr(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lsr_accumulator_zero_flag(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x01,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {panic!{"Read function was called"}},
            write: |address, data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);


        lsr(&mut cpu_initial, AddressModeValue::Accumulator);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ora(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x80,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x90,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        ora(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ora_zero_flag(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address{
                    0x00ff => 0x00,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator:0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        ora(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_pha(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |address, data| {
                match address {
                    0x01fd => assert_eq!(data, 0x10),
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{stack_pointer: 0xfc,..cpu_initial.clone()};

        pha(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_php(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |address, data| {
                match address {
                    0x01fd => assert_eq!(data, 0x00),
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{stack_pointer: 0xfc,..cpu_initial.clone()};

        php(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_pla(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfc,
            status_register: 0x00,
            read: |address| {
                match address {
                    0x01fd => 0xff,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{accumulator: 0xff, stack_pointer: 0xfd,..cpu_initial.clone()};

        pla(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_plp(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfc,
            status_register: 0x00,
            read: |address| {
                match address {
                    0x01fd => 0x81,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{status_register: 0x81, stack_pointer: 0xfd,..cpu_initial.clone()};

        plp(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rol(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address {
                    0x00ff => 0x41,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x83);
            },
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Carry, false);

        rol(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rol_accumulator(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x80,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        rol(&mut cpu_initial, AddressModeValue::Accumulator);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ror(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |address| {
                match address {
                    0x00ff => 0x02,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x81);
            },
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Carry, false);

        ror(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ror_accumulator(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x01,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        ror(&mut cpu_initial, AddressModeValue::Accumulator);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rti(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x01,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfa,
            status_register: 0x00,
            read: |_address| {
                match _address {
                    0x01fb => 0xe1,
                    0x01fc => 0x01,
                    0x01fd => 0x40,
                    _ => panic!("Unintended Address Accessed: {:4X}", _address)
                }
            },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{program_counter: 0x4001, status_register: 0xe1, stack_pointer: 0xfd ,..cpu_initial.clone()};

        rti(&mut cpu_initial, AddressModeValue::Accumulator);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rts(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x00bb,
            stack_pointer: 0xfb,
            status_register: 0x00,
            read: |address| {
                match address {
                    0x01fc => 0x00,
                    0x01fd => 0x10,
                    _ => panic!("Unintended Address Accessed: 0x{:X}", address)
                }
            },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{program_counter: 0x1001, stack_pointer: 0xfd,..cpu_initial.clone()};

        assert_eq!(rts(&mut cpu_initial,AddressModeValue::Implied), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sbc(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| { 0x08 },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let cpu_expected = MOS6502{accumulator: 0x08,..cpu_initial.clone()};

        assert_eq!(sbc(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sbc_overflow(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x81,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| { 0x02 },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{accumulator: 0x7f,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        assert_eq!(sbc(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sbc_zero(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| { 0x10 },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{accumulator: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        assert_eq!(sbc(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sbc_carry_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x10,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| { 0x11 },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{accumulator: 0xff,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Carry, false);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        assert_eq!(sbc(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(not(nes))]
    fn test_sbc_decimal(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x12,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| { 0x06 },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{accumulator: 0x06,..cpu_initial.clone()};

        assert_eq!(sbc(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(not(nes))]
    fn test_sbc_decimal_carry_negative(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x12,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| { 0x18 },
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut cpu_expected = MOS6502{accumulator: 0x94,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Carry, false);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        assert_eq!(sbc(&mut cpu_initial,AddressModeValue::AbsoluteAddress(0x00ff)), 0);
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sta(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x01,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};

        sta(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_stx(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x01,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};

        stx(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sty(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x01,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |address, data| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            },
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{..cpu_initial.clone()};

        sty(&mut cpu_initial, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tax(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x01,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{x_register: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        tax(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tay(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x01,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{y_register: 0x00,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Zero, true);

        tay(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tsx(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{x_register: 0xfd,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        tsx(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_txa(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x80,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let mut cpu_expected = MOS6502{accumulator: 0x80,..cpu_initial.clone()};
        cpu_expected.set_flag(StatusFlag::Negative, true);

        txa(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_txs(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{stack_pointer: 0x00,..cpu_initial.clone()};

        txs(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tya(){
        let mut cpu_initial = MOS6502{
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x01,
            program_counter: 0x0000,
            stack_pointer: 0xfd,
            status_register: 0x00,
            read: |_address| {panic!{"Read function was called"}},
            write: |_address, _data| {panic!{"Write function was called"}},
            remaining_cycles: 0
        };

        let cpu_expected = MOS6502{accumulator: 0x01,..cpu_initial.clone()};

        tya(&mut cpu_initial, AddressModeValue::Implied);

        assert_eq!(cpu_initial, cpu_expected);
    }
}