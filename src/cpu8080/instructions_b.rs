use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction_b(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0xb0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xba => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbe => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
