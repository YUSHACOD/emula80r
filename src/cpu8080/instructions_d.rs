use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_d(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0xd0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xda => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xdb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xdc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xdd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xde => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xdf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
