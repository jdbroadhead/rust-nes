use std::u16;
mod instruction;
mod utils;

/// The 6502 uses two bytes for memory addresses. Not all of it is RAM, cartridge memory is
/// addressed in the same way.
const MEMORY_SIZE: usize = u16::MAX as usize;

struct CPUFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool
}

impl CPUFlags {
    pub fn new() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false
        }
    }
}

struct CPU6502<'a> {
    x: u8,
    y: u8,
    a: u8,
    pc: u16,
    sp: u8,
    flags: CPUFlags,
    memory: &'a mut [u8]
}

impl<'a> CPU6502<'a> {
    pub fn new(memory: &'a mut [u8]) -> Self {
        Self {
            x: 0,
            y: 0,
            a: 0,
            pc: 0,
            sp: 0,
            flags: CPUFlags::new(),
            memory
        }
    }


}

fn main() {
    let mut memory = [0 as u8; MEMORY_SIZE];
    let mut cpu = CPU6502::new(memory.as_mut_slice());
}
