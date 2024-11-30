use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction_6(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x60 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x61 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x62 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x63 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x64 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x65 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x66 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x67 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x68 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x69 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
