use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_5(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x50 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x51 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x52 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x53 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x54 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x55 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x56 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x57 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x58 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x59 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
