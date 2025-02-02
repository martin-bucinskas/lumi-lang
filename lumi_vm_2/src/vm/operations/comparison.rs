use crate::vm::virtual_machine::{ExecutionStatus, VirtualMachine};

impl VirtualMachine {
  pub fn comparison_execute_equal(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 == register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_not_equal(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 != register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_greater_than(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 > register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_less_than(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 < register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_greater_than_or_equal(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 >= register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_less_than_or_equal(&mut self) -> ExecutionStatus {
    let register_1 = self.registers[self.next_8_bits() as usize];
    let register_2 = self.registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 <= register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_equal_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];

    self.equal_flag = (register_1 - register_2).abs() < f64::EPSILON;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_not_equal_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];

    self.equal_flag = (register_1 - register_2).abs() > f64::EPSILON;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_greater_than_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 > register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_less_than_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 < register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_greater_than_or_equal_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 >= register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn comparison_execute_less_than_or_equal_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];

    self.equal_flag = register_1 <= register_2;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
}