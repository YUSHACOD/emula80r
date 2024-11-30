use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction_1(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x10 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x11 => {
                error!("unimplemented instruction : {:02X}", instruction,);
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
