use riscv_emulator::{
    cpu::Cpu,
    device::Rom,
    execution_environment::ExecutionEnvironment,
    mmio::{MemoryMapping, SystemInterface},
};

#[test]
fn addi() {
    // Arrange
    let rom_size = 8;
    let mut rom = Rom::new(rom_size);
    rom.load(vec![
        0b10000000, 0b00010000, 0b00000000, 0b10010011, // ADDI -2047, x0, x1
        0b00000000, 0b00010000, 0b10000000, 0b10010011, // ADDI 1, x1, x1
    ]);
    let bus = SystemInterface {
        size: 2_usize.pow(32),
        mappings: vec![MemoryMapping {
            device: Box::new(rom),
            start: 0x00000000,
            end: rom_size - 1,
        }],
    };
    let mut execution_environment = ExecutionEnvironment::new(Cpu::default(), bus);

    // Act
    execution_environment.cycle();
    execution_environment.cycle();

    // Assert
    assert_eq!(-2046, execution_environment.inspect_register(1));
}
