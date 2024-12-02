use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_5(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x50	MOV D,B	1		D <- B
            0x50 => {
                self.registers.d = self.registers.b;

                self.inst_pointer += 1;
            }

            // 0x51	MOV D,C	1		D <- C
            0x51 => {
                self.registers.d = self.registers.c;

                self.inst_pointer += 1;
            }

            // 0x52	MOV D,D	1		D <- D
            0x52 => {
                self.registers.d = self.registers.d;

                self.inst_pointer += 1;
            }

            // 0x53	MOV D,E	1		D <- E
            0x53 => {
                self.registers.d = self.registers.e;

                self.inst_pointer += 1;
            }

            // 0x54	MOV D,H	1		D <- H
            0x54 => {
                self.registers.d = self.registers.h;

                self.inst_pointer += 1;
            }

            // 0x55	MOV D,L	1		D <- L
            0x55 => {
                self.registers.d = self.registers.l;

                self.inst_pointer += 1;
            }

            // 0x56	MOV D,M	1		D <- (HL)
            0x56 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.registers.d = self.memory[location as usize];

                self.inst_pointer += 1;
            }

            // 0x57	MOV D,A	1		D <- A
            0x57 => {
                self.registers.d = self.registers.a;

                self.inst_pointer += 1;
            }

            // 0x58	MOV E,B	1		E <- B
            0x58 => {
                self.registers.e = self.registers.b;

                self.inst_pointer += 1;
            }

            // 0x59	MOV E,C	1		E <- C
            0x59 => {
                self.registers.e = self.registers.c;

                self.inst_pointer += 1;
            }

            // 0x5a	MOV E,D	1		E <- D
            0x5a => {
                self.registers.e = self.registers.d;

                self.inst_pointer += 1;
            }

            // 0x5b	MOV E,E	1		E <- E
            0x5b => {
                self.registers.e = self.registers.e;

                self.inst_pointer += 1;
            }

            // 0x5c	MOV E,H	1		E <- H
            0x5c => {
                self.registers.e = self.registers.h;

                self.inst_pointer += 1;
            }

            // 0x5d	MOV E,L	1		E <- L
            0x5d => {
                self.registers.e = self.registers.l;

                self.inst_pointer += 1;
            }

            // 0x5e	MOV E,M	1		E <- (HL)
            0x5e => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);

                self.registers.e = self.memory[location as usize];

                self.inst_pointer += 1;
            }

            // 0x5f	MOV E,A	1		E <- A
            0x5f => {
                self.registers.e = self.registers.a;

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
