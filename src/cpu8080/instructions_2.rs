use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction_2(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x20 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x21 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x22 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x23 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x24 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x25 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x26 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x27 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x28 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x29 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
