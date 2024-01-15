use riscv_emulator::{
    cpu::Cpu,
    device::{Ram, Rom},
    execution_environment::ExecutionEnvironment,
    mmio::{MemoryMapping, SystemInterface},
};

fn main() {
    let rom = Rom {};
    let ram = Ram {};
    let bus = SystemInterface {
        mappings: vec![
            MemoryMapping {
                device: Box::new(rom),
                start: 0x00000000,
                end: 0x7FFFFFFF,
            },
            MemoryMapping {
                device: Box::new(ram),
                start: 0x80000000,
                end: 0xFFFFFFFF,
            },
        ],
    };
    let _execution_environment = ExecutionEnvironment::new(
        Cpu {},
        bus,
    );
}
