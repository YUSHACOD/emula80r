use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_6(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x60	MOV H,B	1		H <- B
            0x60 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x61	MOV H,C	1		H <- C
            0x61 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x62	MOV H,D	1		H <- D
            0x62 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x63	MOV H,E	1		H <- E
            0x63 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x64	MOV H,H	1		H <- H
            0x64 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x65	MOV H,L	1		H <- L
            0x65 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x66	MOV H,M	1		H <- (HL)
            0x66 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x67	MOV H,A	1		H <- A
            0x67 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x68	MOV L,B	1		L <- B
            0x68 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x69	MOV L,C	1		L <- C
            0x69 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x6a	MOV L,D	1		L <- D
            0x6a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x6b	MOV L,E	1		L <- E
            0x6b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x6c	MOV L,H	1		L <- H
            0x6c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x6d	MOV L,L	1		L <- L
            0x6d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x6e	MOV L,M	1		L <- (HL)
            0x6e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x6f	MOV L,A	1		L <- A
            0x6f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
