use crate::ioutils;
use log::info;
use std::io;

use super::CPU;

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
}

impl CPU {
    /// Update Flags with arithmetic operation
    pub fn alu_operation(
        &mut self,
        operation: Operation,
        operand1: u8,
        operand2: u8,
        carry_in: bool,
    ) -> u8 {
        let carry_in_value = if carry_in { 1 } else { 0 };

        let (result, full_result): (u8, u16) = match operation {
            Operation::Add => {
                let full = (operand1 as u16) + (operand2 as u16) + (carry_in_value as u16);
                (full as u8, full)
            }
            Operation::Sub => {
                let full = (operand1 as u16)
                    .wrapping_sub(operand2 as u16)
                    .wrapping_sub(carry_in_value as u16);
                (full as u8, full)
            }
        };

        self.flags.zero = result == 0;

        self.flags.sign = result & 0x80 != 0;

        self.flags.parity = result.count_ones() % 2 == 0;

        self.flags.carry = full_result > 0xFF;

        self.flags.aux_carry = match operation {
            Operation::Add => ((operand1 & 0x0F) + (operand2 & 0x0F) + carry_in_value) > 0x0F,
            Operation::Sub => {
                let borrow = (operand2 & 0x0F) + carry_in_value;
                (operand1 & 0x0F) < borrow
            }
        };

        info!("Flags updated: zero: [{}], sign: [{}], parity: [{}], carry: [{}], aux_carry: [{}]",
              self.flags.zero, self.flags.sign, self.flags.parity, self.flags.carry, self.flags.aux_carry);

        result
    }


    pub fn load_program(&mut self, file_name: &str, address: u8) -> io::Result<()> {
        let mut bytes: Vec<u8> = Vec::new();
        ioutils::read_file(file_name.to_string(), bytes.as_mut())?;

        for i in 0..bytes.len() {
            self.memory[i + address as usize] = bytes[i];
        }
        info!("Loaded Program");

        Ok(())
    }
}
