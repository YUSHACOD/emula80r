use log4rs;
use log::info;
use std::io;

mod cpu8080;
mod ioutils;
mod logging;

use logging::config_builder::get_config;

fn main() -> io::Result<()> {
    // Logger init
    log4rs::init_config(get_config()).unwrap();

    info!("Creating Cpu instance.");
    let mut cpu = cpu8080::CPU::new();
    info!("Cpu instance created.");

    info!("{}", cpu.get_dbg_string());

    // cpu.get_dbg_memory()?;
    // cpu.get_dbg_io_table()?;
    //
    // cpu.dump_memory_to_file()?;
    // cpu.dump_io_table_to_file()?;

    cpu.execute_instruction();

    Ok(())
}
