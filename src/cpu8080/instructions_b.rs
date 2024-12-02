use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::error;

impl Cpu {
    pub fn execute_instruction_b(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0xb0	ORA B	1	Z, S, P, CY, AC	A <- A | B
            0xb0 => {
                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, self.registers.b, false);

                self.inst_pointer += 1;
            }

            // 0xb1	ORA C	1	Z, S, P, CY, AC	A <- A | C
            0xb1 => {
                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, self.registers.c, false);

                self.inst_pointer += 1;
            }

            // 0xb2	ORA D	1	Z, S, P, CY, AC	A <- A | D
            0xb2 => {
                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, self.registers.d, false);

                self.inst_pointer += 1;
            }

            // 0xb3	ORA E	1	Z, S, P, CY, AC	A <- A | E
            0xb3 => {
                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, self.registers.e, false);

                self.inst_pointer += 1;
            }

            // 0xb4	ORA H	1	Z, S, P, CY, AC	A <- A | H
            0xb4 => {
                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, self.registers.h, false);

                self.inst_pointer += 1;
            }

            // 0xb5	ORA L	1	Z, S, P, CY, AC	A <- A | L
            0xb5 => {
                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, self.registers.l, false);

                self.inst_pointer += 1;
            }

            // 0xb6	ORA M	1	Z, S, P, CY, AC	A <- A | (HL)
            0xb6 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                let value = self.memory[location as usize];

                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, value, false);

                self.inst_pointer += 1;
            }

            // 0xb7	ORA A	1	Z, S, P, CY, AC	A <- A | A
            0xb7 => {
                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, self.registers.a, false);

                self.inst_pointer += 1;
            }

            // 0xb8	CMP B	1	Z, S, P, CY, AC	A - B
            0xb8 => {
                self.registers.a =
                    self.alu_operation(Operation::Cmp, self.registers.a, self.registers.b, false);

                self.inst_pointer += 1;
            }

            // 0xb9	CMP C	1	Z, S, P, CY, AC	A - C
            0xb9 => {
                self.registers.a =
                    self.alu_operation(Operation::Cmp, self.registers.a, self.registers.c, false);

                self.inst_pointer += 1;
            }

            // 0xba	CMP D	1	Z, S, P, CY, AC	A - D
            0xba => {
                self.registers.a =
                    self.alu_operation(Operation::Cmp, self.registers.a, self.registers.d, false);

                self.inst_pointer += 1;
            }

            // 0xbb	CMP E	1	Z, S, P, CY, AC	A - E
            0xbb => {
                self.registers.a =
                    self.alu_operation(Operation::Cmp, self.registers.a, self.registers.e, false);

                self.inst_pointer += 1;
            }

            // 0xbc	CMP H	1	Z, S, P, CY, AC	A - H
            0xbc => {
                self.registers.a =
                    self.alu_operation(Operation::Cmp, self.registers.a, self.registers.h, false);

                self.inst_pointer += 1;
            }

            // 0xbd	CMP L	1	Z, S, P, CY, AC	A - L
            0xbd => {
                self.registers.a =
                    self.alu_operation(Operation::Cmp, self.registers.a, self.registers.l, false);

                self.inst_pointer += 1;
            }

            // 0xbe	CMP M	1	Z, S, P, CY, AC	A - (HL)
            0xbe => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                let value = self.memory[location as usize];

                self.registers.a =
                    self.alu_operation(Operation::Or, self.registers.a, value, false);

                self.inst_pointer += 1;
            }

            // 0xbf	CMP A	1	Z, S, P, CY, AC	A - A
            0xbf => {
                self.registers.a =
                    self.alu_operation(Operation::Cmp, self.registers.a, self.registers.a, false);

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
