pub mod core;
pub use core::*;
pub mod error_generation;

#[allow(unused_imports)]
use crate::set_register;
#[test]
fn set_register() {
    let mut bcpu = CPU::new();
    set_register!(bcpu, 2, 3.0);
    assert_eq!(bcpu.int_reg[2], 3);
}
