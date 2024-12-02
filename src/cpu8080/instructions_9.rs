use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::error;

impl Cpu {
    pub fn execute_instruction_9(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x90	SUB B	1	Z, S, P, CY, AC	A <- A - B
            0x90 => {
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, self.registers.b, false);

                self.inst_pointer += 1;
            }

            // 0x91	SUB C	1	Z, S, P, CY, AC	A <- A - C
            0x91 => {
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, self.registers.c, false);

                self.inst_pointer += 1;
            }

            // 0x92	SUB D	1	Z, S, P, CY, AC	A <- A + D
            0x92 => {
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, self.registers.d, false);

                self.inst_pointer += 1;
            }

            // 0x93	SUB E	1	Z, S, P, CY, AC	A <- A - E
            0x93 => {
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, self.registers.e, false);

                self.inst_pointer += 1;
            }

            // 0x94	SUB H	1	Z, S, P, CY, AC	A <- A + H
            0x94 => {
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, self.registers.h, false);

                self.inst_pointer += 1;
            }

            // 0x95	SUB L	1	Z, S, P, CY, AC	A <- A - L
            0x95 => {
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, self.registers.l, false);

                self.inst_pointer += 1;
            }

            // 0x96	SUB M	1	Z, S, P, CY, AC	A <- A + (HL)
            0x96 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                let value = self.memory[location as usize];

                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, value, false);

                self.inst_pointer += 1;
            }

            // 0x97	SUB A	1	Z, S, P, CY, AC	A <- A - A
            0x97 => {
                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, self.registers.a, false);

                self.inst_pointer += 1;
            }

            // 0x98	SBB B	1	Z, S, P, CY, AC	A <- A - B - CY
            0x98 => {
                self.registers.a = self.alu_operation(
                    Operation::Sub,
                    self.registers.a,
                    self.registers.b,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x99	SBB C	1	Z, S, P, CY, AC	A <- A - C - CY
            0x99 => {
                self.registers.a = self.alu_operation(
                    Operation::Sub,
                    self.registers.a,
                    self.registers.c,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x9a	SBB D	1	Z, S, P, CY, AC	A <- A - D - CY
            0x9a => {
                self.registers.a = self.alu_operation(
                    Operation::Sub,
                    self.registers.a,
                    self.registers.d,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x9b	SBB E	1	Z, S, P, CY, AC	A <- A - E - CY
            0x9b => {
                self.registers.a = self.alu_operation(
                    Operation::Sub,
                    self.registers.a,
                    self.registers.e,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x9c	SBB H	1	Z, S, P, CY, AC	A <- A - H - CY
            0x9c => {
                self.registers.a = self.alu_operation(
                    Operation::Sub,
                    self.registers.a,
                    self.registers.h,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x9d	SBB L	1	Z, S, P, CY, AC	A <- A - L - CY
            0x9d => {
                self.registers.a = self.alu_operation(
                    Operation::Sub,
                    self.registers.a,
                    self.registers.l,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x9e	SBB M	1	Z, S, P, CY, AC	A <- A - (HL) - CY
            0x9e => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                let value = self.memory[location as usize];

                self.registers.a =
                    self.alu_operation(Operation::Sub, self.registers.a, value, self.flags.carry);

                self.inst_pointer += 1;
            }

            // 0x9f	SBB A	1	Z, S, P, CY, AC	A <- A - A - CY
            0x9f => {
                self.registers.a = self.alu_operation(
                    Operation::Sub,
                    self.registers.a,
                    self.registers.a,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            _ => {
                error!("Something very bad happened");
                error!("{}", self.get_dbg_string());
            }
        }
    }
}
