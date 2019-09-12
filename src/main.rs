mod mos6502;

use mos6502::MOS6502;

//TODO: Refactor this into a library
fn main() {
    let mut test: MOS6502 = MOS6502::new(read, write);
}

fn write(address: u16, data: u8){
    println!("Writing {:2X} to {:4X}", data, address);
}

fn read(address: u16) -> u8{
    println!("Reading from {:4X}", address);
    return Default::default();
}

fn decimal_add(x: u8, y:u8) -> u8{
    let mut sum = x.wrapping_add(y);
    if (x & 0x0f) + (y & 0x0f) > 0x09{
        sum = sum.wrapping_add(0x06);
    }
    if (sum & 0xf0) > 0x90{
        sum = sum.wrapping_add(0x60);
    }
    return sum
}

fn decimal_subtract(x: u8, y: u8) -> u8{
    let mut diff = x.wrapping_sub(y);
    if (x & 0x0f) < (y & 0x0f){
        diff = diff.wrapping_sub(0x06);
    }
    if (x & 0xf0) < (y & 0xf0){
        diff = diff.wrapping_sub(0x60);
    }
    return diff
}