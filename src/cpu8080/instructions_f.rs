use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_f(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xf0	RP	1		if P, RET
            0xf0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf1	POP PSW	1		flags <- (sp); A <- (sp+1); sp <- sp+2
            0xf1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf2	JP adr	3		if P=1 PC <- adr
            0xf2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf3	DI	1		special
            0xf3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf4	CP adr	3		if P, PC <- adr
            0xf4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf5	PUSH PSW	1		(sp-2)<-flags; (sp-1)<-A; sp <- sp - 2
            0xf5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf6	ORI D8	2	Z, S, P, CY, AC	A <- A | data
            0xf6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf7	RST 6	1		CALL $30
            0xf7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf8	RM	1		if M, RET
            0xf8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xf9	SPHL	1		SP=HL
            0xf9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xfa	JM adr	3		if M, PC <- adr
            0xfa => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xfb	EI	1		special
            0xfb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xfc	CM adr	3		if M, CALL adr
            0xfc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xfd	-			
            0xfd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xfe	CPI D8	2	Z, S, P, CY, AC	A - data
            0xfe => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xff	RST 7	1		CALL $38
            0xff => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
