mod instruction;
mod utils;
use core::panic;
use std::{collections::btree_map, fmt::{write, Display}, ops::Add, u16};

use instruction::{AddressingMode, Instruction, Opcode};
use utils::{is_negative, is_zero, to_address_from_bytes, to_bytes_from_address, was_page_boundary_crossed};

/// The 6502 uses two bytes for memory addresses. Not all of it is RAM, cartridge memory is
/// addressed in the same way.
const MEMORY_SIZE: usize = u16::MAX as usize + 1;
const STACK_PAGE : u16 = 0x0100;

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

    pub fn push_on_stack(&mut self, byte: u8) {
        let address = STACK_PAGE + self.sp as u16;
        self.memory[address as usize] = byte;
        // Stack is addressed top-down - i.e. stack pointer of 0xFF means empty stack
        // and a stack pointer of 0x00 means a full stack - so we decrement the pointer
        self.sp -= 1;
    }

    pub fn pop_from_stack(&mut self) -> u8 {
        self.sp += 1;
        let address = STACK_PAGE + self.sp as u16;
        let byte = self.memory[address as usize];
        byte
    }

    fn load_memory(&mut self, location: u16, data: &[u8]) {
        let _: Vec<_> = data.iter().enumerate().map(|tuple| {
            let (index, byte) = tuple;
            self.memory[index + location as usize] = *byte;
        } ).collect();
    }

    pub fn load_and_execute(&mut self) {
        let instruction = Instruction::decode(&self.memory, self.pc);
        self.execute_instruction(instruction)
    }

    fn set_flags(&mut self, byte: u8) {
        self.flags.zero = is_zero(byte);
        self.flags.negative = is_negative(byte);
    }

    fn compare_and_set_flags(&mut self, register_byte: u8, memory_byte: u8) {
        let result = register_byte.wrapping_sub(memory_byte);
        self.set_flags(result);
        self.flags.carry = !(register_byte < memory_byte);
    }

    fn branch_on_condition(&mut self, condition: bool, instruction: &Instruction) {
        if condition {
            let (branch_address, page_boundary_crossed) = self.get_address_operand(instruction.data, instruction.addressing_mode);
            // Pre-decrement the PC with the width, because the execution loop will increment it afterwards
            self.pc = (branch_address - instruction.width) as u16;
            self.cycles += 1;
            if page_boundary_crossed { self.cycles += 1 }
        }  
    }

    fn add_extra_cycles(&mut self, addressing_mode: &AddressingMode, page_boundary_crossed: bool) {
        match addressing_mode {
            AddressingMode::AbsoluteIndexedX | AddressingMode::AbsoluteIndexedY | AddressingMode::IndirectIndexed => {
                if page_boundary_crossed { self.cycles += 1 }
            },
            _ => ()
        }
    }

    fn add_with_carry(&mut self, operand: u8) {
        let result = self.a.wrapping_add(operand).wrapping_add(self.flags.carry as u8);
        self.set_flags(result);
        self.flags.carry = self.a > result;
        // Overflow occurs when the operands have the same sign bit, but the result does not
        self.flags.overflow = ((!(self.a ^ operand)) & 0x80  // true when operands have same sign
                            & ((operand ^ result))) == 0x80; // and result is different 
        self.a = result;
        self.set_flags(self.a)
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        // Add variable bindings here to keep the execution switch statement (reasonably)
        // concise and readable
        let addressing_mode = instruction.addressing_mode;
        let opcode = instruction.opcode;
        let instruction_data = instruction.data;

        match opcode {
            Opcode::ADC => {
                let (operand, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.add_with_carry(operand);
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed);
            },

            Opcode::AND => {
                let (operand, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.a &= operand;
                self.set_flags(self.a);
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed);
            },

            Opcode::ASL => {
                if addressing_mode != AddressingMode::Accumulator {
                    let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                    let byte = self.memory[address];
                    self.flags.carry = (byte & 0b10000000) == 0b10000000;
                    self.memory[address] = byte << 1;
                    self.set_flags(self.memory[address]);
                    
                } else {
                    let (operand, _) = self.get_value_operand(instruction_data, addressing_mode);
                    self.flags.carry = (operand & 0b10000000) == 0b10000000;
                    self.a = operand << 1;
                    self.set_flags(self.a);
                }
            }

            Opcode::BCC => self.branch_on_condition(!self.flags.carry, &instruction),
            Opcode::BCS => self.branch_on_condition(self.flags.carry, &instruction),
            Opcode::BEQ => self.branch_on_condition(self.flags.zero, &instruction),

            Opcode::BIT => {
                let (byte, _) = self.get_value_operand(instruction_data, addressing_mode);
                self.flags.negative = ((0b10000000 & byte) >> 7) == 1;
                self.flags.overflow = ((0b01000000 & byte) >> 6) == 1;
                self.flags.zero = (self.a & byte) == 0;
            },

            Opcode::BMI => self.branch_on_condition(self.flags.negative, &instruction),
            Opcode::BNE => self.branch_on_condition(!self.flags.zero, &instruction),
            Opcode::BPL => self.branch_on_condition(!self.flags.negative, &instruction),
            Opcode::BVC => self.branch_on_condition(!self.flags.overflow, &instruction),
            Opcode::BVS => self.branch_on_condition(self.flags.overflow, &instruction),

            Opcode::CLC => self.flags.carry = false,
            Opcode::CLD => self.flags.decimal_mode = false,
            Opcode::CLI => self.flags.interrupt_disable = false,
            Opcode::CLV => self.flags.overflow = false,

            Opcode::CMP => {
                let (operand, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.compare_and_set_flags(self.a, operand);
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed);
            },
            Opcode::CPX => {
                let (operand, _) = self.get_value_operand(instruction_data, addressing_mode);
                self.compare_and_set_flags(self.x, operand);
            },
            Opcode::CPY => {
                let (operand, _) = self.get_value_operand(instruction_data, addressing_mode);
                self.compare_and_set_flags(self.y, operand);
            },

            Opcode::DEC => {
                let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.memory[address] = self.memory[address].wrapping_sub(1);
                self.set_flags(self.memory[address]);
            },
            Opcode::DEX => {
                self.x = self.x.wrapping_sub(1);
                self.set_flags(self.x);
            },
            Opcode::DEY => {
                self.y = self.y.wrapping_sub(1);
                self.set_flags(self.y);
            },

            Opcode::EOR => {
                let (operand, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.a ^= operand;
                self.set_flags(self.a);
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed);
            },

            Opcode::INC => {
                let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.memory[address] = self.memory[address].wrapping_add(1);
                self.set_flags(self.memory[address]);
            },
            Opcode::INX => {
                self.x = self.x.wrapping_add(1);
                self.set_flags(self.x);
            },
            Opcode::INY => {
                self.y = self.y.wrapping_add(1);
                self.set_flags(self.y);
            },

            Opcode::JMP => {
                let (new_address, _) = self.get_address_operand(instruction_data, addressing_mode);
                // Pre-decrement the PC with the width, because the execution loop will increment it afterwards
                self.pc = (new_address - instruction.width) as u16;
            },
            Opcode::JSR => {
                // Return address is next instruction - or PC plus 2
                let return_address_bytes = to_bytes_from_address(self.pc + 2);
                self.push_on_stack(return_address_bytes.1);
                self.push_on_stack(return_address_bytes.0);
                let (new_address, _) = self.get_address_operand(instruction_data, addressing_mode);
                // Pre-decrement the PC with the width, because the execution loop will increment it afterwards
                self.pc = (new_address - instruction.width) as u16;
            },
            Opcode::LDA => {
                let (byte, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.a = byte;
                self.set_flags(self.a);
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed);
            },
            Opcode::LDX => {
                let (byte, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.set_flags(byte);
                self.x = byte;      
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed);
            },
            Opcode::LDY => {
                let (byte, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.set_flags(byte);
                self.y = byte;            
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed); 
            },

            Opcode::LSR => {
                if addressing_mode != AddressingMode::Accumulator {
                    let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                    let byte = self.memory[address];
                    self.flags.carry = (byte & 0x01) == 1;
                    self.memory[address] = byte >> 1;
                    self.set_flags(self.memory[address]);
                }
                else {
                    let (byte, _) = self.get_value_operand(instruction_data, addressing_mode);
                    self.flags.carry = (byte & 0x01) == 1;
                    self.a = byte >> 1;
                    self.set_flags(self.a);
                }
            }

            Opcode::NOP => (),

            Opcode::ORA => {
                let (operand, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.a |= operand;
                self.set_flags(self.a);
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed);
            }

            Opcode::PHA => self.push_on_stack(self.a),
            // PHP sets the break flag on the value pushed to the stack
            // Ref: https://www.nesdev.org/wiki/Status_flags#The_B_flag
            Opcode::PHP => self.push_on_stack(self.flags.as_byte() | 0b00010000),
            Opcode::PLA => {
                self.a = self.pop_from_stack();
                self.set_flags(self.a);  
            },
            Opcode::PLP => {
                let new_flags = self.pop_from_stack();
                self.flags.set_from_byte(new_flags);
                self.flags.break_command = false;  
            },

            Opcode::ROL => {
                let carry = self.flags.carry as u8;
                if addressing_mode != AddressingMode::Accumulator {
                    let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                    let byte = self.memory[address];
                    self.flags.carry = (byte & 0b10000000) == 0b10000000;
                    self.memory[address] = (byte << 1) + carry;
                    self.set_flags(self.memory[address]);
                } else {
                    let (operand, _) = self.get_value_operand(instruction_data, addressing_mode);
                    self.flags.carry = (operand & 0b10000000) == 0b10000000;
                    self.a = (operand << 1) + carry;
                    self.set_flags(self.a);
                } 
            },
            Opcode::ROR => {
                let carry = self.flags.carry as u8;
                if addressing_mode != AddressingMode::Accumulator {
                    let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                    let byte = self.memory[address];
                    self.flags.carry = (byte & 0x01) == 1;
                    self.memory[address] = (byte >> 1) + (carry << 7);
                    self.set_flags(self.memory[address]);
                } else {
                    let (operand, _) = self.get_value_operand(instruction_data, addressing_mode);
                    self.flags.carry = (operand & 0x01) == 1;
                    self.a = (operand >> 1) + (carry << 7);
                    self.set_flags(self.a);
                }
            }

            Opcode::RTI => {
                let new_flags = self.pop_from_stack();
                self.flags.set_from_byte(new_flags);
                self.flags.break_command = false;  
                let lo_byte = self.pop_from_stack();
                let address = to_address_from_bytes((lo_byte, self.pop_from_stack())) as u16;
                // Pre-decrement address, as it's incremented again in the execution loop
                self.pc = address.wrapping_sub(1);  
            },

            Opcode::RTS => {
                let lo_byte = self.pop_from_stack();
                let address = to_address_from_bytes((lo_byte, self.pop_from_stack())) as u16;
                self.pc = address;  
            },

            Opcode::SBC => {
                let (operand, page_boundary_crossed) = self.get_value_operand(instruction_data, addressing_mode);
                self.add_with_carry(!operand);
                self.add_extra_cycles(&addressing_mode, page_boundary_crossed);
            },

            Opcode::SEC => self.flags.carry = true,
            Opcode::SED => self.flags.decimal_mode = true,
            Opcode::SEI => self.flags.interrupt_disable = true,

            Opcode::STA => {
                let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.memory[address] = self.a;
            },
            Opcode::STX => {
                let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.memory[address] = self.x; 
            },
            Opcode::STY => {
                let (address, _) = self.get_address_operand(instruction_data, addressing_mode);
                self.memory[address] = self.y;        
            },

            Opcode::TAX => {
                self.x = self.a;
                self.set_flags(self.x);
            },
            Opcode::TAY => {
                self.y = self.a;
                self.set_flags(self.y);
            },
            Opcode::TSX => {
                self.x = self.sp;
                self.set_flags(self.x);
            },
            Opcode::TXA => {
                self.a = self.x;
                self.set_flags(self.a);
            },
            Opcode::TXS => self.sp = self.x,
            Opcode::TYA => {
                self.a = self.y;
                self.set_flags(self.a);
            },

            _ => panic!("Unsupported instruction executed")
        }
        
        self.pc += instruction.width as u16;
        self.cycles += instruction.cycles;
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
                let indirect_address = (instruction_data.0.wrapping_add(self.x));
                let address = to_address_from_bytes((self.memory[indirect_address as usize], self.memory[indirect_address.wrapping_add(1) as usize]));
                (address, false)
            }

            // Indirect indexed retrieves two bytes from the zero page to get an address, which is indexed
            // by Y with carry, and the word at that address is returned
            AddressingMode::IndirectIndexed => {
                let address = to_address_from_bytes((self.memory[instruction_data.0 as usize],
                    self.memory[instruction_data.0.wrapping_add(1) as usize])) as u16;
                let indexed_address = address.wrapping_add(self.y as u16) as usize;
                (indexed_address, was_page_boundary_crossed(address as usize, indexed_address))
            },

            // Relative addressing mode takes the address of the next instruction and adds a signed
            // offset given by the next byte
            AddressingMode::Relative => {
                let offset = (instruction_data.0 as i8) as i32;
                // Relative instructions are always two bytes wide, so the next instruction is always
                // the PC plus 2
                let pc = (self.pc + 2) as i32;
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
                // Infuriatingly some instructions seem to have special logging requirements, where the value at the address is included
                match instruction.opcode {
                    Opcode::STX | Opcode::STY | Opcode::STA | Opcode::LDA | Opcode::LDX | Opcode::LDY | Opcode::BIT
                    | Opcode::ORA | Opcode::AND | Opcode::EOR | Opcode::ADC | Opcode::SBC | Opcode::CMP | Opcode::CPX
                    | Opcode::CPY | Opcode::LSR | Opcode::ASL | Opcode::ROR | Opcode::ROL | Opcode::INC | Opcode::DEC => {
                        let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                        let byte = self.memory[address]; 
                        operand_fragment = format!("{:?} ${:02X}{:02X} = {:02X}", instruction.opcode, instruction.data.1, instruction.data.0, byte)
                    },
                    _ => operand_fragment = format!("{:?} ${:02X}{:02X}", instruction.opcode, instruction.data.1, instruction.data.0)

                }
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
                operand_fragment = format!("{:?} (${:02X},X) @ {:02X} = {:04X} = {:02X}", instruction.opcode, instruction.data.0, self.x.wrapping_add(instruction.data.0), address as u16, byte);
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
                let address = to_address_from_bytes((self.memory[instruction.data.0 as usize],
                    self.memory[instruction.data.0.wrapping_add(1) as usize]));
                let (end_address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                let byte = self.memory[end_address]; 
                operand_fragment = format!("{:?} (${:02X}),Y = {:04X} @ {:04X} = {:02X}", instruction.opcode, instruction.data.0, address, end_address, byte);
            },
            AddressingMode::Accumulator => operand_fragment = format!("{:?} A", instruction.opcode),
            AddressingMode::Relative => {
                let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
                operand_fragment = format!("{:?} ${:02X}", instruction.opcode, address as u16);
            },
            AddressingMode::Indirect => {
                let (address, _) = self.get_address_operand(instruction.data, instruction.addressing_mode);
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
    let cpu = CPU6502::new(memory.as_mut_slice());
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
                    std::panic!("Expected\n{},\ngot\n{}", remove_ppu_from_log(&log), remove_ppu_from_log(&cpu.to_string()))
                }
            }
        }
    }
}