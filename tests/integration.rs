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
        size: 8,
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
    assert_eq!(
        -2046,
        execution_environment.inspect_register(1),
        "x1 contains -2046"
    );
}

#[test]
fn slti() {
    // Arrange
    let rom_size = 12;
    let mut rom = Rom::new(rom_size);
    rom.load(vec![
        0b00000000, 0b00010000, 0b10000000, 0b10010011, // ADDI 1, x1, x1
        0b10000000, 0b00010000, 0b10100001, 0b00010011, // SLTI x2, x1, -2047
        0b01111111, 0b11110000, 0b10100001, 0b10010011, // SLTI x3, x1, 2047
    ]);
    let bus = SystemInterface {
        size: 12,
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
    execution_environment.cycle();

    // Assert
    assert_eq!(
        0,
        execution_environment.inspect_register(2),
        "x2 contains 0"
    );
    assert_eq!(
        1,
        execution_environment.inspect_register(3),
        "x3 contains 1"
    );
}

#[test]
fn sltiu() {
    // Arrange
    let rom_size = 12;
    let mut rom = Rom::new(rom_size);
    rom.load(vec![
        0b00000000, 0b00010000, 0b10000000, 0b10010011, // ADDI x1, x1, 1
        0b00000000, 0b00010000, 0b00110001, 0b00010011, // SLTIU x2, x0, 1
        0b00000000, 0b00010000, 0b10110001, 0b10010011, // SLTIU x3, x1, 1
    ]);
    let bus = SystemInterface {
        size: 12,
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
    execution_environment.cycle();

    // Assert
    assert_eq!(
        1,
        execution_environment.inspect_register(2),
        "x2 contains 1"
    );
    assert_eq!(
        0,
        execution_environment.inspect_register(3),
        "x3 contains 0"
    );
}
