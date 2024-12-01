use super::utils::*;
use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_0(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x00 => {
                self.inst_pointer += 1;
            }

            // 0x01	LXI B,D16	3		B <- byte 3, C <- byte 2
            0x01 => {
                self.registers.c = self.memory[inst_pointer + 1];
                self.registers.b = self.memory[inst_pointer + 2];

                self.inst_pointer += 3;
            }

            // 0x02	STAX B	1		(BC) <- A
            0x02 => {
                let location = ((self.registers.b as u16) << 8) | (self.registers.c as u16);

                self.memory[location as usize] = self.registers.a;

                self.inst_pointer += 1;
            }

            // 0x03	INX B	1		BC <- BC+1
            0x03 => {
                let result = (((self.registers.b as u16) << 8) | (self.registers.c as u16)) + 1;

                self.registers.b = (result >> 8) as u8;
                self.registers.c = (result & 0xff) as u8;

                self.inst_pointer += 1;
            }

            // 0x04	INR B	1	Z, S, P, AC	B <- B+1
            0x04 => {
                self.registers.b = self.alu_operation(Operation::Add, self.registers.b, 0, true);

                self.inst_pointer += 1;
            }

            // 0x05	DCR B	1	Z, S, P, AC	B <- B-1
            0x05 => {
                self.registers.b = self.alu_operation(Operation::Sub, self.registers.b, 0, true);

                self.inst_pointer += 1;
            }

            // 0x06	MVI B, D8	2		B <- byte 2
            0x06 => {
                self.registers.b = self.memory[inst_pointer + 1];

                self.inst_pointer += 2;
            }

            // 0x07	RLC	1	CY	A = A << 1; bit 0 = prev bit 7; CY = prev bit 7
            0x07 => {
                self.flags.carry = (self.registers.a & 0b10000000) == 0b10000000;

                let msb = if self.flags.carry { 1 } else { 0 };
                self.registers.a = (self.registers.a << 1) + msb;

                self.inst_pointer += 1;
            }

            // 0x08	-
            0x08 => {
                self.inst_pointer += 1;
            }

            // 0x09	DAD B	1	CY	HL = HL + BC
            0x09 => {
                let mut hl: u32 = ((self.registers.h as u32) << 8) | (self.registers.l as u32);
                let bc: u32 = ((self.registers.b as u32) << 8) | (self.registers.c as u32);

                hl = hl + bc;

                self.flags.carry = hl > 0xffff;
                self.registers.h = (hl >> 8) as u8;
                self.registers.l = hl as u8;

                self.inst_pointer += 1;
            }

            // 0x0a	LDAX B	1		A <- (BC)
            0x0a => {
                let bc: u32 = ((self.registers.b as u32) << 8) | (self.registers.c as u32);

                self.registers.a = self.memory[bc as usize];

                self.inst_pointer += 1;
            }

            // 0x0b	DCX B	1		BC = BC-1
            0x0b => {
                let mut bc: u32 = ((self.registers.b as u32) << 8) | (self.registers.c as u32);
                bc -= 1;

                self.registers.b = (bc >> 8) as u8;
                self.registers.c = bc as u8;

                self.inst_pointer += 1;
            }

            // 0x0c	INR C	1	Z, S, P, AC	C <- C+1
            0x0c => {
                self.registers.c = self.alu_operation(Operation::Add, self.registers.c, 0, true);

                self.inst_pointer += 1;
            }

            // 0x0d	DCR C	1	Z, S, P, AC	C <-C-1
            0x0d => {
                self.registers.c = self.alu_operation(Operation::Sub, self.registers.c, 0, true);

                self.inst_pointer += 1;
            }

            // 0x0e	MVI C,D8	2		C <- byte 2
            0x0e => {
                self.registers.c = self.memory[inst_pointer + 1];

                self.inst_pointer += 2;
            }

            // 0x0f	RRC	1	CY	A = A >> 1; bit 7 = prev bit 0; CY = prev bit 0
            0x0f => {
                self.flags.carry = (self.registers.a & 0b00000001) == 0b00000001;

                let lsb = if self.flags.carry { 1 } else { 0 };
                self.registers.a = (self.registers.a >> 1) + (lsb << 7);

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
