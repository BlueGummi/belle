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
fn jno_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, add, "r0", "4444");
    test_instruction!(bcpu, mul, "r0", "r0");
    assert_eq!(bcpu.oflag, true);
    test_instruction!(bcpu, jno, "$300");
    assert_eq!(bcpu.pc, 3);

    bcpu.uint_reg[0] = 444;
    test_instruction!(bcpu, jno, "&r4");
    assert_eq!(bcpu.pc, 4);
}

#[test]
fn jno_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jno, "$300");
    assert_eq!(bcpu.pc, 300);
}

#[test]
fn jnz_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, cmp, "r0", "r0");
    assert_eq!(bcpu.zflag, true);

    test_instruction!(bcpu, jnz, "$300");
    assert_eq!(bcpu.pc, 2);

    bcpu.uint_reg[0] = 444;
    test_instruction!(bcpu, jnz, "&r4");
    assert_eq!(bcpu.pc, 3);
}

#[test]
fn jnz_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jnz, "$120");
    assert_eq!(bcpu.pc, 120);
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
fn st_success() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 45;
    test_instruction!(bcpu, st, "$31", "r0");
    assert_eq!(bcpu.memory[31], Some(45));
}

#[test]
fn lea_success() {
    let mut bcpu = CPU::new();
    bcpu.memory[44] = Some(45);
    test_instruction!(bcpu, lea, "r0", "$44");
    assert_eq!(bcpu.int_reg[0], 44);
}

#[test]
fn ld_success() {
    let mut bcpu = CPU::new();
    bcpu.memory[44] = Some(45);
    test_instruction!(bcpu, ld, "r0", "$44");
    assert_eq!(bcpu.int_reg[0], 45);
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

#[test]
#[should_panic]
fn cmp_fail() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[2] = 333;

    test_instruction!(bcpu, cmp, "r0", "&r2");
}

#[test]
fn cmp_success() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[2] = 33;

    bcpu.float_reg[0] = 33.0;

    test_instruction!(bcpu, cmp, "r2", "r6");

    assert_eq!(bcpu.zflag, true);

    test_instruction!(bcpu, cmp, "r6", "r7");

    assert_eq!(bcpu.zflag, false);
}

#[test]
#[should_panic]
fn div_fail() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, div, "r0", "0");
}

#[test]
fn div_success() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 9;
    test_instruction!(bcpu, div, "r0", "3");

    assert_eq!(bcpu.int_reg[0], 3);
}

#[test]
#[should_panic]
fn mul_fail() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, mul, "r0", "&$33");
}

#[test]
#[should_panic]
fn mul_fail_2() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[1] = 23;
    test_instruction!(bcpu, mul, "r0", "&r1");
}

#[test]
fn mul_success() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[2] = 33;
    bcpu.int_reg[1] = -3;
    test_instruction!(bcpu, mul, "r2", "r1");

    assert_eq!(bcpu.int_reg[2], -99);

    test_instruction!(bcpu, mul, "r2", "2");

    assert_eq!(bcpu.int_reg[2], -198);
}

#[test]
#[should_panic]
fn mov_fail() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[1] = -32;

    test_instruction!(bcpu, mov, "r0", "&r1");
}

#[test]
#[should_panic]
fn mov_fail_2() {
    let mut bcpu = CPU::new();

    bcpu.uint_reg[0] = 333;

    test_instruction!(bcpu, mov, "r2", "&r4");
}

#[test]
fn mov_success() {
    let mut bcpu = CPU::new();

    bcpu.uint_reg[0] = 33;

    test_instruction!(bcpu, mov, "r0", "r4");

    assert_eq!(bcpu.int_reg[0], 33);

    bcpu.float_reg[0] = 6.9;

    test_instruction!(bcpu, mov, "r1", "r6");

    assert_eq!(bcpu.int_reg[1], 6);

    test_instruction!(bcpu, mov, "r6", "123");

    assert_eq!(bcpu.float_reg[0], 123.0);
}

#[test]
#[should_panic]
fn ret_fail() {
    let mut bcpu = CPU::new();

    bcpu.sp = 88;
    bcpu.bp = 123;

    test_instruction!(bcpu, ret);
}

#[test]
fn ret_success() {
    let mut bcpu = CPU::new();

    // setup a "Fake call stack"
    bcpu.sp = 88;
    bcpu.bp = 89;
    bcpu.memory[88] = Some(123);

    test_instruction!(bcpu, ret);

    assert_eq!(bcpu.sp, 89);

    assert_eq!(bcpu.sp, 89);

    assert_eq!(bcpu.pc, 124);
}

#[test]
fn int_fail() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, int, "90");
}

#[test]
fn int_success() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, int, "11");
    //assert_eq!(bcpu.zflag, true);

    test_instruction!(bcpu, int, "12");
    assert_eq!(bcpu.zflag, false);

    test_instruction!(bcpu, int, "13");
    assert_eq!(bcpu.zflag, true);

    // overflow flag here

    test_instruction!(bcpu, int, "21");
    assert_eq!(bcpu.oflag, true);

    test_instruction!(bcpu, int, "22");
    assert_eq!(bcpu.oflag, false);

    test_instruction!(bcpu, int, "23");
    assert_eq!(bcpu.oflag, true);

    // remainder flag

    test_instruction!(bcpu, int, "31");
    assert_eq!(bcpu.rflag, true);

    test_instruction!(bcpu, int, "32");
    assert_eq!(bcpu.rflag, false);

    test_instruction!(bcpu, int, "33");
    assert_eq!(bcpu.rflag, true);

    // sign flag

    test_instruction!(bcpu, int, "41");
    assert_eq!(bcpu.sflag, true);

    test_instruction!(bcpu, int, "42");
    assert_eq!(bcpu.sflag, false);

    test_instruction!(bcpu, int, "43");
    assert_eq!(bcpu.sflag, true);

    // HLT on overflow

    test_instruction!(bcpu, int, "51");
    assert_eq!(bcpu.hlt_on_overflow, true);

    test_instruction!(bcpu, int, "52");
    assert_eq!(bcpu.hlt_on_overflow, false);

    test_instruction!(bcpu, int, "53");
    assert_eq!(bcpu.hlt_on_overflow, true);

    bcpu.uint_reg[0] = 555;
    // SP
    test_instruction!(bcpu, int, "60");
    assert_eq!(bcpu.sp, 555);

    bcpu.uint_reg[0] = 6154;
    // BP
    test_instruction!(bcpu, int, "61");
    assert_eq!(bcpu.bp, 6154);
}

#[test]
fn jl_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 55;
    bcpu.int_reg[1] = 56;
    test_instruction!(bcpu, cmp, "r0", "r1");
    test_instruction!(bcpu, jl, "$120");
    assert_eq!(bcpu.pc, 120);
}

#[test]
fn jl_no_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 56;
    bcpu.int_reg[1] = 55;
    test_instruction!(bcpu, cmp, "r0", "r1");
    test_instruction!(bcpu, jl, "$120");
    assert_eq!(bcpu.pc, 2);
}

#[test]
fn jg_no_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 55;
    bcpu.int_reg[1] = 56;
    test_instruction!(bcpu, cmp, "r0", "r1");
    test_instruction!(bcpu, jg, "$120");
    assert_eq!(bcpu.pc, 2);
}

#[test]
fn jg_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 56;
    bcpu.int_reg[1] = 55;
    test_instruction!(bcpu, cmp, "r0", "r1");
    test_instruction!(bcpu, jg, "$120");
    assert_eq!(bcpu.pc, 120);
}

#[test]
fn jr_no_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 55;
    bcpu.int_reg[1] = 56;
    test_instruction!(bcpu, div, "r0", "r0");
    test_instruction!(bcpu, jr, "$120");
    assert_eq!(bcpu.pc, 2);
}

#[test]
fn jr_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 56;
    bcpu.int_reg[1] = 55;
    test_instruction!(bcpu, div, "r0", "r1");
    test_instruction!(bcpu, jr, "$120");
    assert_eq!(bcpu.pc, 120);
}
