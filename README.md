# Emula80r: Intel 8080 learning emulator

### What is this?
A tiny, step-through Intel 8080 CPU emulator meant for learning. It lets you load a small program and advance one instruction at a time while logging the CPU state (registers, flags, pointers, current opcode) and dumping memory/IO snapshots to files.

- **Goal**: Understand how a CPU’s state changes instruction-by-instruction.
- **Scope**: Keep things simple. No advanced peripherals or full system emulation.

### Features
- **Step execution**: Press a key to execute the next instruction.
- **State logging**: Human-readable dump of registers, flags, SP, PC, current opcode.
- **Memory/IO dumps**: Hex-style dumps and raw binary snapshots written to disk.
- **8080 focus**: Instruction decoding split by opcode ranges (0x00–0xFF) for clarity.

### Project layout
- `src/main.rs`: Program entry, event loop, loads a binary (e.g., `cpudiag.bin`) and steps the CPU.
- `src/cpu8080/`: CPU core, registers/flags, instruction decode/execute, helpers.
- `src/events.rs`: Keyboard input handling for stepping/pausing/quitting.
- `src/logging/`: Log configuration and hex dump utility.
- `src/ioutils.rs`: Simple file read/write helpers.

### Requirements
- Rust toolchain (2021 edition). Install via `rustup`.
- A terminal that accepts single-key input (Windows Terminal/PowerShell works).

### Setup
1. (Recommended) Create the logs directory so file logging works:
   ```bash
   mkdir -p logs
   ```
   On Windows PowerShell:
   ```powershell
   New-Item -ItemType Directory -Force logs | Out-Null
   ```
2. Place your program binary in the repository root (default expected in code is `cpudiag.bin`).

### Running
- Debug build:
  ```bash
  cargo run
  ```

On startup, the emulator:
- Initializes logging (console + `logs/emula80r.log`).
- Creates a new 8080 `Cpu` instance.
- Loads `cpudiag.bin` at address `0x0100`.
- Sets PC to `0x0100` and enables the CPU.
- Enters a loop waiting for your key presses to step or control execution.

### Controls (keyboard)
- **Enter**: Start
- **Space**: Pause (press Enter to resume, or see below)
- **n** or **Left Arrow**: Execute next instruction (step)
- **q**: Quit

When stepping, you’ll see the CPU state printed each time, e.g. registers, flags, SP, PC, current opcode, and whether the CPU is enabled.

### Outputs and artifacts
The emulator writes helpful logs and snapshots:
- Logs
  - `logs/emula80r.log`: Timestamped log of emulator activity/state.
- Memory (hex-style, human-readable)
  - `memory_dump.bin`: Hex-dump style snapshot of full memory at key points.
  - `io_table.bin`: Hex-dump style snapshot of the IO port table.
- Raw snapshots (binary)
  - `raw_memory_dump_YYYY_MM_DD_HH_MM_SS.bin`
  - `raw_io_table_dump_YYYY_MM_DD_HH_MM_SS.bin`

Hex dumps are formatted similarly to `xxd`, including ASCII on the right, to make inspection easy.

### Loading your own program
By default, `main.rs` calls:
- `cpu.load_program("cpudiag.bin", 0x100)` and then `cpu.set_pc(0x100)`

To run your own program:
- Put your binary at the project root, e.g., `myprog.bin`.
- Update `main.rs` to load it and choose a start address (PC) that matches how you laid out your program.

Minimal expectations for a program:
- It should fit in memory starting at your chosen load address.
- The emulator writes a `HLT` (0x76) after the loaded program in memory so it will stop if it reaches the end.

### Notes and limitations
- This is a learning tool, not a cycle-accurate or complete 8080 emulator.
- Not every instruction or peripheral behavior may be implemented.
- Logging expects the `logs/` directory to exist before launch.
- Input handling uses single-key reads; some terminals may buffer input differently.

### Development hints
- Instruction dispatch lives in `src/cpu8080/instruction.rs`, which groups opcodes by 0x10 ranges (`execute_instruction_0` … `execute_instruction_f`).
- Flags and registers are in `src/cpu8080/mod.rs` (`ConditionFlags`, `Registers`).
- ALU flag math helpers are in `src/cpu8080/utils.rs` (`alu_operation`).
- File IO utilities are in `src/ioutils.rs`.

### License
See `LICENSE`.
