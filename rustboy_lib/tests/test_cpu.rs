use rustboy_lib::cpu::{
    registers::{Registers, FlagsRegister}, 
    instructions::{Instructions, RegisterTarget, VirtualRegisterTarget, BitPosition},
    CPU
};


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
            // pc: 1,
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
    #[test]
    fn test_ccf() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::CCF);
        check_flags_register(cpu.registers.f, FlagsRegister{zero: false, subtract:false, carry: true, half_carry:false});
    }
    #[test]
    fn test_scf() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::SCF);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:false, carry:true, half_carry:false});
    }
    #[test]
    fn test_rra() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::RRA);
        assert_eq!(cpu.registers.a, 0);
        check_flags_register(cpu.registers.f, FlagsRegister { zero:false, subtract:false, half_carry:false, carry:true});
    }
    #[test]
    fn test_rla() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::RLA);
        assert_eq!(cpu.registers.a, 2);
        check_flags_register(cpu.registers.f, FlagsRegister { zero:false, subtract:false, half_carry:false, carry:false})
    }
    #[test]
    fn test_rrca() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::RRCA);
        assert_eq!(cpu.registers.a, 128);
        check_flags_register(cpu.registers.f, FlagsRegister { zero:false, subtract:false, half_carry:false, carry:true});
    }
    #[test]
    fn test_rrla() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::RRLA);
        assert_eq!(cpu.registers.a, 2);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract:false, carry:false, half_carry:false});
    }
    #[test]
    fn test_cpl() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::CPL);
        assert_eq!(cpu.registers.a, 254);
        check_flags_register(cpu.registers.f, FlagsRegister { zero:false, subtract:true, half_carry:true, carry:false });
    }
    #[test]
    fn test_bit() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::BIT(RegisterTarget::D, BitPosition::B3));
        assert_eq!(cpu.registers.d, 4);
        check_flags_register(cpu.registers.f, FlagsRegister { zero:true, subtract:false, half_carry:true, carry: false});
    }
    #[test]
    fn test_reset() {
        let mut cpu = use_test_cpu();
        cpu.execute(Instructions::RESET(RegisterTarget::H, BitPosition::B3));
        assert_eq!(cpu.registers.h, 6);
        check_flags_register(cpu.registers.f, FlagsRegister{zero:false, subtract: false, half_carry:false, carry:false});
    }
}