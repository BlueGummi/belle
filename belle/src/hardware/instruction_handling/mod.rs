pub mod interrupt;
pub mod jump;
pub mod load_store;
pub mod mov_and_math;
pub mod push_pop;
pub mod ret_cmp;

#[allow(unused_imports)]
use crate::test_instruction;
#[allow(unused_imports)] // please stop complaining about this clippy
use crate::CPU;

#[test]
fn add_with_immediate() {
    let mut bcpu = CPU::new();
    test_instruction!(bcpu, add, "r0", "3");
    assert_eq!(bcpu.int_reg[0], 3);
}

#[test]
fn add_with_immediate_overflow() {
    let mut bcpu = CPU::new();
    test_instruction!(bcpu, add, "r0", "32000");
    test_instruction!(bcpu, add, "r0", "32000");
    assert_eq!(bcpu.oflag, true);
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
#[should_panic]
fn add_with_register_fail() {
    let mut bcpu = CPU::new();
    test_instruction!(bcpu, add, "r3434", "r3");
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
#[should_panic]
fn add_with_rptr_fail() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 44;

    test_instruction!(bcpu, add, "r3", "&r0");
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
#[should_panic]
fn add_with_mptr_fail() {
    let mut bcpu = CPU::new();

    bcpu.memory[44] = Some(55);

    test_instruction!(bcpu, add, "r0", "&$44");
}

#[test]
fn add_with_negative_register() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = -42;

    bcpu.uint_reg[1] = 42;

    test_instruction!(bcpu, add, "r5", "r0");
    assert_eq!(bcpu.uint_reg[1], 0);
}

#[test]
fn add_with_negative_register_underflow() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = -42;

    test_instruction!(bcpu, add, "r5", "r0");
    assert_eq!(bcpu.uint_reg[1], 65494);
}

#[test]
fn hlt() {
    let mut bcpu = CPU::new();

    bcpu.running = true;

    test_instruction!(bcpu, hlt);
    assert_eq!(bcpu.running, false);
}

#[test]
fn jo_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, add, "r0", "4444");
    test_instruction!(bcpu, mul, "r0", "r0");
    assert_eq!(bcpu.oflag, true);
    test_instruction!(bcpu, jo, "$300");
    assert_eq!(bcpu.pc, 300);

    bcpu.uint_reg[0] = 444;
    test_instruction!(bcpu, jo, "&r4");
    assert_eq!(bcpu.pc, 444);
}

#[test]
fn jo_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jo, "$300");
    assert_eq!(bcpu.pc, 1);
}

#[test]
fn jz_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, cmp, "r0", "r0");
    assert_eq!(bcpu.zflag, true);

    test_instruction!(bcpu, jz, "$300");
    assert_eq!(bcpu.pc, 300);

    bcpu.uint_reg[0] = 444;
    test_instruction!(bcpu, jz, "&r4");
    assert_eq!(bcpu.pc, 444);
}

#[test]
fn jz_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jz, "$120");
    assert_eq!(bcpu.pc, 1);
}

#[test]
fn jmp_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jmp, "$320");
    assert_eq!(bcpu.pc, 320);
    bcpu.uint_reg[0] = 444;
    test_instruction!(bcpu, jmp, "&r4");
    assert_eq!(bcpu.pc, 444);
}

#[test]
#[should_panic]
fn jmp_fail() {
    let mut bcpu = CPU::new();
    bcpu.float_reg[0] = f32::MAX - 5.0;
    test_instruction!(bcpu, jmp, "&r6");
}

#[test]
#[should_panic]
fn pop_fail() {
    let mut bcpu = CPU::new();
    bcpu.sp = 40;
    bcpu.bp = 39;
    bcpu.pc = 50;
    test_instruction!(bcpu, pop, "r4");
}

#[test]
fn pop_success() {
    let mut bcpu = CPU::new();

    bcpu.sp = 44;

    bcpu.bp = 45;

    bcpu.memory[44] = Some(33);

    test_instruction!(bcpu, pop, "r2");

    assert_eq!(bcpu.int_reg[2], 33);
    assert_eq!(bcpu.sp, 45);
    assert_eq!(bcpu.bp, 45);
}

#[test]
#[should_panic]
fn push_fail() {
    let mut bcpu = CPU::new();

    bcpu.sp = 0;
    bcpu.bp = 0;

    test_instruction!(bcpu, push, "44");
}

#[test]
#[should_panic]
fn push_fail_2() {
    let mut bcpu = CPU::new();

    bcpu.sp = u16::MAX;

    test_instruction!(bcpu, push, "434");
}

#[test]
fn push_success() {
    let mut bcpu = CPU::new();

    bcpu.sp = 32;
    bcpu.bp = 33;

    test_instruction!(bcpu, push, "45");
    assert_eq!(bcpu.memory[31], Some(45));

    bcpu = CPU::new();

    bcpu.sp = 20;
    bcpu.bp = 20;

    test_instruction!(bcpu, push, "33");

    assert_eq!(bcpu.memory[19], Some(33));
}
