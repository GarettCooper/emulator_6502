# emulator_6502

[![Build Status](https://travis-ci.com/GarettCooper/6502-emulator.svg)](https://travis-ci.com/GarettCooper/6502-emulator)
[![Crate](https://img.shields.io/crates/v/emulator_6502.svg)](https://crates.io/crates/emulator_6502)
[![Documentation](https://docs.rs/emulator_6502/badge.svg)](https://docs.rs/emulator_6502)

Hello friends, prospective employers, and people who Googled "6502 emulator rust", you've found a small personal project I've been working on since early September of 2019 to use as a talking point during the interview process for my Winter 2020 co-op placement. The goal of the project is to demonstrate my ability to pick up a new programming language while developing a complex system. 

This is a general purpose Rust implementation of an [MOS 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502) emulator, capable of executing code in isolation or as part of one of the many systems the 6502 was used in, including the Commodore 64, Apple II, and Nintendo Entertainment System. To do so, the library provides the Interface6502 trait which allows the client to implement its own functions for reading and writing to memory addresses.

### Defining an interface

```rust

struct BasicRam{
    ram: Box<[u8; u16::max_value() as usize + 1]> //The maximum address range of the 6502
}

impl BasicRam {
    fn load_program(&mut self, start: usize, data: &mut Vec<u8>){
        self.ram[start..].clone_from_slice(data);
    }
}

impl Interface6502 for BasicRam{
    fn read(&mut self, address: u16) -> u8{
        self.ram[address as usize]
    }

    fn write(&mut self, address: u16, data: u8){
        self.ram[address as usize] = data
    }
}

```

In this example, the interface to be used with the emulator simply maps addresses to ram locations. The client is responsible for loading the 6502 binary program it wishes to run into an appropriate part of the address range. A more complex interface could map specific addresses to other emulated device components.

For example, a NES implementation using this 6502 emulator would map reads and writes to addresses 0x2000-0x2007 to communication with the NES' picture processing unit, while a Commodore 64 implementation would map addresses 0xd000-0xd3ff for drawing to the screen.

### Running a program

```rust

fn main() -> Result<()>{
  let mut ram = BasicRam{ ram: Box::new([0; u16::max_value() as usize + 1]) };
  
  //Load a program into memory...
  let mut file = File::open("C:/some_6502_program.bin")?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;
  
  //Copy it into the BasicRam
  ram.load_program(0x0400, &mut buffer);
  
  let mut cpu = MOS6502::new(); //Create a new emulator instance
  cpu.set_program_counter(0x0400); //Set the program counter to the first byte of the program in memory
  cpu.cycle(&mut ram); // The emulator can execute cycles individually, for systems that require precise timing...
  cpu.execute_instruction(&mut ram); // or instruction by instruction for a coarser approach
  
  Ok(())
}

```
Each cycle/instruction the processor borrows mutable ownership of the interface in order to read and write to it.

NOTE: When an instruction is executed, the entire computation is carried out simultaneously before the processor simply waits for the
remaining number of cycles, meaning that timing of reads and writes is only accurate on an instruction-by-instruction basis, not cycle-by-cycle

### Supported Features:
* Full implementation of documented instruction set
* Emulation of bugs that existed in the original 6502 hardware
* Binary Coded Decimal when the "binary_coded_decimal" compilation feature is enabled
* ... hopefully more if I get around to it

Currently undocumented instructions are implemented as placeholder functions which cause the emulator to panic when called.
