use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction_8(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x80 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x81 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x82 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x83 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x84 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x85 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x86 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x87 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x88 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x89 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
