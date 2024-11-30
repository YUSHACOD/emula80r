use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_f(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0xf0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfa => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfe => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xff => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
