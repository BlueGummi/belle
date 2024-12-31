use crate::Argument::*;
use crate::Instruction::*;
#[allow(unused_imports)] // dumb things that only work in macros (ugh)
use crate::*;
#[test]
fn add_with_immediate() {
    let mut bcpu = CPU::new();
    test_instruction!(bcpu, add, "r0", "3");
    assert_eq!(bcpu.int_reg[0], 3);
}

#[test]
fn add_with_register() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 5;
    bcpu.int_reg[2] = 12;

    test_instruction!(bcpu, add, "r0", "r2");
    assert_eq!(bcpu.int_reg[0], 17);
}

#[test]
fn add_with_rptr() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 21;
    bcpu.int_reg[2] = 33;

    bcpu.memory[33] = Some(42);

    test_instruction!(bcpu, add, "r0", "&r2");
    assert_eq!(bcpu.int_reg[0], 63);
}

#[test]
fn add_with_mptr() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 444;

    bcpu.memory[33] = Some(42);

    bcpu.memory[42] = Some(60);

    test_instruction!(bcpu, add, "r0", "&$33");
    assert_eq!(bcpu.int_reg[0], 504);
}

#[test]
fn add_with_negative_register() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = -42;

    bcpu.uint_reg[1] = 42;

    test_instruction!(bcpu, add, "r5", "r0");
    assert_eq!(bcpu.uint_reg[1], 0);
    test_instruction!(bcpu, add, "r5", "r0");
    assert_eq!(bcpu.uint_reg[1], 65494);
}

#[test]
fn hlt_test() {
    let mut bcpu = CPU::new();

    bcpu.running = true;

    test_instruction!(bcpu, hlt);
    assert_eq!(bcpu.running, false);
}

#[test]
fn jo_test_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, add, "r0", "4444");
    test_instruction!(bcpu, mul, "r0", "r0");
    assert_eq!(bcpu.oflag, true);
    test_instruction!(bcpu, jo, "$300");
    assert_eq!(bcpu.pc, 300);
}

#[test]
fn jo_test_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jo, "$300");
    assert_eq!(bcpu.pc, 1);
}

#[test]
fn jz_test_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, cmp, "r0", "r0");
    assert_eq!(bcpu.zflag, true);

    test_instruction!(bcpu, jz, "$300");
    assert_eq!(bcpu.pc, 300);
}

#[test]
fn jz_test_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jz, "$120");
    assert_eq!(bcpu.pc, 1);
}

#[test]
fn jmp_test_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jmp, "$320");
    assert_eq!(bcpu.pc, 320);
}
