//! Module of test utility types

use super::*;

pub(crate) struct StubInterface6502 {
    pub(crate) read: fn(u16, u8) -> u8,
    pub(crate) read_count: u8,
    pub(crate) write: fn(u16, u8, u8),
    pub(crate) write_count: u8,
}

impl StubInterface6502 {
    pub(crate) fn new(read_fn: fn(u16, u8) -> u8, write_fn: fn(u16, u8, u8)) -> Self {
        StubInterface6502 {
            read: read_fn,
            write: write_fn,
            read_count: 0,
            write_count: 0,
        }
    }
}

impl Interface6502 for StubInterface6502 {
    fn read(&mut self, address: u16) -> u8 {
        self.read_count += 1;
        (self.read)(address, self.read_count)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.write_count += 1;
        (self.write)(address, data, self.read_count)
    }
}

impl Default for StubInterface6502 {
    fn default() -> Self {
        StubInterface6502::new(
            |_address, _read_count| panic!("Read Function was not initialized"),
            |_address, _data, _write_count| panic!("Write Function was not initialized"),
        )
    }
}
