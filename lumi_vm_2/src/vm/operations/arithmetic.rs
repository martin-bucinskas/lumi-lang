use log::debug;
use crate::vm::virtual_machine::{ExecutionStatus, VirtualMachine};

impl VirtualMachine {
  pub fn arithmetic_execute_add(&mut self) -> ExecutionStatus {
    let first_byte = self.next_8_bits();
    let second_byte = self.next_8_bits();
    let third_byte = self.next_8_bits();
    
    debug!("ADD ${} ${} ${}", first_byte, second_byte, third_byte);
    let register_1 = self.registers[first_byte as usize];
    let register_2 = self.registers[second_byte as usize];
    self.registers[third_byte as usize] = register_1 + register_2;
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_sub(&mut self) -> ExecutionStatus {
    let first_byte = self.next_8_bits();
    let second_byte = self.next_8_bits();
    let third_byte = self.next_8_bits();
    
    debug!("SUB ${} ${} ${}", first_byte, second_byte, third_byte);
    let register_1 = self.registers[first_byte as usize];
    let register_2 = self.registers[second_byte as usize];
    self.registers[third_byte as usize] = register_1 - register_2;
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_mul(&mut self) -> ExecutionStatus {
    let first_byte = self.next_8_bits();
    let second_byte = self.next_8_bits();
    let third_byte = self.next_8_bits();
    
    debug!("MUL ${} ${} ${}", first_byte, second_byte, third_byte);
    let register_1 = self.registers[first_byte as usize];
    let register_2 = self.registers[second_byte as usize];
    self.registers[third_byte as usize] = register_1 * register_2;
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_div(&mut self) -> ExecutionStatus {
    let first_byte = self.next_8_bits();
    let second_byte = self.next_8_bits();
    let third_byte = self.next_8_bits();
    
    debug!("DIV ${} ${} ${}", first_byte, second_byte, third_byte);
    let register_1 = self.registers[first_byte as usize];
    let register_2 = self.registers[second_byte as usize];
    self.registers[third_byte as usize] = register_1 / register_2;
    self.remainder = (register_1 % register_2) as u32;
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_add_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];
    
    debug!("ADDF64 ${} ${}", register_1, register_2);
    self.float_registers[self.next_8_bits() as usize] = register_1 + register_2;
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_sub_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];
    
    debug!("SUBF64 ${} ${}", register_1, register_2);
    self.float_registers[self.next_8_bits() as usize] = register_1 - register_2;
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_mul_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];
    
    debug!("MULF64 ${} ${}", register_1, register_2);
    self.float_registers[self.next_8_bits() as usize] = register_1 * register_2;
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_div_f64(&mut self) -> ExecutionStatus {
    let register_1 = self.float_registers[self.next_8_bits() as usize];
    let register_2 = self.float_registers[self.next_8_bits() as usize];
    
    debug!("DIVF64 ${} ${}", register_1, register_2);
    self.float_registers[self.next_8_bits() as usize] = register_1 / register_2;
    self.remainder = (register_1 % register_2) as u32;
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_increment(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    
    debug!("INC ${}", register);
    self.registers[register] += 1;
    self.next_16_bits();
    ExecutionStatus::Continue
  }
  
  pub fn arithmetic_execute_decrement(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    
    debug!("DEC ${}", register);
    self.registers[register] -= 1;
    self.next_16_bits();
    ExecutionStatus::Continue
  }
}