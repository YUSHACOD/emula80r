use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_3(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x30 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x31 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x32 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x33 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x34 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x35 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x36 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x37 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x38 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x39 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
