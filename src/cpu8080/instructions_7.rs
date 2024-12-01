use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_7(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x70	MOV M,B	1		(HL) <- B
            0x70 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x71	MOV M,C	1		(HL) <- C
            0x71 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x72	MOV M,D	1		(HL) <- D
            0x72 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x73	MOV M,E	1		(HL) <- E
            0x73 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x74	MOV M,H	1		(HL) <- H
            0x74 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x75	MOV M,L	1		(HL) <- L
            0x75 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x76	HLT	1		special
            0x76 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x77	MOV M,A	1		(HL) <- A
            0x77 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x78	MOV A,B	1		A <- B
            0x78 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x79	MOV A,C	1		A <- C
            0x79 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x7a	MOV A,D	1		A <- D
            0x7a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x7b	MOV A,E	1		A <- E
            0x7b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x7c	MOV A,H	1		A <- H
            0x7c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x7d	MOV A,L	1		A <- L
            0x7d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x7e	MOV A,M	1		A <- (HL)
            0x7e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x7f	MOV A,A	1		A <- A
            0x7f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
