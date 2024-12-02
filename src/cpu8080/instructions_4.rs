use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_4(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x40	MOV B,B	1		B <- B
            0x40 => {
                self.registers.b = self.registers.b;

                self.inst_pointer += 1;
            }

            // 0x41	MOV B,C	1		B <- C
            0x41 => {
                self.registers.b = self.registers.c;

                self.inst_pointer += 1;
            }

            // 0x42	MOV B,D	1		B <- D
            0x42 => {
                self.registers.b = self.registers.d;

                self.inst_pointer += 1;
            }

            // 0x43	MOV B,E	1		B <- E
            0x43 => {
                self.registers.b = self.registers.e;

                self.inst_pointer += 1;
            }

            // 0x44	MOV B,H	1		B <- H
            0x44 => {
                self.registers.b = self.registers.h;

                self.inst_pointer += 1;
            }

            // 0x45	MOV B,L	1		B <- L
            0x45 => {
                self.registers.b = self.registers.l;

                self.inst_pointer += 1;
            }

            // 0x46	MOV B,M	1		B <- (HL)
            0x46 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.registers.b = self.memory[location as usize];

                self.inst_pointer += 1;
            }

            // 0x47	MOV B,A	1		B <- A
            0x47 => {
                self.registers.b = self.registers.a;

                self.inst_pointer += 1;
            }

            // 0x48	MOV C,B	1		C <- B
            0x48 => {
                self.registers.c = self.registers.b;

                self.inst_pointer += 1;
            }

            // 0x49	MOV C,C	1		C <- C
            0x49 => {
                self.registers.c = self.registers.c;

                self.inst_pointer += 1;
            }

            // 0x4a	MOV C,D	1		C <- D
            0x4a => {
                self.registers.c = self.registers.d;

                self.inst_pointer += 1;
            }

            // 0x4b	MOV C,E	1		C <- E
            0x4b => {
                self.registers.c = self.registers.e;

                self.inst_pointer += 1;
            }

            // 0x4c	MOV C,H	1		C <- H
            0x4c => {
                self.registers.c = self.registers.h;

                self.inst_pointer += 1;
            }

            // 0x4d	MOV C,L	1		C <- L
            0x4d => {
                self.registers.c = self.registers.l;

                self.inst_pointer += 1;
            }

            // 0x4e	MOV C,M	1		C <- (HL)
            0x4e => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.registers.c = self.memory[location as usize];

                self.inst_pointer += 1;
            }

            // 0x4f	MOV C,A	1		C <- A
            0x4f => {
                self.registers.c = self.registers.a;

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
