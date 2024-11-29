use log4rs;
use std::io;

mod cpu8080;
mod ioutils;
mod logging;

use logging::config_builder::get_config;

fn main() -> io::Result<()> {
    // Logger init
    log4rs::init_config(get_config()).unwrap();

    println!("Creating Cpu instance.");
    let cpu = cpu8080::CPU::new();
    println!("Cpu instance created.");

    print!("{}", cpu.get_dbg_string());

    cpu.get_dbg_memory()?;
    cpu.get_dbg_io_table()?;

    cpu.dump_memory_to_file()?;
    cpu.dump_io_table_to_file()?;

    Ok(())
}
