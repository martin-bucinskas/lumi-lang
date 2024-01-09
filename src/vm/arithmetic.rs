use super::VM;
use log::debug;

impl VM {
    pub(crate) fn arithmetic_execute_add(&mut self) {
        let first_byte = self.next_8_bits();
        let second_byte = self.next_8_bits();
        let third_byte = self.next_8_bits();
        debug!("[ADD ${} ${} ${}]", first_byte, second_byte, third_byte);
        let register_1 = self.registers[first_byte as usize];
        let register_2 = self.registers[second_byte as usize];
        self.registers[third_byte as usize] = register_1 + register_2;
    }

    pub(crate) fn arithmetic_execute_sub(&mut self) {
        let first_byte = self.next_8_bits();
        let second_byte = self.next_8_bits();
        let third_byte = self.next_8_bits();
        debug!("[SUB ${} ${} ${}]", first_byte, second_byte, third_byte);
        let register_1 = self.registers[first_byte as usize];
        let register_2 = self.registers[second_byte as usize];
        self.registers[third_byte as usize] = register_1 - register_2;
    }

    pub(crate) fn arithmetic_execute_mul(&mut self) {
        let first_byte = self.next_8_bits();
        let second_byte = self.next_8_bits();
        let third_byte = self.next_8_bits();
        debug!("[MUL ${} ${} ${}]", first_byte, second_byte, third_byte);
        let register_1 = self.registers[first_byte as usize];
        let register_2 = self.registers[second_byte as usize];
        self.registers[third_byte as usize] = register_1 * register_2;
    }

    pub(crate) fn arithmetic_execute_div(&mut self) {
        let first_byte = self.next_8_bits();
        let second_byte = self.next_8_bits();
        let third_byte = self.next_8_bits();
        debug!("[DIV ${} ${} ${}]", first_byte, second_byte, third_byte);
        let register_1 = self.registers[first_byte as usize];
        let register_2 = self.registers[second_byte as usize];
        self.registers[third_byte as usize] = register_1 / register_2;
        self.remainder = (register_1 % register_2) as u32;
    }

    pub(crate) fn arithmetic_execute_add_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.float_registers[self.next_8_bits() as usize] = register_1 + register_2;
    }

    pub(crate) fn arithmetic_execute_sub_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.float_registers[self.next_8_bits() as usize] = register_1 - register_2;
    }

    pub(crate) fn arithmetic_execute_mul_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.float_registers[self.next_8_bits() as usize] = register_1 * register_2;
    }

    pub(crate) fn arithmetic_execute_div_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.float_registers[self.next_8_bits() as usize] = register_1 / register_2;
        self.remainder = (register_1 % register_2) as u32;
    }

    pub(crate) fn arithmetic_execute_increment(&mut self) {
        let first_byte = self.next_8_bits();
        debug!("[INC ${}]", first_byte);
        let register = first_byte as usize;
        self.registers[register] += 1;
        self.next_16_bits();
    }

    pub(crate) fn arithmetic_execute_decrement(&mut self) {
        let first_byte = self.next_8_bits();
        debug!("[DEC ${}]", first_byte);
        let register = first_byte as usize;
        self.registers[register] -= 1;
        self.next_16_bits();
    }
}
