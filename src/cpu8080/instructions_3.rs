use std::usize;

use super::{utils::Operation, Cpu};
use log::error;

impl Cpu {
    pub fn execute_instruction_3(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x30	-
            0x30 => {
                self.inst_pointer += 1;
            }

            // 0x31	LXI SP, D16	3		SP.hi <- byte 3, SP.lo <- byte 2
            0x31 => {
                let d16 = (self.memory[inst_pointer + 2] as u16) << 8
                    | (self.memory[inst_pointer + 1] as u16);
                self.stack_pointer = d16;

                self.inst_pointer += 3;
            }

            // 0x32	STA adr	3		(adr) <- A
            0x32 => {
                let lower8 = self.memory[inst_pointer + 1];
                let upper8 = self.memory[inst_pointer + 2];

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.a;

                self.inst_pointer += 3;
            }

            // 0x33	INX SP	1		SP = SP + 1
            0x33 => {
                self.stack_pointer += 1;

                self.inst_pointer += 1;
            }

            // 0x34	INR M	1	Z, S, P, AC	(HL) <- (HL)+1
            0x34 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                let value = self.memory[location as usize];
                self.memory[location as usize] = self.alu_operation(Operation::Add, value, 0, true);

                self.inst_pointer += 1;
            }

            // 0x35	DCR M	1	Z, S, P, AC	(HL) <- (HL)-1
            0x35 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                let value = self.memory[location as usize];
                self.memory[location as usize] = self.alu_operation(Operation::Sub, value, 0, true);

                self.inst_pointer += 1;
            }

            // 0x36	MVI M,D8	2		(HL) <- byte 2
            0x36 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                self.memory[location as usize] = self.memory[inst_pointer + 1];

                self.inst_pointer += 2;
            }

            // 0x37	STC	1	CY	CY = 1
            0x37 => {
                self.flags.carry = true;

                self.inst_pointer += 1;
            }

            // 0x38	-
            0x38 => {
                self.inst_pointer += 1;
            }

            // 0x39	DAD SP	1	CY	HL = HL + SP
            0x39 => {
                let mut hl: u32 = ((self.registers.h as u32) << 8) | (self.registers.l as u32);

                hl = hl + self.stack_pointer as u32;

                self.flags.carry = hl > 0xffff;
                self.registers.h = (hl >> 8) as u8;
                self.registers.l = hl as u8;

                self.inst_pointer += 1;
            }

            // 0x3a	LDA adr	3		A <- (adr)
            0x3a => {
                let lower8 = self.memory[inst_pointer + 1];
                let upper8 = self.memory[inst_pointer + 2];

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                self.registers.a = self.memory[location as usize];

                self.inst_pointer += 3;
            }

            // 0x3b	DCX SP	1		SP = SP-1
            0x3b => {
                self.stack_pointer -= 1;

                self.inst_pointer += 1;
            }

            // 0x3c	INR A	1	Z, S, P, AC	A <- A+1
            0x3c => {
                self.registers.a = self.alu_operation(Operation::Add, self.registers.a, 0, true);

                self.inst_pointer += 1;
            }

            // 0x3d	DCR A	1	Z, S, P, AC	A <- A-1
            0x3d => {
                self.registers.a = self.alu_operation(Operation::Sub, self.registers.a, 0, true);

                self.inst_pointer += 1;
            }

            // 0x3e	MVI A,D8	2		A <- byte 2
            0x3e => {
                self.registers.a = self.memory[inst_pointer + 1];

                self.inst_pointer += 2;
            }

            // 0x3f	CMC	1	CY	CY=!CY
            0x3f => {
                self.flags.carry = !self.flags.carry;

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
