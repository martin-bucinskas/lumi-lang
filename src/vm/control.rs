use crate::vm::VM;

impl VM {
    pub(crate) fn control_execute_jump(&mut self) {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc = target as usize;
    }

    pub(crate) fn control_execute_jump_forward(&mut self) {
        let value = self.registers[self.next_8_bits() as usize] as usize;
        self.pc += value;
    }

    pub(crate) fn control_execute_jump_backward(&mut self) {
        let value = self.registers[self.next_8_bits() as usize] as usize;
        self.pc -= value;
    }

    pub(crate) fn control_execute_jump_if_equal(&mut self) {
        let register = self.next_8_bits() as usize;
        let target = self.registers[register];
        if self.equal_flag {
            self.pc = target as usize;
        }
        self.next_16_bits();
    }

    pub(crate) fn control_execute_direct_jump(&mut self) {
        let destination = self.next_16_bits();
        self.pc = destination as usize;
    }

    pub(crate) fn control_execute_direct_jump_if_equal(&mut self) {
        let destination = self.next_16_bits();
        if self.equal_flag {
            self.pc = destination as usize;
        } else {
            self.next_8_bits();
        }
    }
}
