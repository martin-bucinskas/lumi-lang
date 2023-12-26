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

    pub(crate) fn control_execute_loop(&mut self) {
        if self.loop_counter != 0 {
            self.loop_counter -= 1;
            let target = self.next_16_bits();
            self.pc = target as usize;
        } else {
            self.pc += 3;
        }
    }

    pub(crate) fn control_execute_create_loop(&mut self) {
        let loop_count = self.next_16_bits();
        self.loop_counter = loop_count as usize;
        self.next_8_bits();
    }
}
