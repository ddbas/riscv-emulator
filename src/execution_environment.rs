use std::fmt;

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
        self.cpu.cycle(&mut self.bus);
    }

    pub fn inspect_register(&self, register: u8) -> i32 {
        self.cpu.inspect_register(register)
    }
}

impl fmt::Display for ExecutionEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cpu)
    }
}
