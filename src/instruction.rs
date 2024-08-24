use std::fmt::Display;

/// Represents the various addressing modes used by the 6502. A more comprehensive explanation is
/// available at [Emulator 101](http://www.emulator101.com/6502-addressing-modes.html)
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AddressingMode {
    /// The target is the A register
    Accumulator,
    /// The data for the instruction is the next byte after the opcode
    Immediate,
    /// The affected data is definitionally implied by the instruction
    Implied,
    /// The address is the PC plus a signed offset (i8) given by the byte after the instruction
    Relative,
    /// The address is the memory location given next two bytes (little-endian)
    Absolute,
    /// The address is the memory location given next two bytes (little-endian) plus the contents of the X register
    AbsoluteIndexedX,
    /// The address is the memory location given next two bytes (little-endian) plus the contents of the Y register
    AbsoluteIndexedY,
    /// The address is given by the next byte (can only be in the range 0x00 to 0xFF)
    ZeroPage,
    /// The address is given by the next byte plus the contents of the X register (can only be in the range 0x00 to 0xFF, wraps around on overflow)
    ZeroPageIndexedX,
    /// The address is given by the next byte plus the contents of the Y register (can only be in the range 0x00 to 0xFF, wraps around on overflow)
    ZeroPageIndexedY,
    /// The target address is stored at the address location represented by the next two bytes (little-endian)
    Indirect,
    /// The target address is stored at the address location represented by the next two bytes (little-endian) plus the contents of the X register
    IndexedIndirect,
    /// The target address is the value at the address location represented by the next two bytes (little-endian), with the contents of the Y register added to it
    IndirectIndexed
}


#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// [Add with carry](https://www.masswerk.at/6502/6502_instruction_set.html#ADC)
    ADC,
    /// [And with accumulator](https://www.masswerk.at/6502/6502_instruction_set.html#AND)
    AND,
    /// [Arithmetic shift left](https://www.masswerk.at/6502/6502_instruction_set.html#ASL)
    ASL,
    /// [Branch if carry flag clear](https://www.masswerk.at/6502/6502_instruction_set.html#BCC)
    BCC,
    /// [Branch if carry flag set](https://www.masswerk.at/6502/6502_instruction_set.html#BCS)
    BCS,
    /// [Branch if zero flag set](https://www.masswerk.at/6502/6502_instruction_set.html#BEQ)
    BEQ,
    /// [Bit test](https://www.masswerk.at/6502/6502_instruction_set.html#BIT)
    BIT,
    /// [Branch if minus flag set](https://www.masswerk.at/6502/6502_instruction_set.html#BMI)
    BMI,
    /// [Branch if zero flag clear](https://www.masswerk.at/6502/6502_instruction_set.html#BNE)
    BNE,
    /// [Branch if zero flag set](https://www.masswerk.at/6502/6502_instruction_set.html#BPL)
    BPL,
    /// [Force Break (software interrupt)](https://www.masswerk.at/6502/6502_instruction_set.html#BRK)
    BRK,
    /// [Branch if overflow flag clear](https://www.masswerk.at/6502/6502_instruction_set.html#BVC)
    BVC,
    /// [Branch if overflow flag set](https://www.masswerk.at/6502/6502_instruction_set.html#BVS)
    BVS,
    /// [Clear overflow flag](https://www.masswerk.at/6502/6502_instruction_set.html#CLC)
    CLC,
    /// [Clear decimal flag](https://www.masswerk.at/6502/6502_instruction_set.html#CLD)
    CLD,
    /// [Clear interrupt disable flag](https://www.masswerk.at/6502/6502_instruction_set.html#CLI)
    CLI,
    /// [Clear overflow flag](https://www.masswerk.at/6502/6502_instruction_set.html#CLI)
    CLV,
    /// [Compare memory to accumulator](https://www.masswerk.at/6502/6502_instruction_set.html#CMP)
    CMP,
    /// [Compare memory and index X](https://www.masswerk.at/6502/6502_instruction_set.html#CPX)
    CPX,
    /// [Compare memory and index Y](https://www.masswerk.at/6502/6502_instruction_set.html#CPY)
    CPY,
    /// [Decrement memory by 1](https://www.masswerk.at/6502/6502_instruction_set.html#DEC)
    DEC,
    /// [Decrement X by 1](https://www.masswerk.at/6502/6502_instruction_set.html#DEX)
    DEX,
    /// [Decrement Y by 1](https://www.masswerk.at/6502/6502_instruction_set.html#DEY)
    DEY,
    /// [XOR memory with accumulator](https://www.masswerk.at/6502/6502_instruction_set.html#EOR)
    EOR,
    /// [Increment memory by one](https://www.masswerk.at/6502/6502_instruction_set.html#INC)
    INC,
    /// [Increment X by one](https://www.masswerk.at/6502/6502_instruction_set.html#INX)
    INX,
    /// [Increment Y by one](https://www.masswerk.at/6502/6502_instruction_set.html#INY)
    INY,
    /// [Jump to new instruction](https://www.masswerk.at/6502/6502_instruction_set.html#JMP)
    JMP,
    /// [Jump to new instruction saving return address](https://www.masswerk.at/6502/6502_instruction_set.html#JSR)
    JSR,
    /// [Load accumulator with memory](https://www.masswerk.at/6502/6502_instruction_set.html#LDA)
    LDA,
    /// [Load X with memory](https://www.masswerk.at/6502/6502_instruction_set.html#LDX)
    LDX,
    /// [Load Y with memory](https://www.masswerk.at/6502/6502_instruction_set.html#LDY)
    LDY,
    /// [Shift one bit right](https://www.masswerk.at/6502/6502_instruction_set.html#LSR)
    LSR,
    /// [No operation](https://www.masswerk.at/6502/6502_instruction_set.html#NOP)
    NOP,
    /// [OR memory with accumulator](https://www.masswerk.at/6502/6502_instruction_set.html#ORA)
    ORA,
    /// [Push accumulator onto stack](https://www.masswerk.at/6502/6502_instruction_set.html#PHA)
    PHA,
    /// [Push processor status onto stack](https://www.masswerk.at/6502/6502_instruction_set.html#PHP)
    PHP,
    /// [Pull accumulator from stack](https://www.masswerk.at/6502/6502_instruction_set.html#PLA)
    PLA,
    /// [Pull processor status from stack](https://www.masswerk.at/6502/6502_instruction_set.html#PLP)
    PLP,
    /// [Rotate one bit left](https://www.masswerk.at/6502/6502_instruction_set.html#ROL)
    ROL,
    /// [Rotate one bit right](https://www.masswerk.at/6502/6502_instruction_set.html#ROR)
    ROR,
    /// [Return from interrupt](https://www.masswerk.at/6502/6502_instruction_set.html#RTI)
    RTI,
    /// [Return from subroutine](https://www.masswerk.at/6502/6502_instruction_set.html#RTS)
    RTS,
    /// [Subtract memory from accumulator with borrow](https://www.masswerk.at/6502/6502_instruction_set.html#SBC)
    SBC,
    /// [Set carry flag](https://www.masswerk.at/6502/6502_instruction_set.html#SEC)
    SEC,
    /// [Set decimal flag](https://www.masswerk.at/6502/6502_instruction_set.html#SDC)
    SED,
    /// [Set interrupt disable status flag](https://www.masswerk.at/6502/6502_instruction_set.html#SEI)
    SEI,
    /// [Store accumulator in memory](https://www.masswerk.at/6502/6502_instruction_set.html#STA)
    STA,
    /// [Store X in memory](https://www.masswerk.at/6502/6502_instruction_set.html#STX)
    STX,
    /// [Store Y in memory](https://www.masswerk.at/6502/6502_instruction_set.html#STY)
    STY,
    /// [Transfer accumulator to X](https://www.masswerk.at/6502/6502_instruction_set.html#TAX)
    TAX,
    /// [Transfer accumulator to Y](https://www.masswerk.at/6502/6502_instruction_set.html#TAX)
    TAY,
    /// [Transfer stack pointer to X](https://www.masswerk.at/6502/6502_instruction_set.html#TSX)
    TSX,
    /// [Transfer X to accumulator](https://www.masswerk.at/6502/6502_instruction_set.html#TXA)
    TXA,
    /// [Transfer X to stack pointer](https://www.masswerk.at/6502/6502_instruction_set.html#TXS)
    TXS,
    /// [Transfer Y to accumulator](https://www.masswerk.at/6502/6502_instruction_set.html#TYA)
    TYA    
}


pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub cycles: usize,
    pub data: (u8, u8),
    pub width: usize,
    pub opcode_byte: u8
}

impl Instruction {
    #[inline]
    pub fn decode(memory: &[u8], memory_position: u16) -> Instruction {
        let index = memory_position as usize;
        let opcode_byte = memory[index];
        // We always pass the next two bytes as data as it simplifies construction logic
        let data = (memory[index+1], memory[index+2]);
        // Cases are in alphabetical order of opcode for readability
        match opcode_byte {

            // ADC
            0x69 => Self {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::Immediate,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },
            0x65 => Self {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::ZeroPage,
                cycles: 3,
                width: 2,
                data,
				opcode_byte
            },
            0x75 => Self {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                cycles: 4,
                width: 2,
                data,
				opcode_byte
            },
            0x6D => Self {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::Absolute,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0x7D => Self {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0x79 => Self {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0x61 => Self {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::IndexedIndirect,
                cycles: 6,
                width: 2,
                data,
				opcode_byte
            },
            0x71 => Self {
                opcode: Opcode::ADC,
                addressing_mode: AddressingMode::IndirectIndexed,
                cycles: 5,
                width: 2,
                data,
				opcode_byte
            },

            // AND
            0x29 => Self {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::Immediate,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },
            0x25 => Self {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::ZeroPage,
                cycles: 3,
                width: 2,
                data,
				opcode_byte
            },
            0x35 => Self {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                cycles: 4,
                width: 2,
                data,
				opcode_byte
            },
            0x2D => Self {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::Absolute,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0x3D => Self {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0x39 => Self {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0x21 => Self {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::IndexedIndirect,
                cycles: 6,
                width: 2,
                data,
				opcode_byte
            },
            0x31 => Self {
                opcode: Opcode::AND,
                addressing_mode: AddressingMode::IndirectIndexed,
                cycles: 5,
                width: 2,
                data,
				opcode_byte
            },

            // ASL
            0x0A => Self {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::Accumulator,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },
            0x06 => Self {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },
            0x16 => Self {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0x0E => Self {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 6,
                data,
				opcode_byte
            },
            0x1E => Self {
                opcode: Opcode::ASL,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 7,
                data,
				opcode_byte
            },

            // BCC
            0x90 => Self {
                opcode: Opcode::BCC,
                addressing_mode: AddressingMode::Relative,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },

            // BCS
            0xB0 => Self {
                opcode: Opcode::BCS,
                addressing_mode: AddressingMode::Relative,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },

            // BEQ
            0xF0 => Self {
                opcode: Opcode::BEQ,
                addressing_mode: AddressingMode::Relative,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },

            // BIT
            0x24 => Self {
                opcode: Opcode::BIT,
                addressing_mode: AddressingMode::ZeroPage,
                cycles: 3,
                width: 2,
                data,
				opcode_byte
            },
            0x2C => Self {
                opcode: Opcode::BIT,
                addressing_mode: AddressingMode::Absolute,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },

            // BMI
            0x30 => Self {
                opcode: Opcode::BMI,
                addressing_mode: AddressingMode::Relative,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },

            // BNE
            0xD0 => Self {
                opcode: Opcode::BNE,
                addressing_mode: AddressingMode::Relative,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },

            // BPL
            0x10 => Self {
                opcode: Opcode::BPL,
                addressing_mode: AddressingMode::Relative,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },

            // BRK
            0x00 => Self {
                opcode: Opcode::BRK,
                addressing_mode: AddressingMode::Implied,
                cycles: 7,
                width: 1,
                data,
				opcode_byte
            },

            // BVC
            0x50 => Self {
                opcode: Opcode::BVC,
                addressing_mode: AddressingMode::Relative,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },

            // BVS
            0x70 => Self {
                opcode: Opcode::BVS,
                addressing_mode: AddressingMode::Relative,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },

            // CLC
            0x18 => Self {
                opcode: Opcode::CLC,
                addressing_mode: AddressingMode::Implied,
                cycles: 2,
                width: 1,
                data,
				opcode_byte
            },

            // CLD
            0xD8 => Self {
                opcode: Opcode::CLD,
                addressing_mode: AddressingMode::Implied,
                cycles: 2,
                width: 1,
                data,
				opcode_byte
            },

            // CLI
            0x58 => Self {
                opcode: Opcode::CLI,
                addressing_mode: AddressingMode::Implied,
                cycles: 2,
                width: 1,
                data,
				opcode_byte
            },

            // CLV
            0xB8 => Self {
                opcode: Opcode::CLV,
                addressing_mode: AddressingMode::Implied,
                cycles: 2,
                width: 1,
                data,
				opcode_byte
            },

            // CMP
            0xC9 => Self {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::Immediate,
                cycles: 2,
                width: 2,
                data,
				opcode_byte
            },
            0xC5 => Self {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::ZeroPage,
                cycles: 3,
                width: 2,
                data,
				opcode_byte
            },
            0xD5 => Self {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                cycles: 4,
                width: 2,
                data,
				opcode_byte
            },
            0xCD => Self {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::Absolute,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0xDD => Self {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0xD9 => Self {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                cycles: 4,
                width: 3,
                data,
				opcode_byte
            },
            0xC1 => Self {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::IndexedIndirect,
                cycles: 6,
                width: 2,
                data,
				opcode_byte
            },
            0xD1 => Self {
                opcode: Opcode::CMP,
                addressing_mode: AddressingMode::IndirectIndexed,
                cycles: 5,
                width: 2,
                data,
				opcode_byte
            },

            // CPX
            0xE0 => Self {
                opcode: Opcode::CPX,
                addressing_mode: AddressingMode::Immediate,
                width: 2,
                cycles: 2,
                data,
				opcode_byte
            },
            0xE4 => Self {
                opcode: Opcode::CPX,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0xEC => Self {
                opcode: Opcode::CPX,
                addressing_mode: AddressingMode::Absolute,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },

            // CPY
            0xC0 => Self {
                opcode: Opcode::CPY,
                addressing_mode: AddressingMode::Immediate,
                width: 2,
                cycles: 2,
                data,
				opcode_byte
            },
            0xC4 => Self {
                opcode: Opcode::CPY,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0xCC => Self {
                opcode: Opcode::CPY,
                addressing_mode: AddressingMode::Absolute,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },

            // DEC
            0xC6 => Self {
                opcode: Opcode::DEC,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },
            0xD6 => Self {
                opcode: Opcode::DEC,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0xCE => Self {
                opcode: Opcode::DEC,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 6,
                data,
				opcode_byte
            },
            0xDE => Self {
                opcode: Opcode::DEC,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 7,
                data,
				opcode_byte
            },

            // DEX
            0xCA => Self {
                opcode: Opcode::DEX,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // DEY
            0x88 => Self {
                opcode: Opcode::DEY,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // EOR
            0x49 => Self {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::Immediate,
                width: 2,
                cycles: 2,
                data,
				opcode_byte
            },
            0x45 => Self {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0x55 => Self {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0x4D => Self {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0x5D => Self {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0x59 => Self {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0x41 => Self {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::IndexedIndirect,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0x51 => Self {
                opcode: Opcode::EOR,
                addressing_mode: AddressingMode::IndirectIndexed,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },

            // INC
            0xE6 => Self {
                opcode: Opcode::INC,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },
            0xF6 => Self {
                opcode: Opcode::INC,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0xEE => Self {
                opcode: Opcode::INC,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 6,
                data,
				opcode_byte
            },
            0xFE => Self {
                opcode: Opcode::INC,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 7,
                data,
				opcode_byte
            },

            // INX
            0xE8 => Self {
                opcode: Opcode::INX,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // INY
            0xC8 => Self {
                opcode: Opcode::INY,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // JMP
            0x4C => Self {
                opcode: Opcode::JMP,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 3,
                data,
				opcode_byte
            },
            0x6C => Self {
                opcode: Opcode::JMP,
                addressing_mode: AddressingMode::Indirect,
                width: 3,
                cycles: 5,
                data,
				opcode_byte
            },

            // JSR
            0x20 => Self {
                opcode: Opcode::JSR,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 6,
                data,
				opcode_byte
            },

            // LDA
            0xA9 => Self {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::Immediate,
                width: 2,
                cycles: 2,
                data,
				opcode_byte
            },
            0xA5 => Self {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0xB5 => Self {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0xAD => Self {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0xBD => Self {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0xB9 => Self {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0xA1 => Self {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::IndexedIndirect,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0xB1 => Self {
                opcode: Opcode::LDA,
                addressing_mode: AddressingMode::IndirectIndexed,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },

            // LDX
            0xA2 => Self {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::Immediate,
                width: 2,
                cycles: 2,
                data,
				opcode_byte
            },
            0xA6 => Self {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0xB6 => Self {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::ZeroPageIndexedY,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0xAE => Self {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0xBE => Self {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },

             // LDY
             0xA0 => Self {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::Immediate,
                width: 2,
                cycles: 2,
                data,
				opcode_byte
            },
            0xA4 => Self {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0xB4 => Self {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0xAC => Self {
                opcode: Opcode::LDY,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0xBC => Self {
                opcode: Opcode::LDX,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            
            // LSR
            0x4A => Self {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::Accumulator,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },
            0x46 => Self {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },
            0x56 => Self {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0x4E => Self {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 6,
                data,
				opcode_byte
            },
            0x5E => Self {
                opcode: Opcode::LSR,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 7,
                data,
				opcode_byte
            },

            // NOP
            0xEA => Self {
                opcode: Opcode::NOP,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // ORA
            0x09 => Self {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::Immediate,
                width: 2,
                cycles: 2,
                data,
				opcode_byte
            },
            0x05 => Self {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0x15 => Self {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0x0D => Self {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0x1D => Self {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0x19 => Self {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0x01 => Self {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::IndexedIndirect,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0x11 => Self {
                opcode: Opcode::ORA,
                addressing_mode: AddressingMode::IndirectIndexed,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },

            // PHA
            0x48 => Self {
                opcode: Opcode::PHA,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 3,
                data,
				opcode_byte
            },

            // PHP
            0x08 => Self {
                opcode: Opcode::PHP,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 3,
                data,
				opcode_byte
            },

            // PLA
            0x68 => Self {
                opcode: Opcode::PLA,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 4,
                data,
				opcode_byte
            },

            // PLP
            0x28 => Self {
                opcode: Opcode::PLP,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 4,
                data,
				opcode_byte
            },

            // ROL
            0x2A => Self {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::Accumulator,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },
            0x26 => Self {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },
            0x36 => Self {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0x2E => Self {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 6,
                data,
				opcode_byte
            },
            0x3E => Self {
                opcode: Opcode::ROL,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 7,
                data,
				opcode_byte
            },
            
            // ROR
            0x6A => Self {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::Accumulator,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },
            0x66 => Self {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },
            0x76 => Self {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0x6E => Self {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 6,
                data,
				opcode_byte
            },
            0x7E => Self {
                opcode: Opcode::ROR,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 7,
                data,
				opcode_byte
            },

            // RTI
            0x40 => Self {
                opcode: Opcode::RTI,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 6,
                data,
				opcode_byte
            },

            // RTS
            0x60 => Self {
                opcode: Opcode::RTS,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 6,
                data,
				opcode_byte
            },

            // SBC
            0xE9 => Self {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::Immediate,
                width: 2,
                cycles: 2,
                data,
				opcode_byte
            },
            0xE5 => Self {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0xF5 => Self {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0xED => Self {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::Absolute,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0xFD => Self {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0xF9 => Self {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0xE1 => Self {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::IndexedIndirect,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0xF1 => Self {
                opcode: Opcode::SBC,
                addressing_mode: AddressingMode::IndirectIndexed,
                width: 2,
                cycles: 5,
                data,
				opcode_byte
            },
            
            // SEC
            0x38 => Self {
                opcode: Opcode::SEC,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // SED
            0xF8 => Self {
                opcode: Opcode::SED,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // SEI
            0x78 => Self {
                opcode: Opcode::SEI,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // STA
            0x85 => Self {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0x95 => Self {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0x8D => Self {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },
            0x9D => Self {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::AbsoluteIndexedX,
                width: 3,
                cycles: 5,
                data,
				opcode_byte
            },
            0x99 => Self {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::AbsoluteIndexedY,
                width: 3,
                cycles: 5,
                data,
				opcode_byte
            },
            0x81 => Self {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::IndexedIndirect,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },
            0x91 => Self {
                opcode: Opcode::STA,
                addressing_mode: AddressingMode::IndirectIndexed,
                width: 2,
                cycles: 6,
                data,
				opcode_byte
            },

            // STX
            0x86 => Self {
                opcode: Opcode::STX,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0x96 => Self {
                opcode: Opcode::STX,
                addressing_mode: AddressingMode::ZeroPageIndexedY,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0x8E => Self {
                opcode: Opcode::STX,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },

            // STY
            0x84 => Self {
                opcode: Opcode::STY,
                addressing_mode: AddressingMode::ZeroPage,
                width: 2,
                cycles: 3,
                data,
				opcode_byte
            },
            0x94 => Self {
                opcode: Opcode::STY,
                addressing_mode: AddressingMode::ZeroPageIndexedX,
                width: 2,
                cycles: 4,
                data,
				opcode_byte
            },
            0x8C => Self {
                opcode: Opcode::STY,
                addressing_mode: AddressingMode::Absolute,
                width: 3,
                cycles: 4,
                data,
				opcode_byte
            },

            // TAX
            0xAA => Self {
                opcode: Opcode::TAX,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // TAY
            0xA8 => Self {
                opcode: Opcode::TAY,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // TSX
            0xBA => Self {
                opcode: Opcode::TSX,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // TXA
            0x8A => Self {
                opcode: Opcode::TXA,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // TXS
            0x9A => Self {
                opcode: Opcode::TXS,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            // TYA
            0x98 => Self {
                opcode: Opcode::TYA,
                addressing_mode: AddressingMode::Implied,
                width: 1,
                cycles: 2,
                data,
				opcode_byte
            },

            _ => panic!("Unsupported instruction decoded {}!", opcode_byte)
        }
    }
}