use crate::vm::virtual_machine::{ExecutionStatus, VirtualMachine};

impl VirtualMachine {
  pub fn logical_execute_and(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];
    self.registers[self.next_8_bits() as usize] = register_1 & register_2;
    ExecutionStatus::Continue
  }

  pub fn logical_execute_or(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];
    self.registers[self.next_8_bits() as usize] = register_1 | register_2;
    ExecutionStatus::Continue
  }

  pub fn logical_execute_xor(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];
    self.registers[self.next_8_bits() as usize] = register_1 ^ register_2;
    ExecutionStatus::Continue
  }

  pub fn logical_execute_not(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    self.registers[self.next_8_bits() as usize] = !register_1;
    self.next_8_bits(); // tick over
    ExecutionStatus::Continue
  }
}
