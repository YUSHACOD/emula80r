use super::Cpu;
use crate::cpu8080::utils::Operation;
use log::error;

impl Cpu {
    pub fn execute_instruction_8(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            // 0x80	ADD B	1	Z, S, P, CY, AC	A <- A + B
            0x80 => {
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, self.registers.b, false);

                self.inst_pointer += 1;
            }

            // 0x81	ADD C	1	Z, S, P, CY, AC	A <- A + C
            0x81 => {
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, self.registers.c, false);

                self.inst_pointer += 1;
            }

            // 0x82	ADD D	1	Z, S, P, CY, AC	A <- A + D
            0x82 => {
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, self.registers.d, false);

                self.inst_pointer += 1;
            }

            // 0x83	ADD E	1	Z, S, P, CY, AC	A <- A + E
            0x83 => {
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, self.registers.e, false);

                self.inst_pointer += 1;
            }

            // 0x84	ADD H	1	Z, S, P, CY, AC	A <- A + H
            0x84 => {
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, self.registers.h, false);

                self.inst_pointer += 1;
            }

            // 0x85	ADD L	1	Z, S, P, CY, AC	A <- A + L
            0x85 => {
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, self.registers.l, false);

                self.inst_pointer += 1;
            }

            // 0x86	ADD M	1	Z, S, P, CY, AC	A <- A + (HL)
            0x86 => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                let value = self.memory[location as usize];

                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, value, false);

                self.inst_pointer += 1;
            }

            // 0x87	ADD A	1	Z, S, P, CY, AC	A <- A + A
            0x87 => {
                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, self.registers.a, false);

                self.inst_pointer += 1;
            }

            // 0x88	ADC B	1	Z, S, P, CY, AC	A <- A + B + CY
            0x88 => {
                self.registers.a = self.alu_operation(
                    Operation::Add,
                    self.registers.a,
                    self.registers.b,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x89	ADC C	1	Z, S, P, CY, AC	A <- A + C + CY
            0x89 => {
                self.registers.a = self.alu_operation(
                    Operation::Add,
                    self.registers.a,
                    self.registers.c,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x8a	ADC D	1	Z, S, P, CY, AC	A <- A + D + CY
            0x8a => {
                self.registers.a = self.alu_operation(
                    Operation::Add,
                    self.registers.a,
                    self.registers.d,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x8b	ADC E	1	Z, S, P, CY, AC	A <- A + E + CY
            0x8b => {
                self.registers.a = self.alu_operation(
                    Operation::Add,
                    self.registers.a,
                    self.registers.e,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x8c	ADC H	1	Z, S, P, CY, AC	A <- A + H + CY
            0x8c => {
                self.registers.a = self.alu_operation(
                    Operation::Add,
                    self.registers.a,
                    self.registers.h,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x8d	ADC L	1	Z, S, P, CY, AC	A <- A + L + CY
            0x8d => {
                self.registers.a = self.alu_operation(
                    Operation::Add,
                    self.registers.a,
                    self.registers.l,
                    self.flags.carry,
                );

                self.inst_pointer += 1;
            }

            // 0x8e	ADC M	1	Z, S, P, CY, AC	A <- A + (HL) + CY
            0x8e => {
                let lower8 = self.registers.l;
                let upper8 = self.registers.h;

                let location = ((upper8 as u16) << 8) | (lower8 as u16);
                let value = self.memory[location as usize];

                self.registers.a =
                    self.alu_operation(Operation::Add, self.registers.a, value, self.flags.carry);

                self.inst_pointer += 1;
            }

            // 0x8f	ADC A	1	Z, S, P, CY, AC	A <- A + A + CY
            0x8f => {
                self.registers.a = self.alu_operation(
                    Operation::Add,
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
