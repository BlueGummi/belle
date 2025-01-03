pub mod core;
pub use core::*;
pub mod error_generation;
pub mod printing;

#[allow(unused_imports)]
use crate::set_register;
pub use error_generation::*;
#[test]
fn set_register() {
    let mut bcpu = CPU::new();
    set_register!(bcpu, 2, 3.0);
    assert_eq!(bcpu.int_reg[2], 3);
}
