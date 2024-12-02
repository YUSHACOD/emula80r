use super::Cpu;
use log::{error, info};

impl Cpu {
    pub fn execute_instruction_7(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x70	MOV M,B	1		(HL) <- B
            0x70 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.b;

                self.inst_pointer += 1;
            }

            // 0x71	MOV M,C	1		(HL) <- C
            0x71 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.c;

                self.inst_pointer += 1;
            }

            // 0x72	MOV M,D	1		(HL) <- D
            0x72 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.d;

                self.inst_pointer += 1;
            }

            // 0x73	MOV M,E	1		(HL) <- E
            0x73 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.e;

                self.inst_pointer += 1;
            }

            // 0x74	MOV M,H	1		(HL) <- H
            0x74 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.h;

                self.inst_pointer += 1;
            }

            // 0x75	MOV M,L	1		(HL) <- L
            0x75 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.l;

                self.inst_pointer += 1;
            }

            // 0x76	HLT	1		special
            0x76 => {
                info!(
                    "this is halt need better handling,\n{}",
                    self.get_dbg_string()
                );
                self.enabled = false;
            }

            // 0x77	MOV M,A	1		(HL) <- A
            0x77 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.memory[location as usize] = self.registers.a;

                self.inst_pointer += 1;
            }

            // 0x78	MOV A,B	1		A <- B
            0x78 => {
                self.registers.a = self.registers.b;

                self.inst_pointer += 1;
            }

            // 0x79	MOV A,C	1		A <- C
            0x79 => {
                self.registers.a = self.registers.c;

                self.inst_pointer += 1;
            }

            // 0x7a	MOV A,D	1		A <- D
            0x7a => {
                self.registers.a = self.registers.d;

                self.inst_pointer += 1;
            }

            // 0x7b	MOV A,E	1		A <- E
            0x7b => {
                self.registers.a = self.registers.e;

                self.inst_pointer += 1;
            }

            // 0x7c	MOV A,H	1		A <- H
            0x7c => {
                self.registers.a = self.registers.h;

                self.inst_pointer += 1;
            }

            // 0x7d	MOV A,L	1		A <- L
            0x7d => {
                self.registers.a = self.registers.l;

                self.inst_pointer += 1;
            }

            // 0x7e	MOV A,M	1		A <- (HL)
            0x7e => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.registers.a = self.memory[location as usize];

                self.inst_pointer += 1;
            }

            // 0x7f	MOV A,A	1		A <- A
            0x7f => {
                self.registers.a = self.registers.a;

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
