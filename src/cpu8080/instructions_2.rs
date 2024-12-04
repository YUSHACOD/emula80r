use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::{error, info};

impl Cpu {
    pub fn execute_instruction_2(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x20	-
            0x20 => {
                self.inst_pointer += 1;
            }

            // 0x21	LXI H,D16	3		H <- byte 3, L <- byte 2
            0x21 => {
                self.registers.l = self.memory[inst_pointer + 1];
                self.registers.h = self.memory[inst_pointer + 2];

                self.inst_pointer += 3;
            }

            // 0x22	SHLD adr	3		(adr) <-L; (adr+1)<-H
            0x22 => {
                let lower8 = self.memory[inst_pointer + 1];
                let upper8 = self.memory[inst_pointer + 2];

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.l;
                self.memory[(location + 1) as usize] = self.registers.h;

                self.inst_pointer += 3;
            }

            // 0x23	INX H	1		HL <- HL + 1
            0x23 => {
                let mut hl = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                hl += 1;

                self.registers.h = (hl >> 8) as u8;
                self.registers.l = (hl & 0xff) as u8;

                self.inst_pointer += 1;
            }

            // 0x24	INR H	1	Z, S, P, AC	H <- H+1
            0x24 => {
                self.registers.h = self.alu_operation(Operation::Add, self.registers.h, 0, true);

                self.inst_pointer += 1;
            }

            // 0x25	DCR H	1	Z, S, P, AC	H <- H-1
            0x25 => {
                self.registers.h = self.alu_operation(Operation::Sub, self.registers.h, 0, true);

                self.inst_pointer += 1;
            }

            // 0x26	MVI H,D8	2		H <- byte 2
            0x26 => {
                self.registers.h = self.memory[inst_pointer + 1];

                self.inst_pointer += 2
            }

            // 0x27	DAA	1	Z, S, P, AC, C  Decimal Adjust Accumulator
            0x27 => {
                info!("DAA started beware, of demons \n{}", self.get_dbg_string());
                if ((self.registers.a & 0x0f) > 0x09) || self.flags.aux_carry {
                    self.registers.a =
                        self.alu_operation(Operation::Add, self.registers.a, 0x06, false);
                }

                if ((self.registers.a >> 4) > 0x09) || self.flags.carry {
                    self.registers.a =
                        self.alu_operation(Operation::Add, self.registers.a, 0x60, false);
                }

                info!("DAA executed beware, of demons \n{}", self.get_dbg_string());

                self.inst_pointer += 1
            }

            // 0x28	-
            0x28 => {
                self.inst_pointer += 1;
            }

            // 0x29	DAD H	1	CY	HL = HL + HL
            0x29 => {
                let mut hl: u32 = ((self.registers.h as u32) << 8) | (self.registers.l as u32);

                hl = hl + hl;

                self.flags.carry = hl > 0xffff;
                self.registers.h = (hl >> 8) as u8;
                self.registers.l = hl as u8;

                self.inst_pointer += 1;
            }

            // 0x2a	LHLD adr	3		L <- (adr); H <- (adr+1)
            0x2a => {
                let lower8 = self.memory[inst_pointer + 1];
                let upper8 = self.memory[inst_pointer + 2];

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.registers.l = self.memory[location as usize];
                self.registers.h = self.memory[(location + 1) as usize];

                self.inst_pointer += 3;
            }

            // 0x2b	DCX H	1		HL = HL-1
            0x2b => {
                let mut hl: u32 = ((self.registers.h as u32) << 8) | (self.registers.l as u32);
                hl -= 1;

                self.registers.h = (hl >> 8) as u8;
                self.registers.l = hl as u8;

                self.inst_pointer += 1;
            }

            // 0x2c	INR L	1	Z, S, P, AC	L <- L+1
            0x2c => {
                self.registers.l = self.alu_operation(Operation::Add, self.registers.l, 0, true);

                self.inst_pointer += 1;
            }

            // 0x2d	DCR L	1	Z, S, P, AC	L <- L-1
            0x2d => {
                self.registers.l = self.alu_operation(Operation::Sub, self.registers.l, 0, true);

                self.inst_pointer += 1;
            }

            // 0x2e	MVI L, D8	2		L <- byte 2
            0x2e => {
                self.registers.l = self.memory[inst_pointer + 1];

                self.inst_pointer += 2;
            }

            // 0x2f	CMA	1		A <- !A
            0x2f => {
                self.registers.a = !self.registers.a;

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
