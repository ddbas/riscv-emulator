use riscv_emulator::{
    cpu::Cpu,
    device::{Ram, Rom},
    execution_environment::ExecutionEnvironment,
    mmio::{MemoryMapping, SystemInterface},
};

fn main() {
    let rom_size = 1024 * 1024; // 1MB
    let ram_size = 1024 * 1024; // 1MB

    let rom = Rom::new(rom_size);
    let ram = Ram::new(ram_size);
    let bus = SystemInterface {
        size: 2_usize.pow(32),
        mappings: vec![
            MemoryMapping {
                device: Box::new(rom),
                start: 0x00000000,
                end: 0x00000000 + rom_size - 1,
            },
            MemoryMapping {
                device: Box::new(ram),
                start: 0x00000000 + rom_size,
                end: 0x00000000 + rom_size + ram_size - 1,
            },
        ],
    };
    let mut execution_environment = ExecutionEnvironment::new(Cpu::new(), bus);
    execution_environment.cycle();
}
