use byteorder::{LittleEndian, WriteBytesExt};
use log::debug;
use crate::vm::virtual_machine::VirtualMachine;

impl VirtualMachine {
  pub fn memory_execute_load(&mut self) {
    let register = self.next_8_bits() as usize;
    let integer_immediate = self.next_16_bits();
    
    debug!("LOAD ${} #{}", register, integer_immediate);
    self.registers[register] = integer_immediate as i32;
  }
  
  pub fn memory_execute_load_f64(&mut self) {
    let register = self.next_8_bits() as usize;
    let float_immediate = f64::from(self.next_16_bits());
   
    debug!("LOADF64 ${} #{}", register, float_immediate);
    self.float_registers[register] = float_immediate;
  }
  
  pub fn memory_execute_allocate(&mut self) {
    let register = self.next_8_bits() as usize;
    let bytes = self.registers[register];
    
    debug!("ALOC ${}", register);
    let new_end = self.heap.len() as i32 + bytes;
    self.heap.resize(new_end as usize, 0);
    self.next_8_bits();
    self.next_8_bits();
  }
  
  pub fn memory_execute_load_upper_immediate(&mut self) {
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
  }
  
  pub fn memory_execute_load_memory(&mut self) {
    let offset = self.registers[self.next_8_bits() as usize] as usize;
    let mut slice = &self.heap[offset..offset + 4];
    let data = slice.read_i32::<LittleEndian>().unwrap();
    
    debug!("LOADM ${}", data);
    self.registers[self.next_8_bits() as usize] = data;
    self.next_8_bits();
  }
  
  pub fn memory_execute_set_memory(&mut self) {
    let offset_register = self.next_8_bits();
    let data_register = self.next_8_bits();
    let offset = self.registers[offset_register as usize] as usize;
    let data = self.registers[data_register as usize];
    
    debug!("SETM ${} ${}", offset_register, data_register);
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    buf.as_mut()
      .write_i32::<LittleEndian>(data)
      .expect("failed to write to buffer");
    // todo: can I make this more memory safe by using `self.heap.push_within_capacity(byte);`?
    self.heap[offset..offset + 4].copy_from_slice(&buf);
    self.next_8_bits();
  }
  
  pub fn memory_execute_push_to_stack(&mut self) {
    let register = self.next_8_bits() as usize;
    let value = self.registers[register];
    
    debug!("PUSH ${}", register);
    self.stack.push(value);
    self.sp += 1;
    self.next_16_bits();
  }
  
  pub fn memory_execute_pop_from_stack(&mut self) {
    let register = self.next_8_bits() as usize;
    
    debug!("POP ${}", register);
    self.sp -= 1;
    self.registers[register] = self.stack.pop().unwrap();
    self.next_16_bits();
  }
}
