use super::CPU;

impl CPU {
    pub fn execute_instruction(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x00..=0x0f => self.execute_instruction_0(),
            0x10..=0x1f => self.execute_instruction_1(),
            0x20..=0x2f => self.execute_instruction_2(),
            0x30..=0x3f => self.execute_instruction_3(),
            0x40..=0x4f => self.execute_instruction_4(),
            0x50..=0x5f => self.execute_instruction_5(),
            0x60..=0x6f => self.execute_instruction_6(),
            0x70..=0x7f => self.execute_instruction_7(),
            0x80..=0x8f => self.execute_instruction_8(),
            0x90..=0x9f => self.execute_instruction_9(),
            0xa0..=0xaf => self.execute_instruction_a(),
            0xb0..=0xbf => self.execute_instruction_b(),
            0xc0..=0xcf => self.execute_instruction_c(),
            0xd0..=0xdf => self.execute_instruction_d(),
            0xe0..=0xef => self.execute_instruction_e(),
            0xf0..=0xff => self.execute_instruction_f(),
        }
    }
}
