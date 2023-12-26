use crate::vm::VM;

impl VM {
    pub(crate) fn bitwise_execute_shift_left(&mut self) {
        let reg_number = self.next_8_bits() as usize;
        let shift_left_by = match self.next_8_bits() {
            0 => 16,
            other => other,
        };
        self.registers[reg_number] = self.registers[reg_number].wrapping_shl(shift_left_by.into());
        self.next_8_bits();
    }

    pub(crate) fn bitwise_execute_shift_right(&mut self) {
        let reg_number = self.next_8_bits() as usize;
        let shift_right_by = match self.next_8_bits() {
            0 => 16,
            other => other,
        };
        self.registers[reg_number] = self.registers[reg_number].wrapping_shr(shift_right_by.into());
        self.next_8_bits();
    }
}
