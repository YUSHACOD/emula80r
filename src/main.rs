use std::io;

mod cpu8080;
mod ioutils;
mod logging;

fn main() -> io::Result<()> {
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
