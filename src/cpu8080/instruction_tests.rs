#[cfg(test)]
mod tests {
    use crate::cpu8080::{ConditionFlags, Registers, CPU};
    use crate::logging::config_builder::get_config;
    use log::info;
    use log4rs;
    use std::io;

    #[test]
    fn test_0x01() -> io::Result<()> {
        log4rs::init_config(get_config()).unwrap();

        // Initialize CPU
        let mut cpu = CPU {
            flags: ConditionFlags {
                zero: false,
                sign: false,
                parity: false,
                carry: false,
                aux_carry: false,
            },
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
            },
            stack_pointer: 0,
            inst_pointer: 0,
            memory: Box::new([0u8; 65536]),
            enabled: true,
            io_table: Box::new([0u8; 256]),
            io_port: 0,
        };

        cpu.inst_pointer = 0x01;
        cpu.memory[0x01] = 0x01;
        cpu.memory[0x02] = 0x12;
        cpu.memory[0x03] = 0x34;


        info!("Before execution");
        info!("{}", cpu.get_dbg_string());
        // Execute the LXI B instruction
        cpu.execute_instruction();
        info!("After execution");
        info!("{}", cpu.get_dbg_string());

        cpu.get_dbg_memory()?;

        // Verify the result
        assert_eq!(cpu.registers.b, 0x12); // High byte
        assert_eq!(cpu.registers.c, 0x34); // Low byte
        // assert_eq!(cpu.inst_pointer, 0x01 + 3); // Low byte

        Ok(())
    }

    fn test_0x02() -> io::Result<()> {
        log4rs::init_config(get_config()).unwrap();

        // Initialize CPU
        let mut cpu = CPU {
            flags: ConditionFlags {
                zero: false,
                sign: false,
                parity: false,
                carry: false,
                aux_carry: false,
            },
            registers: Registers {
                a: 0x42, // Value to store
                b: 0x12, // High byte of the address
                c: 0x34, // Low byte of the address
                d: 0,
                e: 0,
                h: 0,
                l: 0,
            },
            stack_pointer: 0,
            inst_pointer: 0,
            memory: Box::new([0u8; 65536]),
            enabled: true,
            io_table: Box::new([0u8; 256]),
            io_port: 0,
        };

        cpu.inst_pointer = 0x01;
        cpu.memory[0x01] = 0x02;

        info!("Before execution");
        info!("{}", cpu.get_dbg_string());
        // Execute the STAX B instruction
        cpu.execute_instruction();
        info!("After execution");
        info!("{}", cpu.get_dbg_string());

        cpu.get_dbg_memory()?;

        // Verify the result
        let address = 0x1234; // BC = 0x1234
        assert_eq!(cpu.memory[address as usize], 0x42); // A was stored at memory[BC]

        Ok(())
    }
}
