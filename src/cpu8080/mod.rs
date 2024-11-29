use crate::{ioutils, logging::dat_chunk};
use chrono::Local;
use std::io;

mod utils;

/// Flags for 8080 cpu
struct ConditionFlags {
    /// if the last arithmetic operation results 0
    zero: bool,

    /// if the last arithmetic operation results into a value which has
    /// the 7th bit (msb) set to 1
    sign: bool,

    /// parity of the result
    parity: bool,

    /// carry if a carry is generated from arithmetic output
    carry: bool,

    /// auxillary carry auxillary carry
    aux_carry: bool,
}

impl ConditionFlags {
    fn new() -> Self {
        Self {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
            aux_carry: false,
        }
    }

    fn get_dbg_string(&self) -> String {
        format!(
            "[Flags]:
\t\tzero:\t\t[{}]
\t\tsign:\t\t[{}]
\t\tparity:\t\t[{}]
\t\tcarry:\t\t[{}]
\t\taux_carry:\t[{}]",
            &self.zero, &self.sign, &self.parity, &self.carry, &self.aux_carry
        )
    }
}

/// All the Registers for the 8080 cpu is in this struct
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl Registers {
    fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    fn get_dbg_string(&self) -> String {
        format!(
            "[Registers]:
\t\ta:\t\t[{}]
\t\tb:\t\t[{}]
\t\tc:\t\t[{}]
\t\td:\t\t[{}]
\t\te:\t\t[{}]
\t\th:\t\t[{}]
\t\tl:\t\t[{}]",
            &self.a, &self.b, &self.c, &self.d, &self.e, &self.h, &self.l
        )
    }
}

/// CPU structure, basically all the things 8080 cpu requires for function
pub struct CPU {
    /// condition flags
    flags: ConditionFlags,

    /// registers
    registers: Registers,

    /// stack pointer
    stack_pointer: u16,

    /// instruction pointer, or program counter
    inst_pointer: u16,

    /// memory aka ram
    memory: Box<[u8; 65536]>,

    /// enable, disable flag
    enabled: bool,

    /// io table
    io_table: Box<[u8; 256]>,

    /// io port
    io_port: u16,
}

impl CPU {
    pub fn new() -> Self {
        let flags = ConditionFlags::new();

        let registers = Registers::new();

        let memory = Box::new([0u8; 65536]);

        let io_table = Box::new([0u8; 256]);

        Self {
            flags,
            registers,
            stack_pointer: 0,
            inst_pointer: 0,
            memory,
            io_table,
            io_port: 0,
            enabled: false,
        }
    }

    pub fn get_dbg_string(&self) -> String {
        format!(
            "+++++++++++++++++++++++++++++++++++++++
[CPU state]:\n
\t{}\n
\t{}\n
\t[Stack Pointer]:\t[{}]\n
\t[Instruction Pointer]:\t[{}]\n
\t[Io Port]:\t\t[{}]\n
\t[Enabled]:\t\t[{}]
+++++++++++++++++++++++++++++++++++++++\n",
            &self.flags.get_dbg_string(),
            &self.registers.get_dbg_string(),
            &self.stack_pointer,
            &self.inst_pointer,
            &self.io_port,
            &self.enabled
        )
    }

    pub fn get_dbg_memory(&self) -> io::Result<()> {
        let hex_dump = dat_chunk::dump_data(&self.memory.as_slice(), "Memory");
        ioutils::write_file(&Some("memory_dump.bin".to_string()), hex_dump.as_bytes())?;
        Ok(())
    }

    pub fn dump_memory_to_file(&self) -> io::Result<()> {
        let timestamp = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
        let filename = format!("raw_memory_dump_{}.bin", timestamp);
        ioutils::write_file(&Some(filename), &self.memory.as_slice())?;
        Ok(())
    }

    pub fn get_dbg_io_table(&self) -> io::Result<()> {
        let hex_dump = dat_chunk::dump_data(&self.io_table.as_slice(), "IO_table");
        ioutils::write_file(&Some("io_table.bin".to_string()), hex_dump.as_bytes())?;
        Ok(())
    }

    pub fn dump_io_table_to_file(&self) -> io::Result<()> {
        let timestamp = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
        let filename = format!("raw_io_table_dump_{}.bin", timestamp);
        ioutils::write_file(&Some(filename), &self.io_table.as_slice())?;
        Ok(())
    }
}
