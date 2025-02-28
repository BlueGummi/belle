pub mod core;
pub use core::*;
pub mod error_generation;
pub mod instruction_handling;
pub mod printing;
pub use instruction_handling::*;
pub mod memory;
pub use memory::*;
pub mod decoder;
#[allow(unused_imports)]
use crate::set_register;
pub use decoder::*;
pub use error_generation::*;

#[test]
fn set_register_0() {
    let mut bcpu = CPU::new();
    set_register!(bcpu, 0, 3.0);
    assert_eq!(bcpu.int_reg[0], 3);
    set_register!(bcpu, 0, -4.9);
    assert_eq!(bcpu.int_reg[0], 3);
}
