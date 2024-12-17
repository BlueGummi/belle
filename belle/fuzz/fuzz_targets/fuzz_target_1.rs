#![no_main]

use belle::CPU;
use libfuzzer_sys::fuzz_target;
extern crate belle;

fuzz_target!(|data: [i16; 512]| {
    if !data.is_empty() {
        let mut cpu = CPU::new();

        for instruction in &data {
            cpu.ir = *instruction;
            let parsed_instruction = cpu.parse_instruction();
            let _ = cpu.execute_instruction(&parsed_instruction);
        }
    }
});
