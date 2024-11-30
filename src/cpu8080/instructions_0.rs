use super::utils::*;
use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction_0(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x00 => {
                self.inst_pointer += 1;
            }

            0x01 => {
                self.registers.b = self.memory[inst_pointer + 1];
                self.registers.c = self.memory[inst_pointer + 2];
                self.inst_pointer += 3;
            }

            0x02 => {
                let location = ((self.registers.b as u16) << 8) | (self.registers.c as u16);
                self.memory[location as usize] = self.registers.a;
                self.inst_pointer += 1;
            }

            0x03 => {
                let result = (((self.registers.b as u16) << 8) | (self.registers.c as u16)) + 1;
                self.registers.b = (result >> 8) as u8;
                self.registers.c = (result & 0xff) as u8;
                self.inst_pointer += 1;
            }

            0x04 => {
                self.registers.b = self.alu_operation(Operation::Add, self.registers.b, 0, true);
                self.inst_pointer += 1;
            }

            0x05 => {
                self.registers.b = self.alu_operation(Operation::Sub, self.registers.b, 0, true);
                self.inst_pointer += 1;
            }

            0x06 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x07 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x08 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x09 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
