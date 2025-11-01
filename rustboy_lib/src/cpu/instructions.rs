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
    CCF(RegisterTarget),   // Toggle the carry flag
    SCF(RegisterTarget),   // Set carry flag to true
    RRA(RegisterTarget),   // Bit rotate A register right through the carry flag
    RLA(RegisterTarget),   // Bit rotate A register left through the carry flag
    RRCA(RegisterTarget),  // Bit rotate A register right 
    RRLA(RegisterTarget),  // Bit rotate A register left
    CPL(RegisterTarget),   // Toggle every bit of A register
    BIT(RegisterTarget),   // Test to see if a specific bit of a register is set
    RESET(RegisterTarget), // Set a specific bit of a register to 0
    SET(RegisterTarget),   // Set a specific bit of a register to 1
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