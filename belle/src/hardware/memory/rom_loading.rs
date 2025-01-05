use crate::*;
impl CPU {
    pub fn load_rom(&mut self, binary: &Vec<i16>) -> Result<(), EmuError> {
        let mut counter = 0;
        let mut start_found = false;

        for element in binary {
            if (element >> 9) == 1 {
                // start directive
                if start_found {
                    self.do_not_run = true;
                    return Err(EmuError::Duplicate(".start directives".to_string()));
                }
                self.starts_at = (element & 0b111111111) as u16;
                start_found = true;
                continue;
            } else if (element >> 9) == 2 {
                self.sp = (element & 0b111111111) as u16;
                continue;
            } else if (element >> 9) == 3 {
                self.bp = (element & 0b111111111) as u16;
                continue;
            }
            if counter + self.starts_at as usize >= MEMORY_SIZE {
                return Err(EmuError::MemoryOverflow());
            }
            self.memory[counter + self.starts_at as usize] = Some(*element as u16);

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
        let some_count = self.memory.iter().filter(|&&e| e.is_some()).count();

        if some_count as u32 + u32::from(self.starts_at) > MEMORY_SIZE.try_into().unwrap() {
            eprintln!("{}", EmuError::MemoryOverflow());
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
    }
}
