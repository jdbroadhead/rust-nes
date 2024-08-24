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
}

fn main() {
    let mut memory = [0 as u8; MEMORY_SIZE];
    let mut cpu = CPU6502::new(memory.as_mut_slice());
}
