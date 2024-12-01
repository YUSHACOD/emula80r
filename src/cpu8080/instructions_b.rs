use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_b(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xb0	ORA B	1	Z, S, P, CY, AC	A <- A | B
            0xb0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb1	ORA C	1	Z, S, P, CY, AC	A <- A | C
            0xb1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb2	ORA D	1	Z, S, P, CY, AC	A <- A | D
            0xb2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb3	ORA E	1	Z, S, P, CY, AC	A <- A | E
            0xb3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb4	ORA H	1	Z, S, P, CY, AC	A <- A | H
            0xb4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb5	ORA L	1	Z, S, P, CY, AC	A <- A | L
            0xb5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb6	ORA M	1	Z, S, P, CY, AC	A <- A | (HL)
            0xb6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb7	ORA A	1	Z, S, P, CY, AC	A <- A | A
            0xb7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb8	CMP B	1	Z, S, P, CY, AC	A - B
            0xb8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xb9	CMP C	1	Z, S, P, CY, AC	A - C
            0xb9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xba	CMP D	1	Z, S, P, CY, AC	A - D
            0xba => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xbb	CMP E	1	Z, S, P, CY, AC	A - E
            0xbb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xbc	CMP H	1	Z, S, P, CY, AC	A - H
            0xbc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xbd	CMP L	1	Z, S, P, CY, AC	A - L
            0xbd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xbe	CMP M	1	Z, S, P, CY, AC	A - (HL)
            0xbe => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xbf	CMP A	1	Z, S, P, CY, AC	A - A
            0xbf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
