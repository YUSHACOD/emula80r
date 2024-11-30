#[cfg(test)]
mod tests {
    use crate::cpu8080::{ConditionFlags, Registers, CPU};
    use crate::logging::config_builder::get_config;
    use log::info;
    use log4rs;
    use std::io;

    #[test]
    fn test_0x01() -> io::Result<()> {
        let _ = log4rs::init_config(get_config());

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


        info!("Before execution 1");
        info!("{}", cpu.get_dbg_string());
        // Execute the LXI B instruction
        cpu.execute_instruction();
        info!("After execution 1");
        info!("{}", cpu.get_dbg_string());

        cpu.get_dbg_memory()?;

        // Verify the result
        assert_eq!(cpu.registers.b, 0x12); // High byte
        assert_eq!(cpu.registers.c, 0x34); // Low byte
        // assert_eq!(cpu.inst_pointer, 0x01 + 3); // Low byte

        Ok(())
    }

    #[test]
    fn test_0x02() -> io::Result<()> {
        let _ = log4rs::init_config(get_config());

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

        info!("Before execution 2");
        info!("{}", cpu.get_dbg_string());
        // Execute the STAX B instruction
        cpu.execute_instruction();
        info!("After execution 2");
        info!("{}", cpu.get_dbg_string());

        cpu.get_dbg_memory()?;

        // Verify the result
        let address = 0x1234; // BC = 0x1234
        assert_eq!(cpu.memory[address as usize], 0x42); // A was stored at memory[BC]

        Ok(())
    }

    #[test]
    fn test_0x04() -> io::Result<()> {
        let _ = log4rs::init_config(get_config());

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
                a: 0x00,
                b: 0xFF, // Initial value of B (to test carry behavior and wrapping)
                c: 0x00,
                d: 0x00,
                e: 0x00,
                h: 0x00,
                l: 0x00,
            },
            stack_pointer: 0,
            inst_pointer: 0x01,
            memory: Box::new([0u8; 65536]),
            enabled: true,
            io_table: Box::new([0u8; 256]),
            io_port: 0,
        };

        // Set the instruction for INR B (opcode 0x04) in memory
        cpu.memory[0x01] = 0x04;

        info!("Before execution 0x04");
        info!("{}", cpu.get_dbg_string());

        // Execute the INR B instruction
        cpu.execute_instruction();

        info!("After execution 0x04");
        info!("{}", cpu.get_dbg_string());

        // Verify the results
        assert_eq!(cpu.registers.b, 0x00); // B should wrap around to 0x00
        assert!(cpu.flags.zero); // Zero flag should be set
        assert!(!cpu.flags.sign); // Sign flag should be cleared
        assert!(cpu.flags.parity); // Parity flag should be set (0 has even parity)
        assert!(cpu.flags.aux_carry); // Aux carry should be set (carry from bit 3 to bit 4)

        cpu.get_dbg_memory()?;
        Ok(())
    }

    #[test]
    fn test_0x05() -> io::Result<()> {
        let _ = log4rs::init_config(get_config());

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
                a: 0x00,
                b: 0x01, // Initial value of B (to test zero flag behavior)
                c: 0x00,
                d: 0x00,
                e: 0x00,
                h: 0x00,
                l: 0x00,
            },
            stack_pointer: 0,
            inst_pointer: 0x01,
            memory: Box::new([0u8; 65536]),
            enabled: true,
            io_table: Box::new([0u8; 256]),
            io_port: 0,
        };

        // Set the instruction for DCR B (opcode 0x05) in memory
        cpu.memory[0x01] = 0x05;

        info!("Before execution 0x05");
        info!("{}", cpu.get_dbg_string());

        // Execute the DCR B instruction
        cpu.execute_instruction();

        info!("After execution 0x05");
        info!("{}", cpu.get_dbg_string());

        // Verify the results
        assert_eq!(cpu.registers.b, 0x00);
        assert!(cpu.flags.zero); 
        assert!(!cpu.flags.sign); 
        assert!(cpu.flags.parity);
        assert!(!cpu.flags.aux_carry);

        // Additional test for decrementing below zero
        cpu.registers.b = 0x00;
        cpu.inst_pointer -= 1;
        cpu.execute_instruction();
        assert_eq!(cpu.registers.b, 0xFF); // B should wrap around to 0xFF
        assert!(!cpu.flags.zero); // Zero flag should be cleared
        assert!(cpu.flags.sign); // Sign flag should be set
        assert!(cpu.flags.parity); // Parity flag should be cleared (odd parity for 0xFF)
        assert!(cpu.flags.aux_carry); // Aux carry should still be set (borrow from bit 3)

        cpu.get_dbg_memory()?;
        Ok(())
    }
}
