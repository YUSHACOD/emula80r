use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_7(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x70 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x71 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x72 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x73 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x74 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x75 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x76 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x77 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x78 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x79 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
