use riscv_emulator::{
    cpu::Cpu,
    device::{Ram, Rom},
    execution_environment::ExecutionEnvironment,
    mmio::MemoryMapping,
};

fn main() {
    let _execution_environment = ExecutionEnvironment::new(
        Cpu {},
        vec![
            MemoryMapping {
                device: Box::new(Rom {}),
                start: 0x00000000,
                end: 0x7FFFFFFF,
            },
            MemoryMapping {
                device: Box::new(Ram {}),
                start: 0x80000000,
                end: 0xFFFFFFFF,
            },
        ],
    );
}
