use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_6(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x60	MOV H,B	1		H <- B
            0x60 => {
                self.registers.h = self.registers.b;

                self.inst_pointer += 1;
            }

            // 0x61	MOV H,C	1		H <- C
            0x61 => {
                self.registers.h = self.registers.c;

                self.inst_pointer += 1;
            }

            // 0x62	MOV H,D	1		H <- D
            0x62 => {
                self.registers.h = self.registers.d;

                self.inst_pointer += 1;
            }

            // 0x63	MOV H,E	1		H <- E
            0x63 => {
                self.registers.h = self.registers.e;

                self.inst_pointer += 1;
            }

            // 0x64	MOV H,H	1		H <- H
            0x64 => {
                self.registers.h = self.registers.h;

                self.inst_pointer += 1;
            }

            // 0x65	MOV H,L	1		H <- L
            0x65 => {
                self.registers.h = self.registers.l;

                self.inst_pointer += 1;
            }

            // 0x66	MOV H,M	1		H <- (HL)
            0x66 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.registers.h = self.memory[location as usize];

                self.inst_pointer += 1;
            }

            // 0x67	MOV H,A	1		H <- A
            0x67 => {
                self.registers.h = self.registers.a;

                self.inst_pointer += 1;
            }

            // 0x68	MOV L,B	1		L <- B
            0x68 => {
                self.registers.l = self.registers.b;

                self.inst_pointer += 1;
            }

            // 0x69	MOV L,C	1		L <- C
            0x69 => {
                self.registers.l = self.registers.c;

                self.inst_pointer += 1;
            }

            // 0x6a	MOV L,D	1		L <- D
            0x6a => {
                self.registers.l = self.registers.d;

                self.inst_pointer += 1;
            }

            // 0x6b	MOV L,E	1		L <- E
            0x6b => {
                self.registers.l = self.registers.e;

                self.inst_pointer += 1;
            }

            // 0x6c	MOV L,H	1		L <- H
            0x6c => {
                self.registers.l = self.registers.h;

                self.inst_pointer += 1;
            }

            // 0x6d	MOV L,L	1		L <- L
            0x6d => {
                self.registers.l = self.registers.l;

                self.inst_pointer += 1;
            }

            // 0x6e	MOV L,M	1		L <- (HL)
            0x6e => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.registers.l = self.memory[location as usize];

                self.inst_pointer += 1;
            }

            // 0x6f	MOV L,A	1		L <- A
            0x6f => {
                self.registers.l = self.registers.a;

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
