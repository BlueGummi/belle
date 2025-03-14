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

    bcpu.memory[33] = 42;

    test_instruction!(bcpu, add, "r0", "&r2");
    assert_eq!(bcpu.int_reg[0], 63);
}

#[test]
fn add_with_rptr_fail() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 44;

    test_instruction!(bcpu, add, "r3", "&r0");
}

#[test]
fn add_with_mptr() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 444;

    bcpu.memory[33] = 42;

    bcpu.memory[42] = 60;

    test_instruction!(bcpu, add, "r0", "&$33");
    assert_eq!(bcpu.int_reg[0], 504);
}

#[test]
fn add_with_mptr_fail() {
    let mut bcpu = CPU::new();

    bcpu.memory[44] = 55;

    test_instruction!(bcpu, add, "r0", "&$44");
}

#[test]
fn add_with_negative_register() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 65493;

    bcpu.int_reg[5] = 42;

    test_instruction!(bcpu, add, "r5", "r0");
    assert_eq!(bcpu.int_reg[5], 65535);
}

#[test]
fn add_with_negative_register_underflow() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 65494;

    test_instruction!(bcpu, add, "r5", "r0");
    assert_eq!(bcpu.int_reg[5], 65494);
}

#[test]
fn hlt() {
    let mut bcpu = CPU::new();

    bcpu.running = true;

    test_instruction!(bcpu, hlt);
    assert_eq!(bcpu.running, false);
}

#[test]
fn bo_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, add, "r0", "4444");
    bcpu.oflag = true;
    assert_eq!(bcpu.oflag, true);
    test_instruction!(bcpu, bo, "$300");
    assert_eq!(bcpu.pc, 300);

    bcpu.int_reg[4] = 444;
    test_instruction!(bcpu, bo, "&r4");
    assert_eq!(bcpu.pc, 444);
}

#[test]
fn bo_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, bo, "$300");
    assert_eq!(bcpu.pc, 1);
}

#[test]
fn bno_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, add, "r0", "4444");
    bcpu.oflag = true;
    assert_eq!(bcpu.oflag, true);
    test_instruction!(bcpu, bno, "$300");
    assert_eq!(bcpu.pc, 2);

    bcpu.int_reg[4] = 444;
    test_instruction!(bcpu, bno, "&r4");
    assert_eq!(bcpu.pc, 3);
}

#[test]
fn bno_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, bno, "$300");
    assert_eq!(bcpu.pc, 300);
}

#[test]
fn bnz_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, cmp, "r0", "r0");
    assert_eq!(bcpu.zflag, true);

    test_instruction!(bcpu, bnz, "$300");
    assert_eq!(bcpu.pc, 2);

    bcpu.int_reg[4] = 444;
    test_instruction!(bcpu, bnz, "&r4");
    assert_eq!(bcpu.pc, 3);
}

#[test]
fn bnz_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, bnz, "$120");
    assert_eq!(bcpu.pc, 120);
}

#[test]
fn bz_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, cmp, "r0", "r0");
    assert_eq!(bcpu.zflag, true);

    test_instruction!(bcpu, bz, "$300");
    assert_eq!(bcpu.pc, 300);

    bcpu.int_reg[4] = 444;
    test_instruction!(bcpu, bz, "&r4");
    assert_eq!(bcpu.pc, 444);
}

#[test]
fn bz_no_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, bz, "$120");
    assert_eq!(bcpu.pc, 1);
}

#[test]
fn jmp_jump() {
    let mut bcpu = CPU::new();

    test_instruction!(bcpu, jmp, "$320");
    assert_eq!(bcpu.pc, 320);
    bcpu.int_reg[4] = 444;
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

    bcpu.memory[44] = 33;

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
    assert_eq!(bcpu.memory[31], (45));
}

#[test]
fn lea_success() {
    let mut bcpu = CPU::new();
    bcpu.memory[44] = 45;
    test_instruction!(bcpu, lea, "r0", "$44");
    assert_eq!(bcpu.int_reg[0], 44);
}

#[test]
fn ld_success() {
    let mut bcpu = CPU::new();
    bcpu.memory[44] = 45;
    test_instruction!(bcpu, ld, "r0", "$44");
    assert_eq!(bcpu.int_reg[0], 45);
}

#[test]
fn push_success() {
    let mut bcpu = CPU::new();

    bcpu.sp = 32;
    bcpu.bp = 33;

    test_instruction!(bcpu, push, "45");
    assert_eq!(bcpu.memory[31], 45);

    bcpu = CPU::new();

    bcpu.sp = 20;
    bcpu.bp = 20;

    test_instruction!(bcpu, push, "33");

    assert_eq!(bcpu.memory[19], 33);
}

#[test]
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
fn mov_fail() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[1] = 65503;

    test_instruction!(bcpu, mov, "r0", "&r1");
}

#[test]
fn mov_fail_2() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[4] = 333;

    test_instruction!(bcpu, mov, "r2", "&r4");
}

#[test]
fn mov_success() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[4] = 33;

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
    bcpu.memory[88] = 123;

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

    bcpu.int_reg[4] = 555;
    // SP
    test_instruction!(bcpu, int, "60");
    assert_eq!(bcpu.sp, 555);

    bcpu.int_reg[4] = 6154;
    // BP
    test_instruction!(bcpu, int, "61");
    assert_eq!(bcpu.bp, 6154);
}

#[test]
fn bl_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 55;
    bcpu.int_reg[1] = 56;
    test_instruction!(bcpu, cmp, "r0", "r1");
    test_instruction!(bcpu, bl, "$120");
    assert_eq!(bcpu.pc, 120);
}

#[test]
fn bl_no_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 56;
    bcpu.int_reg[1] = 55;
    test_instruction!(bcpu, cmp, "r0", "r1");
    test_instruction!(bcpu, bl, "$120");
    assert_eq!(bcpu.pc, 2);
}

#[test]
fn bg_no_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 55;
    bcpu.int_reg[1] = 56;
    test_instruction!(bcpu, cmp, "r0", "r1");
    test_instruction!(bcpu, bg, "$120");
    assert_eq!(bcpu.pc, 2);
}

#[test]
fn bg_jump() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 56;
    bcpu.int_reg[1] = 55;
    test_instruction!(bcpu, cmp, "r0", "r1");
    test_instruction!(bcpu, bg, "$120");
    assert_eq!(bcpu.pc, 120);
}
