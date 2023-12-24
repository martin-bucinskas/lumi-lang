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
}
