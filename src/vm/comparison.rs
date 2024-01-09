use crate::vm::VM;

impl VM {
    pub(crate) fn comparison_execute_equal(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];

        if register_1 == register_2 {
            self.equal_flag = true;
        } else {
            self.equal_flag = false;
        }
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_not_equal(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];

        if register_1 != register_2 {
            self.equal_flag = true;
        } else {
            self.equal_flag = false;
        }
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_greater_than(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];

        if register_1 > register_2 {
            self.equal_flag = true;
        } else {
            self.equal_flag = false;
        }
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_less_than(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];

        if register_1 < register_2 {
            self.equal_flag = true;
        } else {
            self.equal_flag = false;
        }
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_greater_than_or_equal(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];

        if register_1 >= register_2 {
            self.equal_flag = true;
        } else {
            self.equal_flag = false;
        }
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_less_than_or_equal(&mut self) {
        let register_1 = self.registers[self.next_8_bits() as usize];
        let register_2 = self.registers[self.next_8_bits() as usize];

        if register_1 <= register_2 {
            self.equal_flag = true;
        } else {
            self.equal_flag = false;
        }
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_equal_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.equal_flag = (register_1 - register_2).abs() < f64::EPSILON;
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_not_equal_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.equal_flag = (register_1 - register_2).abs() > f64::EPSILON;
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_greater_than_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.equal_flag = register_1 > register_2;
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_less_than_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.equal_flag = register_1 < register_2;
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_greater_than_or_equal_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.equal_flag = register_1 >= register_2;
        self.next_8_bits(); // tick over to complete instruction
    }

    pub(crate) fn comparison_execute_less_than_or_equal_f64(&mut self) {
        let register_1 = self.float_registers[self.next_8_bits() as usize];
        let register_2 = self.float_registers[self.next_8_bits() as usize];
        self.equal_flag = register_1 <= register_2;
        self.next_8_bits(); // tick over to complete instruction
    }
}
