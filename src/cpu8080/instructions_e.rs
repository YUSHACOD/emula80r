use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::error;

impl Cpu {
    pub fn execute_instruction_e(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xe0	RPO	1		if PO, RET
            // PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2
            0xe0 => {
                if !self.flags.parity {
                    let lower8 = self.memory[self.stack_pointer as usize];
                    let upper8 = self.memory[self.stack_pointer as usize + 1];

                    self.stack_pointer += 2;

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xe1	POP H	1		L <- (sp); H <- (sp+1); sp <- sp+2
            0xe1 => {
                self.registers.l = self.memory[self.stack_pointer as usize];
                self.registers.h = self.memory[self.stack_pointer as usize + 1];

                self.stack_pointer += 2;

                self.inst_pointer += 1;
            }

            // 0xe2	JPO adr	3		if PO, PC <- adr
            0xe2 => {
                if !self.flags.parity {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xe3	XTHL	1		L <-> (SP); H <-> (SP+1)
            0xe3 => {
                let temp1 = self.memory[self.stack_pointer as usize];
                let temp2 = self.memory[self.stack_pointer as usize + 1];

                self.memory[self.stack_pointer as usize] = self.registers.l;
                self.memory[self.stack_pointer as usize + 1] = self.registers.h;

                self.registers.l = temp1;
                self.registers.h = temp2;

                self.inst_pointer += 1;
            }

            // 0xe4	CPO adr	3		if PO, CALL adr
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xe4 => {
                if !self.flags.parity {
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

            // 0xe5	PUSH H	1		(sp-2)<-L; (sp-1)<-H; sp <- sp - 2
            0xe5 => {
                self.memory[self.stack_pointer as usize - 2] = self.registers.l;
                self.memory[self.stack_pointer as usize - 1] = self.registers.h;

                self.stack_pointer -= 2;

                self.inst_pointer += 1;
            }

            // 0xe6	ANI D8	2	Z, S, P, CY, AC	A <- A & data
            0xe6 => {
                let value = self.memory[inst_pointer + 1];
                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, value, false);

                self.inst_pointer += 2;
            }

            // 0xe7	RST 4	1		CALL $20
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xe7 => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                self.inst_pointer = 0x0020;
            }

            // 0xe8	RPE	1		if PE, RET
            // PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2
            0xe8 => {
                if self.flags.parity {
                    let lower8 = self.memory[self.stack_pointer as usize];
                    let upper8 = self.memory[self.stack_pointer as usize + 1];

                    self.stack_pointer += 2;

                    let pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                    self.inst_pointer = pointer;
                } else {
                    self.inst_pointer += 1;
                }
            }

            // 0xe9	PCHL	1		PC.hi <- H; PC.lo <- L
            0xe9 => {
                self.inst_pointer = (self.registers.h as u16) << 8 | (self.registers.l as u16);
            }

            // 0xea	JPE adr	3		if PE, PC <- adr
            0xea => {
                if self.flags.parity {
                    let lower8 = self.memory[inst_pointer + 1];
                    let upper8 = self.memory[inst_pointer + 2];

                    self.inst_pointer = ((upper8 as u16) << 8) | (lower8 as u16);
                } else {
                    self.inst_pointer += 3;
                }
            }

            // 0xeb	XCHG	1		H <-> D; L <-> E
            0xeb => {
                let temp1 = self.registers.e;
                let temp2 = self.registers.h;

                self.registers.e = self.registers.l;
                self.registers.h = self.registers.h;

                self.registers.l = temp1;
                self.registers.h = temp2;

                self.inst_pointer += 1;
            }

            // 0xec	CPE adr	3		if PE, CALL adr
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xec => {
                if self.flags.parity {
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

            // 0xed	-
            0xed => {
                self.inst_pointer += 1;
            }

            // 0xee	XRI D8	2	Z, S, P, CY, AC	A <- A ^ data
            0xee => {
                let value = self.memory[inst_pointer + 1];
                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, value, self.flags.carry);

                self.inst_pointer += 2;
            }

            // 0xef	RST 5	1		CALL $28
            // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xef => {
                let lower8 = self.inst_pointer as u8;
                let upper8 = (self.inst_pointer >> 8) as u8;

                self.memory[self.stack_pointer as usize - 1] = upper8;
                self.memory[self.stack_pointer as usize - 2] = lower8;

                self.stack_pointer -= 2;

                self.inst_pointer = 0x0028;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
