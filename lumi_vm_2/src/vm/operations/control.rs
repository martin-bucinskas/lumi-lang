use crate::vm::virtual_machine::{ExecutionStatus, VirtualMachine};

impl VirtualMachine {
  pub fn control_execute_jump(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    let target = self.registers[register];
    self.pc = target as usize;
    ExecutionStatus::Continue
  }

  pub fn control_execute_jump_forward(&mut self) -> ExecutionStatus {
    let value = self.registers[self.next_8_bits() as usize] as usize;
    self.pc += value;
    ExecutionStatus::Continue
  }

  pub fn control_execute_jump_backward(&mut self) -> ExecutionStatus {
    let value = self.registers[self.next_8_bits() as usize] as usize;
    self.pc -= value;
    ExecutionStatus::Continue
  }

  pub fn control_execute_jump_if_equal(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    let target = self.registers[register];
    if self.equal_flag {
      self.pc = target as usize;
    } else {
      self.next_16_bits();
    }
    ExecutionStatus::Continue
  }

  pub fn control_execute_direct_jump(&mut self) -> ExecutionStatus {
    let destination = self.next_16_bits();
    self.pc = destination as usize;
    ExecutionStatus::Continue
  }

  pub fn control_execute_direct_jump_if_equal(&mut self) -> ExecutionStatus {
    let destination = self.next_16_bits();
    if self.equal_flag {
      self.pc = destination as usize;
    } else {
      self.next_8_bits();
    }
    ExecutionStatus::Continue
  }
  
  pub fn control_execute_loop(&mut self) -> ExecutionStatus {
    if self.loop_counter != 0 {
      self.loop_counter -= 1;
      let target = self.next_16_bits();
      self.pc = target as usize;
    } else {
      self.pc += 3;
    }
    ExecutionStatus::Continue
  }
  
  pub fn control_execute_create_loop(&mut self) -> ExecutionStatus{
    let loop_count = self.next_16_bits();
    self.loop_counter = loop_count as usize;
    self.next_8_bits();
    ExecutionStatus::Continue
  }
}
