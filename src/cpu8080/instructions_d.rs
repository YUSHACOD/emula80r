use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_d(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xd0	RNC	1		if NCY, RET
            0xd0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd1	POP D	1		E <- (sp); D <- (sp+1); sp <- sp+2
            0xd1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd2	JNC adr	3		if NCY, PC<-adr
            0xd2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd3	OUT D8	2		special
            0xd3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd4	CNC adr	3		if NCY, CALL adr
            0xd4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd5	PUSH D	1		(sp-2)<-E; (sp-1)<-D; sp <- sp - 2
            0xd5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd6	SUI D8	2	Z, S, P, CY, AC	A <- A - data
            0xd6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd7	RST 2	1		CALL $10
            0xd7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd8	RC	1		if CY, RET
            0xd8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xd9	-			
            0xd9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xda	JC adr	3		if CY, PC<-adr
            0xda => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xdb	IN D8	2		special
            0xdb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xdc	CC adr	3		if CY, CALL adr
            0xdc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xdd	-			
            0xdd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xde	SBI D8	2	Z, S, P, CY, AC	A <- A - data - CY
            0xde => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xdf	RST 3	1		CALL $18
            0xdf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
