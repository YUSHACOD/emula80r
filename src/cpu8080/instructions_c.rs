use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_c(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xc0	RNZ	1		if NZ, RET
            0xc0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc1	POP B	1		C <- (sp); B <- (sp+1); sp <- sp+2
            0xc1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc2	JNZ adr	3		if NZ, PC <- adr
            0xc2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc3	JMP adr	3		PC <= adr
            0xc3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc4	CNZ adr	3		if NZ, CALL adr
            0xc4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc5	PUSH B	1		(sp-2)<-C; (sp-1)<-B; sp <- sp - 2
            0xc5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc6	ADI D8	2	Z, S, P, CY, AC	A <- A + byte
            0xc6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc7	RST 0	1		CALL $0
            0xc7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc8	RZ	1		if Z, RET
            0xc8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xc9	RET	1		PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2
            0xc9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xca	JZ adr	3		if Z, PC <- adr
            0xca => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xcb	-			
            0xcb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xcc	CZ adr	3		if Z, CALL adr
            0xcc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xcd	CALL adr	3		(SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            0xcd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xce	ACI D8	2	Z, S, P, CY, AC	A <- A + data + CY
            0xce => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xcf	RST 1	1		CALL $8
            0xcf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
