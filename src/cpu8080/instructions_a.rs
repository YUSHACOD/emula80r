use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::error;

impl Cpu {
    pub fn execute_instruction_a(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xa0	ANA B	1	Z, S, P, CY, AC	A <- A & B
            0xa0 => {
                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, self.registers.b, false);

                self.inst_pointer += 1;
            }

            // 0xa1	ANA C	1	Z, S, P, CY, AC	A <- A & C
            0xa1 => {
                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, self.registers.c, false);

                self.inst_pointer += 1;
            }

            // 0xa2	ANA D	1	Z, S, P, CY, AC	A <- A & D
            0xa2 => {
                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, self.registers.d, false);

                self.inst_pointer += 1;
            }

            // 0xa3	ANA E	1	Z, S, P, CY, AC	A <- A & E
            0xa3 => {
                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, self.registers.e, false);

                self.inst_pointer += 1;
            }

            // 0xa4	ANA H	1	Z, S, P, CY, AC	A <- A & H
            0xa4 => {
                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, self.registers.h, false);

                self.inst_pointer += 1;
            }

            // 0xa5	ANA L	1	Z, S, P, CY, AC	A <- A & L
            0xa5 => {
                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, self.registers.l, false);

                self.inst_pointer += 1;
            }

            // 0xa6	ANA M	1	Z, S, P, CY, AC	A <- A & (HL)
            0xa6 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                let value = self.memory[location as usize];

                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, value, false);

                self.inst_pointer += 1;
            }

            // 0xa7	ANA A	1	Z, S, P, CY, AC	A <- A & A
            0xa7 => {
                self.registers.a =
                    self.alu_operation(Operation::And, self.registers.a, self.registers.a, false);

                self.inst_pointer += 1;
            }

            // 0xa8	XRA B	1	Z, S, P, CY, AC	A <- A ^ B
            0xa8 => {
                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, self.registers.b, false);

                self.inst_pointer += 1;
            }

            // 0xa9	XRA C	1	Z, S, P, CY, AC	A <- A ^ C
            0xa9 => {
                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, self.registers.c, false);

                self.inst_pointer += 1;
            }

            // 0xaa	XRA D	1	Z, S, P, CY, AC	A <- A ^ D
            0xaa => {
                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, self.registers.d, false);

                self.inst_pointer += 1;
            }

            // 0xab	XRA E	1	Z, S, P, CY, AC	A <- A ^ E
            0xab => {
                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, self.registers.e, false);

                self.inst_pointer += 1;
            }

            // 0xac	XRA H	1	Z, S, P, CY, AC	A <- A ^ H
            0xac => {
                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, self.registers.h, false);

                self.inst_pointer += 1;
            }

            // 0xad	XRA L	1	Z, S, P, CY, AC	A <- A ^ L
            0xad => {
                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, self.registers.l, false);

                self.inst_pointer += 1;
            }

            // 0xae	XRA M	1	Z, S, P, CY, AC	A <- A ^ (HL)
            0xae => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                let value = self.memory[location as usize];

                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, value, false);

                self.inst_pointer += 1;
            }

            // 0xaf	XRA A	1	Z, S, P, CY, AC	A <- A ^ A
            0xaf => {
                self.registers.a =
                    self.alu_operation(Operation::Xor, self.registers.a, self.registers.a, false);

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
