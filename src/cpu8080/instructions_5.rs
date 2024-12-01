use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_5(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x50	MOV D,B	1		D <- B
            0x50 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x51	MOV D,C	1		D <- C
            0x51 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x52	MOV D,D	1		D <- D
            0x52 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x53	MOV D,E	1		D <- E
            0x53 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x54	MOV D,H	1		D <- H
            0x54 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x55	MOV D,L	1		D <- L
            0x55 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x56	MOV D,M	1		D <- (HL)
            0x56 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x57	MOV D,A	1		D <- A
            0x57 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x58	MOV E,B	1		E <- B
            0x58 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x59	MOV E,C	1		E <- C
            0x59 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x5a	MOV E,D	1		E <- D
            0x5a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x5b	MOV E,E	1		E <- E
            0x5b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x5c	MOV E,H	1		E <- H
            0x5c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x5d	MOV E,L	1		E <- L
            0x5d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x5e	MOV E,M	1		E <- (HL)
            0x5e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x5f	MOV E,A	1		E <- A
            0x5f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
