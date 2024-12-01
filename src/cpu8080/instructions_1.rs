use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_1(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x10	-
            0x10 => {
                self.inst_pointer += 1;
            }

            // 0x11	LXI D,D16	3		D <- byte 3, E <- byte 2
            0x11 => {
                self.registers.e = self.memory[inst_pointer + 1];
                self.registers.d = self.memory[inst_pointer + 2];

                self.inst_pointer += 3;
            }

            // 0x12	STAX D	1		(DE) <- A
            0x12 => {
                let de: u32 = ((self.registers.d as u32) << 8) | (self.registers.e as u32);

                self.memory[de as usize] = self.registers.a;

                self.inst_pointer += 1;
            }

            // 0x13	INX D	1		DE <- DE + 1
            0x13 => {
                let result = (((self.registers.d as u16) << 8) | (self.registers.e as u16)) + 1;

                self.registers.d = (result >> 8) as u8;
                self.registers.e = (result & 0xff) as u8;

                self.inst_pointer += 1;
            }

            // 0x14	INR D	1	Z, S, P, AC	D <- D+1
            0x14 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x15	DCR D	1	Z, S, P, AC	D <- D-1
            0x15 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x16	MVI D, D8	2		D <- byte 2
            0x16 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x17	RAL	1	CY	A = A << 1; bit 0 = prev CY; CY = prev bit 7
            0x17 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x18	-			
            0x18 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x19	DAD D	1	CY	HL = HL + DE
            0x19 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x1a	LDAX D	1		A <- (DE)
            0x1a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x1b	DCX D	1		DE = DE-1
            0x1b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x1c	INR E	1	Z, S, P, AC	E <-E+1
            0x1c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }


            // 0x1d	DCR E	1	Z, S, P, AC	E <- E-1
            0x1d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x1e	MVI E,D8	2		E <- byte 2
            0x1e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x1f	RAR	1	CY	A = A >> 1; bit 7 = prev bit 7; CY = prev bit 0
            0x1f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
