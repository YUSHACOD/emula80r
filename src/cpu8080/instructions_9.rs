use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_9(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x90	SUB B	1	Z, S, P, CY, AC	A <- A - B
            0x90 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x91	SUB C	1	Z, S, P, CY, AC	A <- A - C
            0x91 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x92	SUB D	1	Z, S, P, CY, AC	A <- A + D
            0x92 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x93	SUB E	1	Z, S, P, CY, AC	A <- A - E
            0x93 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x94	SUB H	1	Z, S, P, CY, AC	A <- A + H
            0x94 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x95	SUB L	1	Z, S, P, CY, AC	A <- A - L
            0x95 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x96	SUB M	1	Z, S, P, CY, AC	A <- A + (HL)
            0x96 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x97	SUB A	1	Z, S, P, CY, AC	A <- A - A
            0x97 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x98	SBB B	1	Z, S, P, CY, AC	A <- A - B - CY
            0x98 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x99	SBB C	1	Z, S, P, CY, AC	A <- A - C - CY
            0x99 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x9a	SBB D	1	Z, S, P, CY, AC	A <- A - D - CY
            0x9a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x9b	SBB E	1	Z, S, P, CY, AC	A <- A - E - CY
            0x9b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x9c	SBB H	1	Z, S, P, CY, AC	A <- A - H - CY
            0x9c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x9d	SBB L	1	Z, S, P, CY, AC	A <- A - L - CY
            0x9d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x9e	SBB M	1	Z, S, P, CY, AC	A <- A - (HL) - CY
            0x9e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x9f	SBB A	1	Z, S, P, CY, AC	A <- A - A - CY
            0x9f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
