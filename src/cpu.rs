mod registers;
mod instructions;

use crate::cpu::registers::{Registers, FlagsRegister};
use crate::cpu::instructions::{Instructions, RegisterTarget, VirtualRegisterTarget};


struct CPU {
    registers: Registers,
    pc: u16,
}
impl CPU {
    fn execute(&mut self, instruction: Instructions) {
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
        self.registers.f.zero = is_zero;
        self.registers.f.subtract = did_subtract;
        self.registers.f.carry = did_carry;
        self.registers.f.half_carry = did_half_carry;
    }
}

#[cfg(test)]
mod cpu_tests {
    use super::*;

    fn use_test_cpu() -> CPU{
        return CPU {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: 5,
                f: FlagsRegister {
                    zero: false,
                    subtract: false,
                    half_carry: false,
                    carry: false,
                },
                h: 6,
                l: 7
            },
            pc: 1,
        }
    }

    fn check_flags_register(cpu_flags: FlagsRegister, compare_flags: FlagsRegister) {
        assert_eq!(cpu_flags.zero, compare_flags.zero);
        assert_eq!(cpu_flags.subtract, compare_flags.subtract);
        assert_eq!(cpu_flags.carry, compare_flags.carry);
        assert_eq!(cpu_flags.half_carry, compare_flags.half_carry);

        assert_eq!(u8::from(cpu_flags), u8::from(compare_flags));

    }

    #[test]
    fn test_add() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::ADD(RegisterTarget::C));
        assert_eq!(cpu.registers.a, 4);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:false, carry: false, half_carry: false});
    }
    #[test]
    fn test_addhl() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::ADDHL(VirtualRegisterTarget::BC));
        assert_eq!(cpu.registers.get_hl(), 2058);
        assert_eq!(cpu.registers.h, 8);
        assert_eq!(cpu.registers.l, 10);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:false, carry: false, half_carry: false});

    }
    #[test]
    fn test_adc() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::ADC(RegisterTarget::B));
        assert_eq!(cpu.registers.a, 3);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:false, carry: false, half_carry: false});

    }
    #[test]
    fn test_sub() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::SUB(RegisterTarget::C));
        assert_eq!(cpu.registers.a, 254);
        assert!(cpu.registers.f.carry);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:true, carry: true, half_carry: true});

    }
    #[test]
    fn test_sbc() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::SBC(RegisterTarget::B));
        assert_eq!(cpu.registers.a, 255);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:true, carry: true, half_carry: true});

    }
    #[test]
    fn test_and() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::AND(RegisterTarget::D));
        assert_eq!(cpu.registers.a, 0);
        let mut cpu2 = use_test_cpu();
        cpu2.execute(Instructions::AND(RegisterTarget::A));
        assert_eq!(cpu2.registers.a, 1);

        check_flags_register(cpu.registers.f, FlagsRegister{zero:true, subtract:false, carry: false, half_carry: true});
        check_flags_register(cpu2.registers.f, FlagsRegister{zero:false, subtract:false, carry: false, half_carry: true});

    }
    #[test]
    fn test_or() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::OR(RegisterTarget::E));
        assert_eq!(cpu.registers.a, 5);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:false, carry: false, half_carry: false});

    }
    #[test]
    fn test_xor() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::XOR(RegisterTarget::C));
        assert_eq!(cpu.registers.a, 2);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:false, carry: false, half_carry: false});

    }
    #[test]
    fn test_cp() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::CP(RegisterTarget::C));
        assert_eq!(cpu.registers.a, 1);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:true, carry: true, half_carry: true});

    }
    #[test]
    fn test_inc() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::INC(RegisterTarget::H));
        assert_eq!(cpu.registers.h, 7);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:false, carry:false, half_carry:false});
    }
    #[test]
    fn test_dec() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::DEC(RegisterTarget::L));
        assert_eq!(cpu.registers.l, 6);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:true, carry:false, half_carry:false});
    }
}