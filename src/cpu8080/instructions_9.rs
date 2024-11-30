use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_9(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x90 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x91 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x92 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x93 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x94 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x95 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x96 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x97 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x98 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x99 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
