pub mod registers;
pub mod instructions;

use registers::{Registers, FlagsRegister};
use instructions::{Instructions, RegisterTarget, VirtualRegisterTarget};
#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    // pc: u16,
}
impl CPU {
    pub fn new() -> CPU {
        return CPU {
            registers: Registers::new(),
            // pc: 1
        }
    }
    pub fn execute(&mut self, instruction: Instructions) {
        match instruction {
            Instructions::ADD(target) => {
                let register_value = self.get_target_register(&target);
                let (result, did_overflow) = self.registers.a.overflowing_add(register_value);
                self.set_flags_register(
                    result == 0, 
                    false, 
                    did_overflow, 
                    // Half carry is set if adding the lower nibbles of the value and register A together
                    // result in a value bigger than 0xF. If the result is larger than 0xF
                    // than the addition caused a carry from the lower nibble to the upper nibble
                    ((self.registers.a & 0xF) + (register_value & 0xF)) > 0xF
                );
                self.set_target_register(RegisterTarget::A, result);
            }
            Instructions::ADDHL(target) => {
                let register_value = match target {
                    VirtualRegisterTarget::BC => self.registers.get_bc(),
                    VirtualRegisterTarget::DE => self.registers.get_de(),
                    VirtualRegisterTarget::HL => self.registers.get_hl(),
                    _ => 0,
                };
                let hl = self.registers.get_hl();
                let (result, did_overflow) = hl.overflowing_add(register_value);
                self.set_flags_register(
                    result == 0, 
                    false, 
                    did_overflow, 
                    ((register_value & 0xFFF) + (hl & 0xFFF)) > 0xFFF
                );
                self.registers.set_hl(result);
            }
            Instructions::ADC(target) => {
                let additional_carry = if self.registers.f.carry { 1 } else { 0 };
                let register_value = self.get_target_register(&target);
                let (result, did_overflow) = self.registers.a.overflowing_add(register_value);
                let (new_result, new_did_overflow) = result.overflowing_add(additional_carry);
                self.set_flags_register(
                    result == 0, 
                    false, 
                    new_did_overflow || did_overflow, 
                    ((self.registers.a & 0xF) + (register_value & 0xF) + additional_carry) > 0xF
                );
                self.set_target_register(RegisterTarget::A, new_result);
                // self.registers.a = new_result;
            }
            Instructions::SUB(target) => {
                let register_value = self.get_target_register(&target);
                let (result, did_overflow) = self.registers.a.overflowing_sub(register_value);
                self.set_flags_register(
                    result == 0, 
                    true, 
                    did_overflow, 
                    (self.registers.a & 0xF) < (register_value & 0xF)
                );
                // self.registers.a = result;
                self.set_target_register(RegisterTarget::A, result);
            }
            Instructions::SBC(target) => {
                let additional_carry = if self.registers.f.carry { 1 } else { 0 };
                let register_value = self.get_target_register(&target);
                let (result, did_overflow) = self.registers.a.overflowing_sub(register_value);
                let (new_result, new_did_overflow) = result.overflowing_sub(additional_carry);
                self.set_flags_register(
                    new_result == 0, 
                    true, 
                    new_did_overflow || did_overflow, 
                    (self.registers.a & 0xF) < (register_value & 0xF) + additional_carry
                );
                self.set_target_register(RegisterTarget::A, new_result);
                // self.registers.a = new_result;
            }
            Instructions::AND(target) => {
                let register_value = self.get_target_register(&target);
                let result = register_value & self.registers.a;
                self.set_flags_register(
                    result == 0, 
                    false, 
                    false, 
                    true
                );
                self.set_target_register(RegisterTarget::A, result)
                // self.registers.a = result;
            }
            Instructions::OR(target) => {
                let register_value = self.get_target_register(&target);
                let result = register_value | self.registers.a;
                self.set_flags_register(
                    result == 0, 
                    false, 
                    false, 
                    false
                );
                self.set_target_register(RegisterTarget::A, result)
                // self.registers.a = result;
            }
            Instructions::XOR(target) => {
                let register_target = self.get_target_register(&target);
                let result = register_target ^ self.registers.a;
                self.set_flags_register(
                    result == 0, 
                    false, 
                    false, 
                    false
                );
                self.registers.a = result;
                self.set_target_register(RegisterTarget::A, result);
            }
            Instructions::CP(target) => {
                let register_value = self.get_target_register(&target);
                let (result, did_overflow) = self.registers.a.overflowing_sub(register_value);
                self.set_flags_register(
                    result == 0,
                    true,
                    did_overflow,
                    (self.registers.a & 0xF) < (result & 0xF)
                );
            }
            Instructions::INC(target) => {
                let register_value = self.get_target_register(&target);
                let (result, did_overflow) = register_value.overflowing_add(1);
                self.set_flags_register(
                    result == 0,
                    false,
                    did_overflow,
                    ((result & 0xF) + (register_value & 0xF)) > 0xF
                );
                self.set_target_register(target, result);
            }
            Instructions::DEC(target) => {
                let register_value = self.get_target_register(&target);
                let (result, did_overflow) = register_value.overflowing_sub(1);
                self.set_flags_register(
                    result == 0,
                    true,
                    did_overflow,
                    (register_value & 0xF) < (result & 0xF)
                );
                self.set_target_register(target, result);
            }
            Instructions::CCF => {
                self.set_flags_register(
                    self.registers.f.zero, 
                    false,
                    !self.registers.f.carry, 
                    false
                );
            }
            Instructions::SCF => {
                self.set_flags_register(
                    self.registers.f.zero,
                    false,
                    true,
                     false
                );
            }
            Instructions::RRA => {
                let carry_bit = if self.registers.f.carry { 1 } else { 0 } << 7;
                let register_value = self.get_target_register(&RegisterTarget::A);
                let new_value = carry_bit | (register_value >> 1);
                self.set_flags_register(
                    false,
                    false,
                    register_value & 0b1 == 0b1,
                    false,
                );
                self.set_target_register(RegisterTarget::A, new_value);
            }
            Instructions::RLA => {
                let carry_bit = if self.registers.f.carry { 1 } else { 0 } << 7;
                let register_value = self.get_target_register(&RegisterTarget::A);
                let new_value = (register_value << 1) | carry_bit;
                self.set_flags_register(
                    false,
                    false,
                    (register_value & 0x80) == 0x80,
                    false,
                );
                self.set_target_register(RegisterTarget::A, new_value);
            }
            Instructions::RRCA => {
                let register_value = self.get_target_register(&RegisterTarget::A);
                let new_value = register_value.rotate_right(1);
                self.set_flags_register(
                    false,
                    false,
                    register_value & 0b1 == 0b1,
                    false
                );
                self.set_target_register(RegisterTarget::A, new_value);
            }
            Instructions::RRLA => {
                let register_value = self.get_target_register(&RegisterTarget::A);
                let carry = (register_value & 0x80) >> 7;
                let new_value = register_value.rotate_left(1) | carry;
                self.set_flags_register(
                    false, 
                    false, 
                    carry == 0x01, 
                    false
                );
                self.set_target_register(RegisterTarget::A, new_value);
            }
            Instructions::CPL => {
                let new_value = !self.get_target_register(&RegisterTarget::A);
                self.set_flags_register(
                    false, 
                    true, 
                    false, 
                    true);
                self.set_target_register(RegisterTarget::A, new_value);
            }

            _ => {
                println!("Other instructions coming soon...")
            }
        }

    }

    fn get_target_register(&self, target: &RegisterTarget) -> u8 {
        match target {
            RegisterTarget::A => self.registers.a,
            RegisterTarget::B => self.registers.b,
            RegisterTarget::C => self.registers.c,
            RegisterTarget::D => self.registers.d,
            RegisterTarget::E => self.registers.e,
            RegisterTarget::H => self.registers.h,
            RegisterTarget::L => self.registers.l,
        }
    }

    fn set_target_register(&mut self, target: RegisterTarget, value: u8) {
        match target {
            RegisterTarget::A => self.registers.a = value,
            RegisterTarget::B => self.registers.b = value,
            RegisterTarget::C => self.registers.c = value,
            RegisterTarget::D => self.registers.d = value,
            RegisterTarget::E => self.registers.e = value,
            RegisterTarget::H => self.registers.h = value,
            RegisterTarget::L => self.registers.l = value,


        }
    }

    fn set_flags_register(&mut self, is_zero: bool, did_subtract: bool, did_carry: bool, did_half_carry: bool) {
        self.registers.f = FlagsRegister {
            zero: is_zero,
            subtract: did_subtract,
            carry: did_carry,
            half_carry: did_half_carry,
        }
    }
}
