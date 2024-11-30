use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction_4(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x40 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x41 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x42 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x43 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x44 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x45 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x46 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x47 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x48 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x49 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
