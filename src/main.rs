use log::info;
use std::io;

mod cpu8080;
mod ioutils;
mod logging;

use logging::config_builder::get_config;

fn main() -> io::Result<()> {
    // Logger init
    log4rs::init_config(get_config()).unwrap();

    info!("Cpudiag test inititation: ");
    info!("Creating Cpu instance.");
    let mut cpu = cpu8080::Cpu::new();
    info!("Cpu instance created.");
    info!("{}", cpu.get_dbg_string());



    info!("Loading the binary");
    cpu.load_program("cpudiag.bin", 0x100)?;
    info!("Binary loaded");

    info!("Setting up");
    // Start from 0x100
    cpu.set_pc(0x100);

    // Test fix
    cpu.memory[0x170] = 0x07;
    info!("Setup done");

    cpu.enable_cpu();
    info!("Cpu enabled");


    info!("Starting execution");
    while cpu.on() {
        info!("{}", cpu.get_dbg_string());
        cpu.execute_instruction();
    }
    info!("{}", cpu.get_dbg_string());
    info!("Exectuion Ended");
    cpu.get_dbg_memory()?;
    cpu.get_dbg_io_table()?;
    cpu.dump_memory_to_file()?;
    cpu.dump_io_table_to_file()?;
    info!("All debug written. Test ended");
    

    cpu.execute_instruction();

    Ok(())
}
