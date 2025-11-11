pub enum Instructions {
    ADD(RegisterTarget),   // Adds a specific register to the A register
    ADDHL(VirtualRegisterTarget), // ADD to the HL register
    ADC(RegisterTarget),   // ADD with the carry flag
    SUB(RegisterTarget),   // Subtracts a specific register with A register
    SBC(RegisterTarget),   // SUB with the carry flag
    AND(RegisterTarget),   // Bitwise AND with a specific register and A register
    OR(RegisterTarget),    // Bitwise OR with a specific register and A register
    XOR(RegisterTarget),   // Bitwase XOR with a specific register and A register
    CP(RegisterTarget),    // SUB except the value is not stored back in A register
    INC(RegisterTarget),   // Increment a specific register by 1
    DEC(RegisterTarget),   // Decrement a specific register by 1
    CCF,                   // Toggle the carry flag
    SCF,                   // Set carry flag to true
    RRA,                   // Bit rotate A register right through the carry flag
    RLA,                   // Bit rotate A register left through the carry flag
    RRCA,                  // Bit rotate A register right 
    RRLA,                  // Bit rotate A register left
    CPL,                   // Toggle every bit of A register
    DAA,
    BIT(RegisterTarget, BitPosition),   // Test to see if a specific bit of a register is set
    RESET(RegisterTarget, BitPosition), // Set a specific bit of a register to 0
    SET(RegisterTarget, BitPosition),   // Set a specific bit of a register to 1
    SRL(RegisterTarget),   // Bit shift a specific register right by 1
    RR(RegisterTarget),    // Bit rotate a specific register right by 1 through the carry flag
    RL(RegisterTarget),    // Bit rotate a specific register left by 1 through the carry flag
    RRC(RegisterTarget),   // Bit rotate a specific register right by 1
    RLC(RegisterTarget),   // Bit rotate a specific register left by 1
    SRA(RegisterTarget),   // Arithmetic shift a specific register right by 1
    SLA(RegisterTarget),   // Arithmetic shift a specific register left by 1
    SWAP(RegisterTarget),  // Switch upper and lower nibble of a specific register 
}

pub enum RegisterTarget {
    A, B, C, D, E, H, L, 
}

pub enum VirtualRegisterTarget {
    BC, DE, HL, SP,
}

pub enum BitPosition {
    B0, B1, B2, B3, B4, B5, B6, B7
}
impl std::convert::From<BitPosition> for u8 {
    fn from(position: BitPosition) -> u8 {
        match position {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7,
        }
    }
}