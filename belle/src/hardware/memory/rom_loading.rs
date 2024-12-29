use crate::{config::CONFIG, *};
impl CPU {
    pub fn load_rom(&mut self, binary: &Vec<i16>) -> Result<(), EmuError> {
        let mut counter = 0;
        let mut start_found = false;

        for element in binary {
            if (element >> 9) == 1 {
                // start directive
                if start_found {
                    EmuError::Duplicate(".start directives".to_string()).err();
                    self.do_not_run = true;
                }
                self.starts_at = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".start directive found.");
                }
                start_found = true;
                if CONFIG.verbose {
                    println!("program starts at {}", self.starts_at);
                }
                continue;
            } else if (element >> 9) == 2 {
                self.sp = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".ssp directive found");
                }
                continue;
            } else if (element >> 9) == 3 {
                self.bp = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".sbp directive found");
                }
                continue;
            }
            if counter + self.starts_at as usize >= MEMORY_SIZE {
                return Err(EmuError::MemoryOverflow());
            }
            self.memory[counter + self.starts_at as usize] = Some(*element as u16);
            if CONFIG.verbose {
                println!("Element {element:016b} loaded into memory");
            }

            counter += 1;
        }
        self.shift_memory();
        self.pc = self.starts_at;
        Ok(())
    }

    fn shift_memory(&mut self) {
        if let Some(first_val) = self.memory.iter().position(|&e| e.is_some()) {
            if self.pc == first_val as u16 {
                return;
            }
        }

        if CONFIG.verbose {
            println!("Shifting memory...");
        }

        let some_count = self.memory.iter().filter(|&&e| e.is_some()).count();

        if some_count as u32 + u32::from(self.starts_at) > MEMORY_SIZE.try_into().unwrap() {
            EmuError::MemoryOverflow().err();
        }

        let mut new_memory = Box::new([None; MEMORY_SIZE]);

        let first_some_index = self.memory.iter().position(|&e| e.is_some()).unwrap_or(0);
        for (i, value) in self.memory.iter().enumerate() {
            if let Some(val) = value {
                let new_index = (self.starts_at + (i - first_some_index) as u16) as usize;
                new_memory[new_index] = Some(*val);
            }
        }

        std::mem::swap(&mut self.memory, &mut new_memory);
        self.pc = self.starts_at;

        if CONFIG.verbose {
            println!("Shift completed.");
        }
    }
}
