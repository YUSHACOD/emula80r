use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_c(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0xc0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xca => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xcb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xcc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xcd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xce => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xcf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
