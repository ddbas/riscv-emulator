use riscv_emulator::{
    cpu::Cpu,
    device::{Ram, Rom},
    execution_environment::ExecutionEnvironment,
    mmio::{MemoryMapping, SystemInterface},
};

fn main() {
    let rom_size = 1024 * 1024; // 1MB
    let ram_size = 1024 * 1024; // 1MB

    let mut rom = Rom::new(rom_size);
    rom.load(vec![
        0b00000000, 0b00010000, 0b00000000, 0b10010011, // ADDI 1, x0, x1
        0b00000000, 0b00010000, 0b10000000, 0b10010011, // ADDI 1, x1, x1
        0b00000000, 0b00010000, 0b10000000, 0b10010011, // ADDI 1, x1, x1
    ]);
    let ram = Ram::new(ram_size);
    let bus = SystemInterface {
        size: 2_usize.pow(32),
        mappings: vec![
            MemoryMapping {
                device: Box::new(rom),
                start: 0x00000000,
                end: rom_size - 1,
            },
            MemoryMapping {
                device: Box::new(ram),
                start: rom_size,
                end: rom_size + ram_size - 1,
            },
        ],
    };
    let mut execution_environment = ExecutionEnvironment::new(Cpu::default(), bus);
    println!("{}", execution_environment);
    execution_environment.cycle();
    println!("{}", execution_environment);
    execution_environment.cycle();
    println!("{}", execution_environment);
    execution_environment.cycle();
    println!("{}", execution_environment);
}
