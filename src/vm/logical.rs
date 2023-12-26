use crate::vm::VM;

impl VM {
    /// AND $1 $2 $3
    pub(crate) fn logical_execute_and(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register_1 & register_2;
    }

    pub(crate) fn logical_execute_or(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register_1 | register_2;
    }

    pub(crate) fn logical_execute_xor(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register_1 ^ register_2;
    }

    pub(crate) fn logical_execute_not(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = !register_1;
        self.next_8_bits(); // tick over
    }
}
