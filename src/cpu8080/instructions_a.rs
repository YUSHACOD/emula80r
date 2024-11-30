use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_a(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0xa0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xaa => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xab => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xac => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xad => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xae => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xaf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
