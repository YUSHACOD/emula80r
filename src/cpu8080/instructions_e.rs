use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction_e(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0xe0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xea => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xeb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xec => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xed => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xee => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xef => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
