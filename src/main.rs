mod instruction;
mod utils;
use core::panic;
use std::{collections::btree_map, fmt::{write, Display}, u16};

use instruction::{AddressingMode, Instruction, Opcode};
use utils::{is_negative, is_zero, to_address_from_bytes, was_page_boundary_crossed};

/// The 6502 uses two bytes for memory addresses. Not all of it is RAM, cartridge memory is
/// addressed in the same way.
const MEMORY_SIZE: usize = u16::MAX as usize + 1;

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

    pub fn set_from_byte(&mut self, byte: u8) {
        self.carry = (0b00000001 & byte) == 1;
        self.zero = ((0b00000010 & byte) >> 1) == 1;
        self.interrupt_disable = ((0b00000100 & byte) >> 2) == 1;
        self.decimal_mode = ((0b00001000 & byte) >> 3) == 1;
        self.break_command = ((0b00010000 & byte) >> 4) == 1;
        // Bit 5 is ignored
        self.overflow = ((0b01000000 & byte) >> 6) == 1;
        self.negative = ((0b10000000 & byte) >> 7) == 1;
    }

    pub fn from_byte(byte: u8) -> Self {
        let mut flags = Self::new();
        flags.set_from_byte(byte);
        flags
    }

    pub fn as_byte(&self) -> u8 {
        let mut byte = self.negative as u8;
        byte = (byte << 1) | self.overflow as u8;
        // Bit 5 is always 1
        byte = (byte << 1) | 1;
        byte = (byte << 1) | self.break_command as u8;
        byte = (byte << 1) | self.decimal_mode as u8;
        byte = (byte << 1) | self.interrupt_disable as u8;
        byte = (byte << 1) | self.zero as u8;
        (byte << 1) | self.carry as u8
    }
}

struct CPU6502<'a> {
    x: u8,
    y: u8,
    a: u8,
    pc: u16,
    sp: u8,
    cycles: usize,
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
            cycles: 0,
            sp: 0xFF,
            flags: CPUFlags::new(),
            memory
        }
    }

    pub fn load_memory(&mut self, location: u16, data: &[u8]) {
        let _: Vec<_> = data.iter().enumerate().map(|tuple| {
            let (index, byte) = tuple;
            self.memory[index + location as usize] = *byte;
        } ).collect();
    }

    pub fn load_and_execute(&mut self) {
        let instruction = Instruction::decode(&self.memory, self.pc);
        self.execute_instruction(instruction)
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        // Add variable bindings here to keep the execution switch statement (reasonably)
        // concise and readable
        let addressing_mode = instruction.addressing_mode;
        let opcode = instruction.opcode;
        let instruction_data = instruction.data;

        match opcode {
            Opcode::JMP => {
                let (new_address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.pc = new_address as u16;
                // JMP and other branch instructions add 1 cycle if branch occurs to same page, 2 if elsewhere
                self.cycles += instruction.cycles
            },

            Opcode::LDA => {
                let (byte, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.flags.zero = is_zero(byte);
                self.flags.negative = is_negative(byte);
                self.a = byte;
                self.cycles += instruction.cycles;
                if (addressing_mode == AddressingMode::AbsoluteIndexedX || addressing_mode == AddressingMode::AbsoluteIndexedY)
                        && page_boundary_crossed {
                    self.cycles += 1
                }
                self.pc += instruction.width as u16;
            },

            Opcode::LDX => {
                let (byte, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.flags.zero = is_zero(byte);
                self.flags.negative = is_negative(byte);
                self.x = byte;
                self.cycles += instruction.cycles;
                if addressing_mode == AddressingMode::AbsoluteIndexedY && page_boundary_crossed {
                    self.cycles += 1
                }
                self.pc += instruction.width as u16;
            },

            Opcode::LDY => {
                let (byte, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.flags.zero = is_zero(byte);
                self.flags.negative = is_negative(byte);
                self.y = byte;
                self.cycles += instruction.cycles;
                if addressing_mode == AddressingMode::AbsoluteIndexedX && page_boundary_crossed {
                    self.cycles += 1
                }
                self.pc += instruction.width as u16;
            },

            Opcode::STA => {
                let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.memory[address] = self.a;
                self.pc += instruction.width as u16;
                self.cycles += instruction.cycles;
            },

            Opcode::STX => {
                let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.memory[address] = self.x;
                self.pc += instruction.width as u16;
                self.cycles += instruction.cycles;
            },

            Opcode::STY => {
                let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.memory[address] = self.y;
                self.pc += instruction.width as u16;
                self.cycles += instruction.cycles;
            }


            _ => panic!("Unsupported instruction executed")
        }
    }

    /// For instructions which take values as an operand. Takes the two bytes following the opcode and the addressing mode, and returns a tuple containing
    /// the intended the intended operand for the instruction and a bool representing whether a page boundary
    /// has been crossed
    fn get_value_operand(&self, instruction_data: (u8, u8), addressing_mode: AddressingMode) -> (u8, bool) {
        match addressing_mode {
            // Immediate instructions just take the next byte as an operand
            AddressingMode::Immediate => (instruction_data.0, false),

            // Accumulator instructions just need the value in the accumulator
            AddressingMode::Accumulator => (self.a, false),

            // An implied addressing mode effectively means there is no operand. We return 0x00
            // for simplicity
            AddressingMode::Implied => panic!("Cannot resolve operand for addressing mode {:?}", addressing_mode),

            // Other addressing modes need the value at the memory address indicated by the data and
            // the addressing mode
            _ => {
                let (address, page_boundary_crossed) = self.get_address_operand(instruction_data, addressing_mode);
                (self.memory[address], page_boundary_crossed)
            }
        }
    }

    /// For instructions which take an address as an operand. Takes the addressing mode and the two bytes following the instruction and returns a
    /// tuple containing the address and a bool indicating whether a page boundary has been crossed.
    fn get_address_operand(&self, instruction_data: (u8, u8), addressing_mode: AddressingMode) -> (usize, bool) {
        match addressing_mode {
            // Absolute instructions need the value in memory at the address given by the data
            // bytes (little-endian)
            AddressingMode::Absolute => {
                let address = utils::to_address_from_bytes(instruction_data);
                (address, false)
            },

            // Absolute instructions need the value in memory at the address given by the data
            // bytes (little-endian) plus the value in register X
            AddressingMode::AbsoluteIndexedX => {
                let address = utils::to_address_from_bytes(instruction_data);
                let indexed_address = address + self.x as usize;
                (address, was_page_boundary_crossed(address, indexed_address))
            },

            // Absolute instructions need the value in memory at the address given by the data
            // bytes (little-endian) plus the value in register Y
            AddressingMode::AbsoluteIndexedY => {
                let address = utils::to_address_from_bytes(instruction_data);
                let indexed_address = address + self.x as usize;
                (address, was_page_boundary_crossed(address, indexed_address))
            },

            // Returns the byte on the zero page at the address given by the first byte of
            // instruction data
            AddressingMode::ZeroPage => {
                let address = instruction_data.0 as usize;
                (address, false)  
              },
  
              // Returns the byte on the zero page at the address given by indexing the first byte
              // of instruction data with the contents of the X register. This may overflow, which
              // is intended behaviour
              AddressingMode::ZeroPageIndexedX => {
                  let address = (instruction_data.0 + self.x) as usize;
                  (address, false)
              },
  
              // Returns the byte on the zero page at the address given by indexing the first byte
              // of instruction data with the contents of the Y register. This may overflow, which
              // is intended behaviour
              AddressingMode::ZeroPageIndexedY => {
                  let address = (instruction_data.0 + self.y) as usize;
                  (address, false)
              },

              // Indirect is word at address given by reading two bytes from address given by instruction data
            AddressingMode::Indirect => {
                let indirect_address = to_address_from_bytes(instruction_data);
                let address = to_address_from_bytes((self.memory[indirect_address], self.memory[indirect_address+1]));
                (address, false)
            }

            // Indexed indirect retrieves two bytes from the zero page indexed by X to get an address,
            // then returns the word at that address
            AddressingMode::IndexedIndirect => {
                let indirect_address = (instruction_data.0 + self.x) as usize;
                let address = to_address_from_bytes((self.memory[indirect_address], self.memory[indirect_address+1]));
                (address, false)
            }

            // Indirect indexed retrieves two bytes from the zero page to get an address, which is indexed
            // by Y with carry, and the word at that address is returned
            AddressingMode::IndirectIndexed => {
                let address = to_address_from_bytes((instruction_data.0, instruction_data.0 + 1));
                let indexed_address = address + self.y as usize;
                (indexed_address, was_page_boundary_crossed(address, indexed_address))
            },

            // Relative addressing mode takes the address currently in the program counter and adds a signed
            // offset given by the next byte
            AddressingMode::Relative => {
                let offset = (instruction_data.0 as i8) as i32;
                let pc = self.pc as i32;
                let address = (pc + offset) as usize;
                (address, was_page_boundary_crossed(pc as usize, address))
            },

            // All other addressing modes don't refer to an address in memory but a register (or none at all)
            _ => panic!("Can't resolve a memory address for addressing mode {:?}", addressing_mode)
        }
    }
}

impl<'a> Display for CPU6502<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // First half is instruction information
        let instruction = Instruction::decode(&self.memory, self.pc); 
        
        let bytes_fragment: String;
        match instruction.width {
            1 => bytes_fragment = format!("{:02X}        ", instruction.opcode_byte),
            2 => bytes_fragment = format!("{:02X} {:02X}     ", instruction.opcode_byte, instruction.data.0),
            3 => bytes_fragment = format!("{:02X} {:02X} {:02X}  ", instruction.opcode_byte, instruction.data.0, instruction.data.1),
            _ => panic!("Invalid width value {}!", instruction.width)
        };

        let operand_fragment: String;
        match instruction.addressing_mode {
            AddressingMode::Implied => operand_fragment = format!("{:?}", instruction.opcode),
            AddressingMode::Immediate => operand_fragment = format!("{:?} #${:02X}", instruction.opcode, instruction.data.0),
            AddressingMode::Absolute => {
                operand_fragment = format!("{:?} ${:02X}{:02X}", instruction.opcode, instruction.data.1, instruction.data.0);
            },
            AddressingMode::AbsoluteIndexedX => {
                let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                operand_fragment = format!("{:?} ${:02X},X @ {:02X} = {:02X}", instruction.opcode, instruction.data.0, self.x, address as u16);
            },
            AddressingMode::AbsoluteIndexedY => {
                let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                operand_fragment = format!("{:?} ${:02X},Y @ {:02X} = {:02X}", instruction.opcode, instruction.data.0, self.y, address as u16);
            },
            AddressingMode::IndexedIndirect => {
                let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                let byte = self.memory[address]; 
                operand_fragment = format!("{:?} (${:02X},X) @ {:02X} = {:02X} = {:02X}", instruction.opcode, instruction.data.0, self.x, address as u16, byte);
            },
            AddressingMode::ZeroPage => {
                let (byte, _) = self.get_value_operand(instruction.data, instruction.addressing_mode);
                operand_fragment = format!("{:?} ${:02X} = {:02X}", instruction.opcode, instruction.data.0, byte);
            },
            AddressingMode::ZeroPageIndexedX => {
                let (byte, _) = self.get_value_operand(instruction.data, instruction.addressing_mode);
                operand_fragment = format!("{:?} ${:02X},X @ {:02X} = {:02X}", instruction.opcode, instruction.data.0, self.x, byte);
            },
            AddressingMode::ZeroPageIndexedY => {
                let (byte, _) = self.get_value_operand(instruction.data, instruction.addressing_mode);
                operand_fragment = format!("{:?} ${:02X},Y @ {:02X} = {:02X}", instruction.opcode, instruction.data.0, self.y, byte);
            },
            AddressingMode::IndirectIndexed => {
                let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                let byte = self.memory[address]; 
                operand_fragment = format!("{:?} (${:02X}),Y @ {:02X} = {:02X} = {:02X}", instruction.opcode, instruction.data.0, self.y, address as u16, byte);
            },
            AddressingMode::Accumulator => operand_fragment = format!("{:?} A", instruction.opcode),
            AddressingMode::Relative => {
                let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                operand_fragment = format!("{:?} ${:02X}", instruction.opcode, address as u16);
            },
            AddressingMode::Indirect => {
                let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                let byte = self.memory[address]; 
                operand_fragment = format!("{:?} (${:02X}{:02X}) = {:02X}", instruction.opcode, instruction.data.1, instruction.data.0, address as u16);
            },
        };

        let mut first_half = format!("{:X}  {}{}", self.pc, bytes_fragment, operand_fragment);
        let padding_required = 48 - first_half.len();
        first_half += (0..padding_required).map(|_| " ").collect::<String>().as_str();

        // Second is processor state
        let processor_fragment = format!("A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:  0, 00 CYC:{}",
                self.a, self.x, self.y, self.flags.as_byte(), self.sp, self.cycles);

        write!(f, "{}{}", first_half, processor_fragment)
    }
}

fn main() {
    let mut memory = [0 as u8; MEMORY_SIZE];
    let _ = CPU6502::new(memory.as_mut_slice());
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{self, BufRead}};

    use super::*;

    fn remove_ppu_from_log(log: &String) -> String {
        let (first_part, rest) = log.split_at(74);
        let (_, second_part) = rest.split_at(12);
        first_part.to_owned().to_string() + second_part
    }

    #[test]
    fn run_nestest() {
        // We need something against which we can compare our execution of the nestest binary. Fortunately there are
        // log files available. So we open the nestest.log file into a line-by-line iterator
        let log_file = File::open("nestest.log").unwrap();
        let logs = io::BufReader::new(log_file).lines();

        // ...then we set up the CPU as it would be if booting from a cartridge after the start
        // vector has been run
        let mut memory: [u8; MEMORY_SIZE] = [0;MEMORY_SIZE];
        let mut cpu = CPU6502 {
            pc: 0xC000,
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            cycles: 7,
            flags: CPUFlags::from_byte(0x24),
            memory: memory.as_mut_slice()
        };

        // ... load the binary into memory
        cpu.load_memory(0xC000, include_bytes!("../nestest.bin"));

        // ...and iterate through the log lines, executing instructions as we go
        for line in logs.enumerate() {
            if let (line_no, Ok(log)) = line {
                // The logs include PPU information, which we obviously can't test here, so we split the strings
                let cpu_log = remove_ppu_from_log(&cpu.to_string());
                if remove_ppu_from_log(&log.trim().to_string()) == cpu_log {
                    println!("Instruction {} ✓ - {} ", line_no, cpu_log);
                    cpu.load_and_execute();
                } else {
                    std::panic!("Expected {}, got {}", remove_ppu_from_log(&log), remove_ppu_from_log(&cpu.to_string()))
                }
            }
        }
    }
}