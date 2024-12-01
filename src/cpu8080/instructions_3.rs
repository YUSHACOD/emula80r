use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_3(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x30	-			
            0x30 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x31	LXI SP, D16	3		SP.hi <- byte 3, SP.lo <- byte 2
            0x31 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x32	STA adr	3		(adr) <- A
            0x32 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x33	INX SP	1		SP = SP + 1
            0x33 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x34	INR M	1	Z, S, P, AC	(HL) <- (HL)+1
            0x34 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x35	DCR M	1	Z, S, P, AC	(HL) <- (HL)-1
            0x35 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x36	MVI M,D8	2		(HL) <- byte 2
            0x36 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x37	STC	1	CY	CY = 1
            0x37 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x38	-			
            0x38 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x39	DAD SP	1	CY	HL = HL + SP
            0x39 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x3a	LDA adr	3		A <- (adr)
            0x3a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x3b	DCX SP	1		SP = SP-1
            0x3b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x3c	INR A	1	Z, S, P, AC	A <- A+1
            0x3c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x3d	DCR A	1	Z, S, P, AC	A <- A-1
            0x3d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x3e	MVI A,D8	2		A <- byte 2
            0x3e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x3f	CMC	1	CY	CY=!CY
            0x3f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
