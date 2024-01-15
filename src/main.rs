use riscv_emulator::{
    Cpu, ExecutionEnvironment, MemoryMapping, RamDevice, RomDevice, SystemInterface,
};

fn main() {
    let rom_memory_mapping = MemoryMapping {
        device: Box::new(RomDevice {}),
        start: 0x00000000,
        end: 0x7FFFFFFF,
    };
    let ram_memory_mapping = MemoryMapping {
        device: Box::new(RamDevice {}),
        start: 0x80000000,
        end: 0xFFFFFFFF,
    };

    let _execution_environment = ExecutionEnvironment {
        cpu: Cpu {},
        system_interface: SystemInterface {
            memory_mappings: vec![rom_memory_mapping, ram_memory_mapping],
        },
    };
}
