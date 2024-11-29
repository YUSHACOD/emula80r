use super::CPU;
use log::error;

impl CPU {
    pub fn execute_instruction(&mut self) {
        let inst_pointer = self.inst_pointer as usize;
        let instruction = self.memory[inst_pointer];

        match instruction {
            0x00 => {
                self.inst_pointer += 1;
            }

            0x01 => {
                self.registers.b = self.memory[inst_pointer + 1];
                self.registers.c = self.memory[inst_pointer + 2];
                self.inst_pointer += 3;
            }

            0x02 => {
                let location = ((self.registers.b as u16) << 8) | (self.registers.c as u16);
                self.memory[location as usize] = self.registers.a;
            }

            0x03 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x04 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x05 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x06 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x07 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x08 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x09 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x0f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x10 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x11 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x12 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x13 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x14 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x15 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x16 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x17 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x18 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x19 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x1f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x20 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x21 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x22 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x23 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x24 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x25 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x26 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x27 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x28 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x29 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x2f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x30 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x31 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x32 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x33 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x34 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x35 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x36 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x37 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x38 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x39 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x3f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x40 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x41 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x42 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x43 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x44 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x45 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x46 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x47 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x48 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x49 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x4f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x50 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x51 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x52 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x53 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x54 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x55 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x56 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x57 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x58 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x59 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x5f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x60 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x61 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x62 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x63 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x64 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x65 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x66 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x67 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x68 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x69 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x6f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x70 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x71 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x72 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x73 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x74 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x75 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x76 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x77 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x78 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x79 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x7f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x80 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x81 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x82 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x83 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x84 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x85 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x86 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x87 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x88 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x89 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x8f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x90 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x91 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x92 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x93 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x94 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x95 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x96 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x97 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x98 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x99 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9a => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9b => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9c => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9d => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9e => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0x9f => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xa9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xaa => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xab => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xac => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xad => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xae => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xaf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xb9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xba => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbe => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xbf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xc9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xca => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xcb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xcc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xcd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xce => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xcf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xd9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xda => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xdb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xdc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xdd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xde => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xdf => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xe9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xea => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xeb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xec => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xed => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xee => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xef => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf0 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf1 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf2 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf3 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf4 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf5 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf6 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf7 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf8 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xf9 => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfa => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfb => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfc => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfd => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xfe => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }

            0xff => {
                error!("unimplemented instruction : {:02X}", instruction,);
            }
        }
    }
}
