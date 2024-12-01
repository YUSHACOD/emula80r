use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_4(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x40	MOV B,B	1		B <- B
            0x40 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x41	MOV B,C	1		B <- C
            0x41 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x42	MOV B,D	1		B <- D
            0x42 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x43	MOV B,E	1		B <- E
            0x43 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x44	MOV B,H	1		B <- H
            0x44 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x45	MOV B,L	1		B <- L
            0x45 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x46	MOV B,M	1		B <- (HL)
            0x46 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x47	MOV B,A	1		B <- A
            0x47 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x48	MOV C,B	1		C <- B
            0x48 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x49	MOV C,C	1		C <- C
            0x49 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x4a	MOV C,D	1		C <- D
            0x4a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x4b	MOV C,E	1		C <- E
            0x4b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x4c	MOV C,H	1		C <- H
            0x4c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x4d	MOV C,L	1		C <- L
            0x4d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x4e	MOV C,M	1		C <- (HL)
            0x4e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x4f	MOV C,A	1		C <- A
            0x4f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
