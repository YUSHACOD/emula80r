use super::Cpu;
use log::error;

impl Cpu {
    pub fn execute_instruction_8(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x80	ADD B	1	Z, S, P, CY, AC	A <- A + B
            0x80 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x81	ADD C	1	Z, S, P, CY, AC	A <- A + C
            0x81 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x82	ADD D	1	Z, S, P, CY, AC	A <- A + D
            0x82 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x83	ADD E	1	Z, S, P, CY, AC	A <- A + E
            0x83 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x84	ADD H	1	Z, S, P, CY, AC	A <- A + H
            0x84 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x85	ADD L	1	Z, S, P, CY, AC	A <- A + L
            0x85 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x86	ADD M	1	Z, S, P, CY, AC	A <- A + (HL)
            0x86 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x87	ADD A	1	Z, S, P, CY, AC	A <- A + A
            0x87 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x88	ADC B	1	Z, S, P, CY, AC	A <- A + B + CY
            0x88 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x89	ADC C	1	Z, S, P, CY, AC	A <- A + C + CY
            0x89 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x8a	ADC D	1	Z, S, P, CY, AC	A <- A + D + CY
            0x8a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x8b	ADC E	1	Z, S, P, CY, AC	A <- A + E + CY
            0x8b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x8c	ADC H	1	Z, S, P, CY, AC	A <- A + H + CY
            0x8c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x8d	ADC L	1	Z, S, P, CY, AC	A <- A + L + CY
            0x8d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x8e	ADC M	1	Z, S, P, CY, AC	A <- A + (HL) + CY
            0x8e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            // 0x8f	ADC A	1	Z, S, P, CY, AC	A <- A + A + CY
            0x8f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
