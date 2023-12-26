use super::VM;

impl VM {
    pub(crate) fn arithmetic_execute_add(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register_1 + register_2;
    }

    pub(crate) fn arithmetic_execute_sub(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register_1 - register_2;
    }

    pub(crate) fn arithmetic_execute_mul(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register_1 * register_2;
    }

    pub(crate) fn arithmetic_execute_div(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register_1 / register_2;
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
        let register = self.next_8_bits() as usize;
        self.registers[register] += 1;
        self.next_16_bits();
    }

    pub(crate) fn arithmetic_execute_decrement(&mut self) {
        let register = self.next_8_bits() as usize;
        self.registers[register] -= 1;
        self.next_16_bits();
    }
}
