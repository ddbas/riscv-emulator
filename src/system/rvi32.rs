use crate::{
    cpu::Cpu,
    device::{Ram, Rom},
    mmio::{MemoryMapping, SystemInterface},
};

use super::ExecutionEnvironment;

pub struct RvI32System {
    _cpu: Cpu,
    _system_interface: SystemInterface,
}

impl RvI32System {
    pub fn new() -> Self {
        let rom_memory_mapping = MemoryMapping {
            device: Box::new(Rom {}),
            start: 0x00000000,
            end: 0x7FFFFFFF,
        };
        let ram_memory_mapping = MemoryMapping {
            device: Box::new(Ram {}),
            start: 0x80000000,
            end: 0xFFFFFFFF,
        };

        RvI32System {
            _cpu: Cpu {},
            _system_interface: SystemInterface {
                memory_mappings: vec![rom_memory_mapping, ram_memory_mapping],
            },
        }
    }
}

impl ExecutionEnvironment for RvI32System {}
