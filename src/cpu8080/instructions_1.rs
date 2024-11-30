use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_1(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x10 => {
                self.inst_pointer += 1
            }

            0x11 => {
                self.registers.d = self.memory[inst_pointer + 1]; 
                self.registers.e = self.memory[inst_pointer + 2]; 
                self.inst_pointer += 3;
            }

            0x12 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x13 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x14 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x15 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x16 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x17 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x18 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x19 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
