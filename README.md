# 6502-emulator
Hello friends, prospective employers, and people who Googled "6502 emulator rust", you've found a small personal project I've been working on since early September of 2019 to use as a talking point during the interview process for my Winter 2020 co-op placement.

This is a general purpose Rust implementation of an [MOS 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502) emulator, capable of executing code in isolation or as part of one of the many systems the 6502 was used in, including the Commodore 64, Apple II, and Nintendo Entertainment System. To do so, the library provides the Interface6502 trait which allows the client to implement its own functions for reading and writing to memory addresses.

*The following samples use the current version of the emulator's public interface, which is still subject to change over the immediate future.*

### Defining an interface

```rust

struct BasicBus{
    ram: Box<[u8; u16::max_value() as usize + 1]> //The maximum address range of the 6502
}

impl BasicRam {
    // Functions for loading a program into the ram go here
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

In this example, the interface to be used with the emulator simply maps addresses to ram locations. The client is responsible for loading the 6502 binary program it wishes to run into an appropriate part of the address range.

### Running a program

```rust

fn main(){
  let mut ram = BasicRam{ ram: Box::new([0; u16::max_value() as usize + 1]) };
  //Load a program into memory...
  let mut cpu = MOS6502::new(); //Create a new emulator instance
  cpu.set_program_counter(0x0400); //Set the program counter to the first byte of the program in memory
  cpu.cycle(&mut ram); // The emulator can execute cycles individually, for systems that require precise timing...
  cpu.execute_instruction(&mut ram); // or instruction by instruction for a coarser approach
}

```
Each cycle/instruction the processor borrows mutable ownership of the interface in order to read and write to it. 

NOTE: When an instruction is executed, the entire computation is carried out simultaneously before the processor simply waits for the
remaining number of cycles, meaning that timing of reads and writes is only accurate on an instruction-by-instruction basis, not cycle-by-cycle

### Supported Features:
* Binary Coded Decimal when the "binary_coded_decimal" compilation feature is enabled
* ... hopefully more if I get around to it

Currently undocumented instructions are implemented as placeholder functions which cause the emulator to panic when called.
