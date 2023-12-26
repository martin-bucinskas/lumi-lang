use crate::vm::VM;
use log::{debug, error, info};

impl VM {
    pub(crate) fn system_execute_print_string(&mut self) {
        let starting_offset = self.next_24_bits() as usize;
        let mut ending_offset = starting_offset;
        let slice = self.ro_data.as_slice();

        debug!("Starting offset: {}", starting_offset);
        debug!("Ending offset: {}", ending_offset);
        debug!("Slice: {:?}", slice);

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

    /// Calls a subroutine.
    /// Creates a return destination.
    /// Gets the address destination to jump to.
    /// Pushes the return address to stack.
    pub(crate) fn system_execute_call(&mut self) {
        let return_destination = self.pc + 3;
        let destination = self.next_16_bits();
        self.stack.push(return_destination as i32);
        self.stack.push(self.bp as i32);
        self.bp = self.sp;
        self.pc = destination as usize;
    }

    pub(crate) fn system_execute_return(&mut self) {
        self.sp = self.bp;
        self.bp = self.stack.pop().unwrap() as usize;
        self.pc = self.stack.pop().unwrap() as usize;
    }
}
