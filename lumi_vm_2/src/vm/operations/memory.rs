use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::{debug, error};
use crate::vm::virtual_machine::{ExecutionStatus, VirtualMachine};

impl VirtualMachine {
  
  // TODO: potentially create/calculate memory map of a program before running it
  //   to perform bounds checking on memory access
  pub fn system_safe_memory_access_range(&mut self, start: usize, len: usize) -> Option<&mut [u8]> {
    // TODO: benchmark this to see how much impact this has on performance
    if start < self.heap.len() && start + len <= self.heap.len() {
      Some(&mut self.heap[start..start + len])
    } else {
      None
    }
  }
  
  pub fn memory_execute_load(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    let integer_immediate = self.next_16_bits();
    
    debug!("LOAD ${} #{}", register, integer_immediate);
    self.registers[register] = integer_immediate as i32;
    ExecutionStatus::Continue
  }
  
  pub fn memory_execute_load_f64(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    let float_immediate = f64::from(self.next_16_bits());
   
    debug!("LOADF64 ${} #{}", register, float_immediate);
    self.float_registers[register] = float_immediate;
    ExecutionStatus::Continue
  }
  
  pub fn memory_execute_allocate(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    let bytes = self.registers[register];
    
    debug!("ALOC ${}", register);
    let new_end = self.heap.len() as i32 + bytes;
    self.heap.resize(new_end as usize, 0);
    self.next_8_bits();
    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn memory_execute_load_upper_immediate(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    let value = self.registers[register];
    let uv1 = i32::from(self.next_8_bits());
    let uv2 = i32::from(self.next_8_bits());
    let mut value = value.checked_shl(8).unwrap();
    value = value | uv1;
    value = value.checked_shl(8).unwrap();
    value = value | uv2;
    
    debug!("LOADUI ${} #{}", register, value);
    self.registers[register] = value;
    ExecutionStatus::Continue
  }

  pub fn memory_execute_load_memory(&mut self) -> ExecutionStatus {
    let offset = self.registers[self.next_8_bits() as usize] as usize;

    if let Some(value) = self.system_safe_memory_access_range(offset, 4) {
      let mut cursor = Cursor::new(value);
      let data = cursor.read_i32::<LittleEndian>().unwrap();

      debug!("LOADM ${}", data);
      self.registers[self.next_8_bits() as usize] = data;
      self.next_8_bits();
      ExecutionStatus::Continue
    } else {
      debug!("Memory access out of bounds for LOADM at offset {}", offset);
      ExecutionStatus::Crash(10)
    }
  }

  pub fn memory_execute_set_memory(&mut self) -> ExecutionStatus {
    let offset_register = self.next_8_bits();
    let data_register = self.next_8_bits();
    let offset = self.registers[offset_register as usize] as usize;
    let data = self.registers[data_register as usize];

    debug!("SETM ${} ${}", offset_register, data_register);
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    buf.as_mut()
      .write_i32::<LittleEndian>(data)
      .expect("failed to write to buffer");

    if let Some(range) = self.system_safe_memory_access_range(offset, 4) {
      range.copy_from_slice(&buf);
    } else {
      error!("Memory access out of bounds: offset={}, size=4", offset);
      return ExecutionStatus::Crash(10);
    }

    self.next_8_bits();
    ExecutionStatus::Continue
  }
  
  pub fn memory_execute_push_to_stack(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    let value = self.registers[register];
    
    debug!("PUSH ${}", register);
    self.stack.push(value);
    self.sp += 1;
    self.next_16_bits();
    ExecutionStatus::Continue
  }
  
  pub fn memory_execute_pop_from_stack(&mut self) -> ExecutionStatus {
    let register = self.next_8_bits() as usize;
    
    debug!("POP ${}", register);
    self.sp -= 1;
    self.registers[register] = self.stack.pop().unwrap();
    self.next_16_bits();
    ExecutionStatus::Continue
  }
}
