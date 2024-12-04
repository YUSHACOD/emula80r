use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::error;

impl Cpu {
    pub fn execute_instruction_d(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xd0	RNC	1		if NCY, RET
            0xd0 => {
                if !self.flags.carry {
                    let lower8 = self.memory[self.stack_pointer as usize];
                    let upper8 = self.memory[self.stack_pointer as usize + 1];

                    self.stack_pointer += 2;

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xd1	POP D	1		E <- (sp); D <- (sp+1); sp <- sp+2
            0xd1 => {
                self.registers.e = self.memory[self.stack_pointer as usize];
                self.registers.d = self.memory[self.stack_pointer as usize + 1];

                self.stack_pointer += 2;

                self.inst_pointer += 1;
            }

            // 0xd2	JNC adr	3		if NCY, PC<-adr
            0xd2 => {
                if !self.flags.carry {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xd3	OUT D8	2		special
            0xd3 => {
                self.io_table[inst_pointer as usize + 1] = self.registers.a;

                self.inst_pointer += 2;
            }

            // 0xd4	CNC adr	3		if NCY, CALL adr
            0xd4 => {
                if !self.flags.carry {
                    let lower8 = self.inst_pointer as u8;
                    let upper8 = (self.inst_pointer >> 8) as u8;

                    self.memory[self.stack_pointer as usize - 1] = upper8;
                    self.memory[self.stack_pointer as usize - 2] = lower8;

                    self.stack_pointer -= 2;

                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];
                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);

                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xd5	PUSH D	1		(sp-2)<-E; (sp-1)<-D; sp <- sp - 2
            0xd5 => {
                self.memory[self.stack_pointer as usize - 2] = self.registers.e;
                self.memory[self.stack_pointer as usize - 1] = self.registers.d;

                self.stack_pointer -= 2;

                self.inst_pointer += 1;
            }

            // 0xd6	SUI D8	2	Z, S, P, CY, AC	A <- A - data
            0xd6 => {
                let value = self.memory[inst_pointer + 1];
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, value, false);

                self.inst_pointer += 2;
            }

            // 0xd7	RST 2	1		CALL $10
            0xd7 => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                self.inst_pointer = 0x0010;
            }

            // 0xd8	RC	1		if CY, RET
            0xd8 => {
                if self.flags.carry {
                    let lower8 = self.memory[self.stack_pointer as usize];
                    let upper8 = self.memory[self.stack_pointer as usize + 1];

                    self.stack_pointer += 2;

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xd9	-
            0xd9 => {
                self.inst_pointer += 1;
            }

            // 0xda	JC adr	3		if CY, PC<-adr
            0xda => {
                if self.flags.carry {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xdb	IN D8	2		special
            0xdb => {
                self.registers.a = self.io_table[inst_pointer as usize + 1];

                self.inst_pointer += 2;
            }

            // 0xdc	CC adr	3		if CY, CALL adr
            0xdc => {
                if self.flags.carry {
                    let lower8 = self.inst_pointer as u8;
                    let upper8 = (self.inst_pointer >> 8) as u8;

                    self.memory[self.stack_pointer as usize - 1] = upper8;
                    self.memory[self.stack_pointer as usize - 2] = lower8;

                    self.stack_pointer -= 2;

                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];
                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);

                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xdd	-
            0xdd => {
                self.inst_pointer += 1;
            }

            // 0xde	SBI D8	2	Z, S, P, CY, AC	A <- A - data - CY
            0xde => {
                let value = self.memory[inst_pointer + 1];
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, value, self.flags.carry);

                self.inst_pointer += 2;
            }

            // 0xdf	RST 3	1		CALL $18
            0xdf => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                self.inst_pointer = 0x0018;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
