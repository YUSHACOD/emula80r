use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_a(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xa0	ANA B	1	Z, S, P, CY, AC	A <- A & B
            0xa0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa1	ANA C	1	Z, S, P, CY, AC	A <- A & C
            0xa1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa2	ANA D	1	Z, S, P, CY, AC	A <- A & D
            0xa2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa3	ANA E	1	Z, S, P, CY, AC	A <- A & E
            0xa3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa4	ANA H	1	Z, S, P, CY, AC	A <- A & H
            0xa4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa5	ANA L	1	Z, S, P, CY, AC	A <- A & L
            0xa5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa6	ANA M	1	Z, S, P, CY, AC	A <- A & (HL)
            0xa6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa7	ANA A	1	Z, S, P, CY, AC	A <- A & A
            0xa7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa8	XRA B	1	Z, S, P, CY, AC	A <- A ^ B
            0xa8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xa9	XRA C	1	Z, S, P, CY, AC	A <- A ^ C
            0xa9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xaa	XRA D	1	Z, S, P, CY, AC	A <- A ^ D
            0xaa => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xab	XRA E	1	Z, S, P, CY, AC	A <- A ^ E
            0xab => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xac	XRA H	1	Z, S, P, CY, AC	A <- A ^ H
            0xac => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xad	XRA L	1	Z, S, P, CY, AC	A <- A ^ L
            0xad => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xae	XRA M	1	Z, S, P, CY, AC	A <- A ^ (HL)
            0xae => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0xaf	XRA A	1	Z, S, P, CY, AC	A <- A ^ A
            0xaf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
