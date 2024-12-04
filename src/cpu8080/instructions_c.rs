use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::error;

impl Cpu {
    pub fn execute_instruction_c(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xc0	RNZ	1		if NZ, RET
            // PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2
            0xc0 => {
                if !self.flags.zero {
                    let lower8 = self.memory[self.stack_pointer as usize];
                    let upper8 = self.memory[self.stack_pointer as usize + 1];

                    self.stack_pointer += 2;

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xc1	POP B	1		C <- (sp); B <- (sp+1); sp <- sp+2
            0xc1 => {
                self.registers.c = self.memory[self.stack_pointer as usize];
                self.registers.b = self.memory[self.stack_pointer as usize + 1];

                self.stack_pointer += 2;

                self.inst_pointer += 1;
            }

            // 0xc2	JNZ adr	3		if NZ, PC <- adr
            0xc2 => {
                if !self.flags.zero {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xc3	JMP adr	3		PC <= adr
            0xc3 => {
                let lower8 = self.memory[inst_pointer + 1];
                let upper8 = self.memory[inst_pointer + 2];

                self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
            }

            // 0xc4	CNZ adr	3		if NZ, CALL adr
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xc4 => {
                if !self.flags.zero {
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

            // 0xc5	PUSH B	1		(sp-2)<-C; (sp-1)<-B; sp <- sp - 2
            0xc5 => {
                self.memory[self.stack_pointer as usize - 2] = self.registers.c;
                self.memory[self.stack_pointer as usize - 1] = self.registers.b;

                self.stack_pointer -= 2;

                self.inst_pointer += 1;
            }

            // 0xc6	ADI D8	2	Z, S, P, CY, AC	A <- A + byte
            0xc6 => {
                let value = self.memory[inst_pointer + 1];
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, value, false);

                self.inst_pointer += 2;
            }

            // 0xc7	RST 0	1		CALL $0
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xc7 => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                self.inst_pointer = 0x0000;
            }

            // 0xc8	RZ	1		if Z, RET
            // PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2
            0xc8 => {
                if self.flags.zero {
                    let lower8 = self.memory[self.stack_pointer as usize];
                    let upper8 = self.memory[self.stack_pointer as usize + 1];

                    self.stack_pointer += 2;

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xc9	RET	1		PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2
            0xc9 => {
                let lower8 = self.memory[self.stack_pointer as usize];
                let upper8 = self.memory[self.stack_pointer as usize + 1];

                self.stack_pointer += 2;

                let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                self.inst_pointer = pointer;
            }

            // 0xca	JZ adr	3		if Z, PC <- adr
            0xca => {
                if self.flags.zero {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xcb	-
            0xcb => {
                self.inst_pointer += 1;
            }

            // 0xcc	CZ adr	3		if Z, CALL adr
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xcc => {
                if self.flags.zero {
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

            // 0xcd	CALL adr	3
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xcd => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                let lower8 = self.memory[inst_pointer + 1];
                let upper8 = self.memory[inst_pointer + 2];
                let pointer = ((upper8 as u16) << 8) | (lower8 as u16);

                self.inst_pointer = pointer;
            }

            // 0xce	ACI D8	2	Z, S, P, CY, AC	A <- A + data + CY
            0xce => {
                let value = self.memory[inst_pointer + 1];
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, value, self.flags.carry);

                self.inst_pointer += 2;
            }

            // 0xcf	RST 1	1		CALL $8
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xcf => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                self.inst_pointer = 0x0008;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
