use crate::vm::VM;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::debug;
use nom::character::complete::i32;
use std::io::Write;

impl VM {
    pub(crate) fn memory_execute_load(&mut self) {
        let register = self.next_8_bits() as usize;
        let number = self.next_16_bits() as u16;
        debug!("LOAD ${} #{}", register, number);
        self.registers[register] = number as i32;
    }

    pub(crate) fn memory_execute_load_f64(&mut self) {
        let register = self.next_8_bits() as usize;
        let number = f64::from(self.next_16_bits());
        self.float_registers[register] = number;
    }

    pub(crate) fn memory_execute_allocate(&mut self) {
        let register = self.next_8_bits() as usize;
        let bytes = self.registers[register];
        let new_end = self.heap.len() as i32 + bytes;
        self.heap.resize(new_end as usize, 0);
    }

    pub(crate) fn memory_execute_load_upper_immediate(&mut self) {
        let register = self.next_8_bits() as usize;
        let value = self.registers[register];
        let uv1 = i32::from(self.next_8_bits());
        let uv2 = i32::from(self.next_8_bits());
        let mut value = value.checked_shl(8).unwrap();
        value = value | uv1;
        value = value.checked_shl(8).unwrap();
        value = value | uv2;
        self.registers[register] = value;
    }

    pub(crate) fn memory_execute_load_memory(&mut self) {
        let offset = self.registers[self.next_8_bits() as usize] as usize;

        let mut slice = &self.heap[offset..offset + 4];
        let data = slice.read_i32::<LittleEndian>().unwrap();
        self.registers[self.next_8_bits() as usize] = data;
        self.next_8_bits();
    }

    pub(crate) fn memory_execute_set_memory(&mut self) {
        let _offset = self.registers[self.next_8_bits() as usize] as usize;
        let data = self.registers[self.next_8_bits() as usize];
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        buf.as_mut()
            .write_i32::<LittleEndian>(data)
            .expect("failed to write to buffer");
        // self.heap.append(&buf);
        for byte in buf {
            self.heap.push(byte);
            // self.heap.push_within_capacity(byte); // safer?
        }
    }

    pub(crate) fn memory_execute_push_to_stack(&mut self) {
        let data = self.registers[self.next_8_bits() as usize];
        self.stack.push(data);
        self.sp = self.sp + 1;
    }

    pub(crate) fn memory_execute_pop_from_stack(&mut self) {
        let target_register = self.next_8_bits() as usize;
        self.registers[target_register] = self.stack.pop().unwrap();
        self.sp = self.sp - 1;
    }
}
