/// Addresses are big-endian in our memory array, but are retrieved
/// little-endian. This function takes as input the pair of bytes
/// retrieved, and returns the memory address they encode 
pub fn to_address_from_bytes(bytes: (u8, u8)) -> usize {
    let big_byte = bytes.1 as usize;
    let little_byte = bytes.0 as usize;
    let address = big_byte << 8;
    address + little_byte
}

/// The address space is conceived of as consisting of 256-byte pages.
/// Crossing a page boundary when addressing incurs an additional cycle
/// depending on the instruction, so we need to know when it happens.
pub fn was_page_boundary_crossed(address: usize, indexed_address: usize) -> bool {
    let address = address as u16;
    let indexed_address = indexed_address as u16;
    let bitmask: u16 = 0xFF00;

    // The high byte of the address (when thought of as two bytes)
    // is effectively a page index. Bitwise-and'ing with the given
    // mask leaves just the high byte, and comparing the result gives
    // us the answer

    (address & bitmask) != (indexed_address & bitmask)
}