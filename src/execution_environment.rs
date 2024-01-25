use crate::{cpu::Cpu, mmio::SystemInterface};

pub struct ExecutionEnvironment {
    cpu: Cpu,
    bus: SystemInterface,
}

impl ExecutionEnvironment {
    pub fn new(cpu: Cpu, bus: SystemInterface) -> Self {
        ExecutionEnvironment { cpu, bus }
    }

    /// Executes a single CPU cycle.
    pub fn cycle(&mut self) {
        self.cpu.execute(&mut self.bus);
    }
}
