use crate::config::CONFIG;
use crate::*;
use colored::*;
impl CPU {
    pub fn load_rom(&mut self, binary: &Vec<i16>) -> Result<(), EmuError> {
        let mut counter = 0;
        let mut start_found = false;
        let mut rom_metadata = String::from("");
        if let Some(number) = binary.first() {
            if (number >> 8) == 1 {
                if number & 0xFF != 2 {
                    eprintln!(
                        "{}: {} '{}' version does not match emulator version.",
                        "warning".yellow(),
                        "ROM".magenta(),
                        CONFIG.rom.to_string().green()
                    );
                }
            } else {
                eprintln!(
                    "{}: {} '{}' does not have version.\nmay be invalid",
                    "warning".yellow(),
                    "ROM".magenta(),
                    CONFIG.rom.to_string().green()
                );
            }
        }
        for element in binary {
            match element >> 9 {
                1 => {
                    if start_found {
                        self.do_not_run = true;
                        return Err(EmuError::Duplicate(".start directives".to_string()));
                    }
                    self.starts_at = (element & 0b1_1111_1111) as u16;
                    start_found = true;
                    continue;
                }
                0b10 => {
                    self.sp = (element & 0b1_1111_1111) as u16;
                    continue;
                }
                0b11 => {
                    self.bp = (element & 0b1_1111_1111) as u16;
                    continue;
                }
                _ => {
                    if (element >> 8) == 1 {
                        rom_metadata =
                            format!("{}{}", rom_metadata, char::from((element & 0x7F) as u8));
                        continue;
                    }
                }
            }
            if counter + self.starts_at as usize >= MEMORY_SIZE {
                return Err(EmuError::MemoryOverflow());
            }
            self.memory[counter + self.starts_at as usize] = Some(*element as u16);

            counter += 1;
        }
        if CONFIG.metadata {
            println!("======METADATA======");
            println!("{rom_metadata}");
            println!("====END METADATA====");
        }
        self.shift_memory()?;
        self.pc = self.starts_at;
        Ok(())
    }

    fn shift_memory(&mut self) -> Result<(), EmuError> {
        if let Some(first_val) = self.memory.iter().position(|&e| e.is_some()) {
            if self.pc == first_val as u16 {
                return Ok(());
            }
        }
        let some_count = self.memory.iter().filter(|&&e| e.is_some()).count();

        if some_count as u32 + u32::from(self.starts_at) > MEMORY_SIZE.try_into().unwrap() {
            return Err(EmuError::MemoryOverflow());
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
        Ok(())
    }
}
