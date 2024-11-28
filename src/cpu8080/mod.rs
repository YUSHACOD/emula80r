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

    /// memmory aka ram
    memmory: Box<[u8; 65536]>,

    /// enable, disable flag
    enabled: bool,

    /// io table
    io_table: Box<[u8; 256]>,

    /// io port
    io_port: u16,
}

impl CPU {
    pub fn new() -> Self {
        let flags = ConditionFlags {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
            aux_carry: false,
        };

        let registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        };

        let memmory = Box::new([0u8; 65536]);

        let io_table = Box::new([0u8; 256]);

        Self {
            flags,
            registers,
            stack_pointer: 0,
            inst_pointer: 0,
            memmory,
            io_table,
            io_port: 0,
            enabled: false,
        }
    }
}
