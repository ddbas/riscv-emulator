use crate::{cpu::Cpu, mmio::SystemInterface};

pub struct ExecutionEnvironment {
    _cpu: Cpu,
    _bus: SystemInterface,
}

impl ExecutionEnvironment {
    pub fn new(cpu: Cpu, bus: SystemInterface) -> Self {
        ExecutionEnvironment {
            _cpu: cpu,
            _bus: bus,
        }
    }

    /// Executes a single CPU cycle.
    pub fn cycle(&mut self) {
        todo!();
    }
}
