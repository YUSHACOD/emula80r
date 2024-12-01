use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_e(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xe0	RPO	1		if PO, RET
            0xe0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe1	POP H	1		L <- (sp); H <- (sp+1); sp <- sp+2
            0xe1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe2	JPO adr	3		if PO, PC <- adr
            0xe2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe3	XTHL	1		L <-> (SP); H <-> (SP+1)
            0xe3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe4	CPO adr	3		if PO, CALL adr
            0xe4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe5	PUSH H	1		(sp-2)<-L; (sp-1)<-H; sp <- sp - 2
            0xe5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe6	ANI D8	2	Z, S, P, CY, AC	A <- A & data
            0xe6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe7	RST 4	1		CALL $20
            0xe7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe8	RPE	1		if PE, RET
            0xe8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xe9	PCHL	1		PC.hi <- H; PC.lo <- L
            0xe9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xea	JPE adr	3		if PE, PC <- adr
            0xea => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xeb	XCHG	1		H <-> D; L <-> E
            0xeb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xec	CPE adr	3		if PE, CALL adr
            0xec => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xed	-			
            0xed => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xee	XRI D8	2	Z, S, P, CY, AC	A <- A ^ data
            0xee => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xef	RST 5	1		CALL $28
            0xef => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
