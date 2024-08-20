/// Represents the various addressing modes used by the 6502. A more comprehensive explanation is
/// available at [Emulator 101](http://www.emulator101.com/6502-addressing-modes.html)
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
    ZeroPageIndexed,
    /// The target address is stored at the address location represented by the next two bytes (little-endian)
    Indirect,
    /// The target address is stored at the address location represented by the next two bytes (little-endian) plus the contents of the X register
    IndexedIndirect,
    /// The target address is the value at the address location represented by the next two bytes (little-endian), with the contents of the Y register added to it
    IndirectIndexed
}

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
