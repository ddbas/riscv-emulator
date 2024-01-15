use crate::{
    cpu::Cpu,
    mmio::{MemoryMapping, SystemInterface},
};

pub struct ExecutionEnvironment {
    _cpu: Cpu,
    _bus: SystemInterface,
}

impl ExecutionEnvironment {
    pub fn new(cpu: Cpu, mappings: Vec<MemoryMapping>) -> Self {
        ExecutionEnvironment {
            _cpu: cpu,
            _bus: SystemInterface { mappings },
        }
    }
}
