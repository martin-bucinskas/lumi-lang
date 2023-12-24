use crate::vm::VM;
use log::debug;

impl VM {
    pub(crate) fn memory_execute_load(&mut self) {
        let register = self.next_8_bits() as usize;
        let number = self.next_16_bits() as u16;
        debug!("LOAD ${} #{}", register, number);
        self.registers[register] = number as i32;
    }

    pub(crate) fn memory_execute_allocate(&mut self) {
        let register = self.next_8_bits() as usize;
        let bytes = self.registers[register];
        let new_end = self.heap.len() as i32 + bytes;
        self.heap.resize(new_end as usize, 0);
    }
}
