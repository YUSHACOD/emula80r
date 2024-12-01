use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_2(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x20	-
            0x20 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x21	LXI H,D16	3		H <- byte 3, L <- byte 2
            0x21 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x22	SHLD adr	3		(adr) <-L; (adr+1)<-H
            0x22 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x23	INX H	1		HL <- HL + 1
            0x23 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x24	INR H	1	Z, S, P, AC	H <- H+1
            0x24 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x25	DCR H	1	Z, S, P, AC	H <- H-1
            0x25 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x26	MVI H,D8	2		H <- byte 2
            0x26 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x27	DAA	1		special
            0x27 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x28	-
            0x28 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x29	DAD H	1	CY	HL = HL + HI
            0x29 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x2a	LHLD adr	3		L <- (adr); H<-(adr+1)
            0x2a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x2b	DCX H	1		HL = HL-1
            0x2b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x2c	INR L	1	Z, S, P, AC	L <- L+1
            0x2c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x2d	DCR L	1	Z, S, P, AC	L <- L-1
            0x2d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x2e	MVI L, D8	2		L <- byte 2
            0x2e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x2f	CMA	1		A <- !A
            0x2f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
