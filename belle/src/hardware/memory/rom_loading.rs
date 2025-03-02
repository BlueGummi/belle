use crate::config::CONFIG;
use crate::*;
use colored::*;
impl CPU {
    pub fn load_rom(&mut self, binary: &[i16]) -> Result<(), EmuError> {
        let mut counter = 0;
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
        if let Some(val) = binary.get(1) {
            self.starts_at = *val as u16;
        }
        let start_ind = if let Some(val) = binary.get(2) {
            ((val / 2) + 3) as usize
        } else {
            3
        };
        for (index, element) in binary.iter().enumerate() {
            if index < 3 {
                continue;
            }
            if index < start_ind {
                rom_metadata = format!(
                    "{}{}",
                    rom_metadata,
                    char::from(((element & 0x7F00) >> 8) as u8)
                );
                rom_metadata = format!("{}{}", rom_metadata, char::from((element & 0x7F) as u8));
                continue;
            } else {
                break;
            }
        }
        for (index, element) in binary.iter().enumerate() {
            if index < start_ind {
                continue;
            }
            if counter + self.starts_at as usize >= MEMORY_SIZE {
                return Err(EmuError::MemoryOverflow());
            }
            self.memory[counter + self.starts_at as usize] = *element as u16;

            counter += 1;
        }
        if CONFIG.verbose {
            if !rom_metadata.is_empty() {
                let longest_length = rom_metadata
                    .lines()
                    .map(|line| line.len())
                    .max()
                    .unwrap_or(0);
                let longest_length = if longest_length > 12 {
                    longest_length
                } else {
                    12
                };
                let val = if longest_length % 2 == 0 { 5 } else { 4 };
                println!(
                    "╔{}╡ {} ╞{}╗",
                    "═".repeat((longest_length / 2) - 5),
                    "METADATA".bright_green(),
                    "═".repeat((longest_length / 2) - val)
                );
                for line in rom_metadata.lines() {
                    println!("║ {:width$} ║", line, width = longest_length);
                }
                println!("╚{}╝", "═".repeat(longest_length + 2));
            } else {
                println!("=====NO METADATA====");
            }
        }
        self.pc = self.starts_at;
        Ok(())
    }
}
