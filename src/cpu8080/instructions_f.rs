use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::error;

impl Cpu {
    pub fn execute_instruction_f(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xf0	RP	1		if P, RET
            0xf0 => {
                if !self.flags.sign {
                    let lower8 = self.memory[self.stack_pointer as usize];
                    let upper8 = self.memory[self.stack_pointer as usize + 1];

                    self.stack_pointer += 2;

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xf1	POP PSW	1		flags <- (sp); A <- (sp+1); sp <- sp+2
            0xf1 => {
                self.registers.a = self.memory[self.stack_pointer as usize + 1];
                let value = self.memory[self.stack_pointer as usize];

                self.flags.zero = (value & 0b10000000) == 0b10000000;
                self.flags.sign = (value & 0b01000000) == 0b01000000;
                self.flags.parity = (value & 0b00100000) == 0b00100000;
                self.flags.carry = (value & 0b00010000) == 0b00010000;
                self.flags.aux_carry = (value & 0b00001000) == 0b00001000;

                self.inst_pointer += 1;
            }

            // 0xf2	JP adr	3		if P=1 PC <- adr
            0xf2 => {
                if !self.flags.sign {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xf3	DI	1		special
            0xf3 => {
                self.interupts_enabled = false;

                self.inst_pointer += 1;
            }

            // 0xf4	CP adr	3		if P, PC <- adr
            0xf4 => {
                if !self.flags.sign {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);

                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xf5	PUSH PSW	1		(sp-2)<-flags; (sp-1)<-A; sp <- sp - 2
            0xf5 => {
                let mut value: u8 = 0;

                if self.flags.zero {
                    value |= 0b10000000;
                }
                if self.flags.sign {
                    value |= 0b01000000;
                }
                if self.flags.parity {
                    value |= 0b00100000;
                }
                if self.flags.carry {
                    value |= 0b00010000;
                }
                if self.flags.aux_carry {
                    value |= 0b00001000;
                }

                self.memory[self.stack_pointer as usize - 1] = self.registers.a;
                self.memory[self.stack_pointer as usize - 2] = value;

                self.stack_pointer -= 2;

                self.inst_pointer += 1;
            }

            // 0xf6	ORI D8	2	Z, S, P, CY, AC	A <- A | data
            0xf6 => {
                let value = self.memory[inst_pointer + 1];
                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, value, false);

                self.inst_pointer += 2;
            }

            // 0xf7	RST 6	1		CALL $30
            0xf7 => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                self.inst_pointer = 0x0030;
            }

            // 0xf8	RM	1		if M, RET
            0xf8 => {
                if self.flags.sign {
                    let lower8 = self.memory[self.stack_pointer as usize];
                    let upper8 = self.memory[self.stack_pointer as usize + 1];

                    self.stack_pointer += 2;

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xf9	SPHL	1		SP=HL
            0xf9 => {
                self.stack_pointer = (self.registers.h as u16) << 8 | (self.registers.l as u16);

                self.inst_pointer += 1;
            }

            // 0xfa	JM adr	3		if M, PC <- adr
            0xfa => {
                if self.flags.sign {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xfb	EI	1		special
            0xfb => {
                self.interupts_enabled = true;

                self.inst_pointer += 1;
            }

            // 0xfc	CM adr	3		if M, CALL adr
            0xfc => {
                if self.flags.sign {
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

            // 0xfd	-
            0xfd => {
                self.inst_pointer += 1;
            }

            // 0xfe	CPI D8	2	Z, S, P, CY, AC	A - data
            0xfe => {
                let value = self.memory[inst_pointer + 1];
                self.registers.a =
                    self.alu_operation(Operation::Cmp, self.registers.a, value, false);

                self.inst_pointer += 2;
            }

            // 0xff	RST 7	1		CALL $38
            0xff => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                self.inst_pointer = 0x0038;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
