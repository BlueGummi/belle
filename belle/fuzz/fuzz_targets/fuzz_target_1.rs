#![no_main]

use belle::CPU;
use libfuzzer_sys::fuzz_target;
extern crate belle;

fuzz_target!(|data: [i16; 2]| {
    if !data.is_empty() {
        let mut cpu = CPU::new();
        cpu.fuzz = true;
        for instruction in &data {
            cpu.ir = *instruction;
            let parsed_instruction = cpu.decode_instruction();
            let _ = cpu.execute_instruction(&parsed_instruction);
        }
    }
});
