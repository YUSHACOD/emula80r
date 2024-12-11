use events::Input;
use log::info;
use std::io;
use std::sync::mpsc::Receiver;

mod cpu8080;
mod events;
mod ioutils;
mod logging;

use logging::config_builder::get_config;

fn block(rcv: &Receiver<Input>) -> Option<()> {
    let mut result = Some(());
    loop {
        match rcv.recv().unwrap_or(Input::Bs) {
            Input::Start | Input::Pause => {
                break;
            }
            Input::Quit => {
                result = None;
                break;
            }
            _ => {}
        }
    }
    result
}

fn main() -> io::Result<()> {
    // Logger init
    log4rs::init_config(get_config()).unwrap();

    // Events stuff
    let (sdr, rcv) = std::sync::mpsc::channel();
    let input_channel = std::thread::spawn(move || events::start(sdr));
    let default = Input::Bs;
    let mut input: Input;

    info!("Cpudiag test inititation: ");
    info!("Creating Cpu instance.");
    let mut cpu = cpu8080::Cpu::new();
    info!("Cpu instance created.");
    info!("{}", cpu.get_dbg_string());
    cpu.get_dbg_memory()?;

    info!("Loading the binary");
    cpu.load_program("cpudiag.bin", 0x100)?;
    cpu.get_dbg_memory()?;
    info!("Binary loaded");

    info!("Setting up");
    // Start from 0x100
    cpu.set_pc(0x100);

    // Test fix
    cpu.memory[0x170] = 0x07;
    info!("Setup done");

    cpu.enable();
    info!("Cpu enabled");

    info!("Starting execution");
    while cpu.on() {
        input = rcv.try_recv().unwrap_or(default.clone());
        match input {
            Input::Quit => cpu.disable(),
            Input::Next => {
                info!("{}", cpu.get_dbg_string());
                cpu.execute_instruction();
            }
            Input::Pause => {
                if block(&rcv).is_none() {
                    break;
                }
            }
            _ => (),
        };
    }

    info!("{}", cpu.get_dbg_string());
    info!("Exectuion Ended");
    cpu.get_dbg_memory()?;
    cpu.get_dbg_io_table()?;
    cpu.dump_memory_to_file()?;
    cpu.dump_io_table_to_file()?;
    info!("All debug written. Test ended");

    let _ = input_channel.join().expect("input channel failed at last");
    Ok(())
}
