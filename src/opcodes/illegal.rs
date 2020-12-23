//! ### ILLEGAL OPCODES
//! This module contains all functions for the illegal opcodes that are not defined by official sources
use super::*;
use crate::address_modes::AddressModeValue;
use crate::MOS6502;

/// SLO: Combines the ASl and ORA opcodes
pub(super) fn slo(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        asl(cpu, bus, address_mode_value);
        ora(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode SLO called, ignoring");
    }
}

/// RLA: Combines the ROL and AND opcodes
pub(super) fn rla(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        rol(cpu, bus, address_mode_value);
        and(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode RLA called, ignoring");
    }
}

/// SRE: Combines the LSR and EOR opcodes
pub(super) fn sre(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        lsr(cpu, bus, address_mode_value);
        eor(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode SRE called, ignoring");
    }
}

/// RRA: Combines the ROR and ADC opcodes
pub(super) fn rra(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        ror(cpu, bus, address_mode_value);
        adc(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode RRA called, ignoring");
    }
}

/// SAX: Sets the accumulator to the result of a logical and performed with the accumulator and x register
pub(super) fn sax(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
            bus.write(address, cpu.accumulator & cpu.x_register);
        } else {
            panic!("SAX opcode called with invalid address mode!")
        }
    } else {
        warn!("Illegal opcode SAX called, ignoring");
    }
}

/// LAX: Combines the LDA and LDX opcodes, loading the addressed value into both registers
pub(super) fn lax(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        lda(cpu, bus, address_mode_value);
        ldx(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode LAX called, ignoring");
    }
}

/// DCP: Combines the DEC and CMP opcodes, decrementing the addressed value and comparing it to the accumulator
pub(super) fn dcp(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        dec(cpu, bus, address_mode_value);
        cmp(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode DCP called, ignoring");
    }
}

/// ISC: Combines the INC and SBC opcodes, incrementing the addressed value and then subtracting it from the accumulator
pub(super) fn isc(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        inc(cpu, bus, address_mode_value);
        sbc(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode ISC called, ignoring");
    }
}
/// ANC: Performs a logical AND on the accumulator with the immediate value and sets the carry flag based on bit 7 like ASL
pub(super) fn anc(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        and(cpu, bus, address_mode_value);
        cpu.set_flag(StatusFlag::Carry, cpu.accumulator >> 7 == 1);
    } else {
        warn!("Illegal opcode ANC called, ignoring");
    }
}

/// ALR: Combines the AND (immediate) and LSR opcodes, shifting the accumulator right after the AND is performed
pub(super) fn alr(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        and(cpu, bus, address_mode_value);
        lsr(cpu, bus, AddressModeValue::Implied);
    } else {
        warn!("Illegal opcode ALR called, ignoring");
    }
}

/// ARR: Combines the AND (immediate) and ROR opcodes, rotating the accumulator right after the AND is performed
///
/// NOTE: This can have some unexpected effects on flags
// TODO: Verify this behaviour with a more reputable source
pub(super) fn arr(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        and(cpu, bus, address_mode_value);
        //Some flags are set based on ADC
        if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
            let value = bus.read(address);
            let result = cpu.accumulator.wrapping_add(value);
            cpu.set_flag(
                StatusFlag::Overflow,
                (!(cpu.accumulator ^ value) & (cpu.accumulator ^ result) & StatusFlag::Negative as u8) > 0,
            );
        } else {
            panic!("ARR opcode called with invalid address mode!")
        }
        cpu.accumulator &= !1u8; // The state of bit 0 is lost instead of being placed in the carry bit
        ror(cpu, bus, AddressModeValue::Implied);
    } else {
        warn!("Illegal opcode ARR called, ignoring");
    }
}

/// XAA: Combines the TXA and AND opcodes, copying the x register into the accumulator and then ANDing it with the addressed value
pub(super) fn xaa(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        txa(cpu, bus, AddressModeValue::Implied);
        and(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode XAA called, ignoring");
    }
}

/// AXS: Sets the x register to the result of the x register AND the accumulator minus the immediate value
pub(super) fn axs(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        let value = cpu.accumulator & cpu.x_register;
        cpu.x_register = compare(cpu, bus, value, address_mode_value);
    } else {
        warn!("Illegal opcode AXS called, ignoring");
    }
}

/// AHX: Sets the addressed value to the high byte of the address AND A AND X
pub(super) fn ahx(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
            bus.write(address, cpu.accumulator & cpu.x_register & (address >> 8) as u8);
        } else {
            panic!("AHX opcode called with invalid address mode!")
        }
    } else {
        warn!("Illegal opcode AHX called, ignoring");
    }
}

/// SHY: Sets the addressed value to the high byte of the address AND Y
pub(super) fn shy(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
            bus.write(address, cpu.y_register & (address >> 8) as u8);
        } else {
            panic!("SHY opcode called with invalid address mode!")
        }
    } else {
        warn!("Illegal opcode SHY called, ignoring");
    }
}

/// SHX: Sets the addressed value to the high byte of the address AND X
pub(super) fn shx(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
            bus.write(address, cpu.x_register & (address >> 8) as u8);
        } else {
            panic!("SHX opcode called with invalid address mode!")
        }
    } else {
        warn!("Illegal opcode SHX called, ignoring");
    }
}

/// TAS: Sets the stack pointer to the accumulator AND the x register and then mimics AHX
pub(super) fn tas(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        cpu.stack_pointer = cpu.accumulator & cpu.x_register;
        ahx(cpu, bus, address_mode_value);
    } else {
        warn!("Illegal opcode TAS called, ignoring");
    }
}

/// LAS: Sets the stack pointer, x register, and accumulator to the addressed value AND the stack pointer
pub(super) fn las(cpu: &mut MOS6502, bus: &mut dyn Interface6502, address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        if let AddressModeValue::AbsoluteAddress(address) = address_mode_value {
            let value = bus.read(address) & cpu.stack_pointer;
            cpu.accumulator = value;
            cpu.x_register = value;
            cpu.stack_pointer = value;
            cpu.set_flag(StatusFlag::Negative, value & StatusFlag::Negative as u8 > 0);
            cpu.set_flag(StatusFlag::Zero, value == 0);
        } else {
            panic!("LAS opcode called with invalid address mode!")
        }
    } else {
        warn!("Illegal opcode LAS called, ignoring");
    }
}

/// KIL: Halts the CPU, calling this function will just call a panic!
pub(super) fn kil(_cpu: &mut MOS6502, _bus: &mut dyn Interface6502, _address_mode_value: AddressModeValue) {
    if cfg!(feature = "illegal_opcodes") {
        error!("KIL opcode called!, crashing the emulator...");
        panic!("KIL opcode called!");
    } else {
        warn!("Illegal opcode KIL called, ignoring");
    }
}

#[cfg(all(test, feature = "illegal_opcodes"))]
mod test {
    #![allow(unused_variables, unused_mut)] // Allow some warnings for test code

    use super::*;
    use crate::address_modes::AddressModeValue;
    use crate::test_utilities::StubInterface6502;
    use crate::MOS6502;

    #[test]
    fn test_slo() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x01,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x4f,
                (0x00ff, 2) => 0x4f << 1,
                _ => panic!("Unintended Address Accessed: 0x{:X}, read_count: {}", address, read_count),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x4f << 1);
            },
            ..Default::default()
        };
        let mut cpu_expected = MOS6502 {
            accumulator: 0x9f,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        slo(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_slo_zero_carry_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x80,
                (0x00ff, 2) => 0x80 << 1,
                _ => panic!("Unintended Address Accessed: 0x{:X}, read_count: {}", address, read_count),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x80 << 1);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, true);
        cpu_expected.set_flag(StatusFlag::Zero, true);

        slo(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rla() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x06,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x41,
                (0x00ff, 2) => 0x83,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x83);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x02,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);

        rla(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rla_zero_carry_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x08,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0xd0,
                (0x00ff, 2) => 0xd0 << 1,
                _ => panic!("Unintended Address Accessed: 0x{:X}, read_count: {}", address, read_count),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0xd0 << 1);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, true);
        cpu_expected.set_flag(StatusFlag::Zero, true);

        rla(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sre() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x80,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x80,
                (0x00ff, 2) => 0x80 >> 1,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x80 >> 1);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0xc0,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        sre(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sre_zero_carry_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x7f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0xff,
                (0x00ff, 2) => 0xff >> 1,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0xff >> 1);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        sre(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rra() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x09,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x20,
                (0x00ff, 2) => 0x90,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x90);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x99,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        rra(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rra_zero_carry_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x02,
                (0x00ff, 2) => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        rra(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_rra_overflow_negative_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x7f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x02,
                (0x00ff, 2) => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x80,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Overflow, true);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        rra(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_rra_decimal_mode() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x09,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x12,
                (0x00ff, 2) => 0x09,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x09);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x18,
            ..cpu_initial
        };

        rra(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_rra_decimal_mode_zero_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x98,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x03,
                (0x00ff, 2) => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        rra(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_rra_decimal_mode_overflow_negative_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x75,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x0c,
                (0x00ff, 2) => 0x06,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x06);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x81,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        rra(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_sax() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x81,
            x_register: 0x41,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| {
                panic! {"Read function was called"}
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x01);
            },
            ..Default::default()
        };

        let cpu_expected = MOS6502 { ..cpu_initial };

        sax(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lax() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0xff,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0xff,
            x_register: 0xff,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        lax(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_lax_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            x_register: 0xab,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| 0x00,
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        lax(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dcp() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x00,
                (0x00ff, 2) => 0xff,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0xff);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        dcp(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_dcp_less() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x0f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x11,
                (0x00ff, 2) => 0x10,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x10);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        dcp(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_isc() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x07,
                (0x00ff, 2) => 0x08,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x08);
            },
            ..Default::default()
        };

        let cpu_expected = MOS6502 {
            accumulator: 0x08,
            ..cpu_initial
        };

        isc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_isc_overflow() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x81,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x01,
                (0x00ff, 2) => 0x02,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x02);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x7f,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        isc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_isc_zero() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x0f,
                (0x00ff, 2) => 0x10,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x10);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        isc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_isc_carry_negative() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x10,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x10,
                (0x00ff, 2) => 0x11,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x11);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0xff,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        isc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_isc_decimal() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x12,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Decimal, true);
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x05,
                (0x00ff, 2) => 0x06,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x06);
            },
            ..Default::default()
        };

        let cpu_expected = MOS6502 {
            accumulator: 0x06,
            ..cpu_initial
        };

        isc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    #[cfg(feature = "binary_coded_decimal")]
    fn test_isc_decimal_carry_negative() {
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
            read: |address, read_count| match (address, read_count) {
                (0x00ff, 1) => 0x17,
                (0x00ff, 2) => 0x18,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(address, 0x00ff);
                assert_eq!(data, 0x18);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x94,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        isc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_anc() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x95,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x80,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x80,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, true);
        cpu_expected.set_flag(StatusFlag::Negative, true);

        anc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_anc_zero_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xf0,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        anc(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_alr() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x95,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, true);
        cpu_expected.set_flag(StatusFlag::Zero, true);

        alr(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_alr_negative_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xf0,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Negative, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0xf0,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x78,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Negative, false);

        alr(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_arr() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x95,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x01,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        arr(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_arr_negative_overflow_flags() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x70,
            ..Default::default()
        };
        cpu_initial.set_flag(StatusFlag::Carry, true);

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x70,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0xb8,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Carry, false);
        cpu_expected.set_flag(StatusFlag::Negative, true);
        cpu_expected.set_flag(StatusFlag::Overflow, true);

        arr(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_xaa() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x01,
            x_register: 0x95,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x80,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x80,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        xaa(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_xaa_zero_flag() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x00,
            x_register: 0xf0,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        xaa(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));

        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_axs() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            x_register: 0x0f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);
        cpu_expected.set_flag(StatusFlag::Carry, true);

        axs(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_axs_less() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            x_register: 0x0f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x00ff => 0x10,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            x_register: 0xff,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        axs(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x00ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_ahx() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xff,
            x_register: 0x0f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x01ff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(0x01ff, address);
                assert_eq!(0x01, data);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial };

        ahx(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x01ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_shx() {
        let mut cpu_initial = MOS6502 {
            x_register: 0x0f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x0fff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(0x01ff, address);
                assert_eq!(0x01, data);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial };

        shx(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x01ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_shy() {
        let mut cpu_initial = MOS6502 {
            y_register: 0x03,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x05ff => 0x09,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(0x05ff, address);
                assert_eq!(0x01, data);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 { ..cpu_initial };

        shy(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x5ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_tas() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0xf1,
            x_register: 0x1f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x01ff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                assert_eq!(0x01ff, address);
                assert_eq!(0x01, data);
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            stack_pointer: 0x11,
            ..cpu_initial
        };

        tas(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x01ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_las() {
        let mut cpu_initial = MOS6502 {
            accumulator: 0x01,
            x_register: 0x29,
            stack_pointer: 0xf0,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x01ff => 0x0f,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x00,
            x_register: 0x00,
            stack_pointer: 0x00,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Zero, true);

        las(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x01ff));
        assert_eq!(cpu_initial, cpu_expected);
    }

    #[test]
    fn test_las_negative() {
        let mut cpu_initial = MOS6502 {
            stack_pointer: 0x8f,
            ..Default::default()
        };

        let mut stub_bus = StubInterface6502 {
            read: |address, read_count| match address {
                0x01ff => 0xf0,
                _ => panic!("Unintended Address Accessed: 0x{:X}", address),
            },
            write: |address, data, write_count| {
                panic! {"Write function was called"}
            },
            ..Default::default()
        };

        let mut cpu_expected = MOS6502 {
            accumulator: 0x80,
            x_register: 0x80,
            stack_pointer: 0x80,
            ..cpu_initial
        };
        cpu_expected.set_flag(StatusFlag::Negative, true);

        las(&mut cpu_initial, &mut stub_bus, AddressModeValue::AbsoluteAddress(0x01ff));
        assert_eq!(cpu_initial, cpu_expected);
    }
}
