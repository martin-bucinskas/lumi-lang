use log::{error, info};
use crate::vm::virtual_machine::{VirtualMachine, WatchType, WatchVariable};

impl VirtualMachine {
  pub fn add_watch_variable(&mut self, watch_type: WatchType) {
    let initial_value = match watch_type {
      WatchType::Memory(addr) => self.heap[addr] as f32,
      WatchType::Register(index) => self.registers[index] as f32,
      WatchType::FloatRegister(index) => self.float_registers[index] as f32,
    };

    self.watch_variables.insert(
      watch_type.clone(),
      WatchVariable {
        watch_type,
        last_value: initial_value,
      },
    );
  }
  
  pub fn system_execute_print_string(&mut self) {
    let starting_offset = self.next_24_bits() as usize;
    let mut ending_offset = starting_offset;
    let slice = self.ro_data.as_slice();

    while slice[ending_offset] != 0 {
      ending_offset += 1;
    }

    let result = std::str::from_utf8(&slice[starting_offset..ending_offset]);
    match result {
      Ok(s) => {
        info!("{}", s);
      }
      Err(e) => {
        error!("Error decoding string for prts instruction: {:#?}", e);
      }
    }
  }
  
  pub fn system_execute_call(&mut self) {
    let return_destination = self.pc + 3;
    let destination = self.next_16_bits();
    self.stack.push(return_destination as i32);
    self.stack.push(self.bp as i32);
    self.bp = self.sp;
    self.pc = destination as usize;
  }
  
  pub fn system_execute_return(&mut self) {
    let bp = self.bp;
    self.sp = bp;
    self.bp = self.stack.pop().unwrap() as usize;
    self.pc = self.stack.pop().unwrap() as usize;
  }
  
  pub fn system_execute_breakpoint(&mut self) {
    
  }
}
