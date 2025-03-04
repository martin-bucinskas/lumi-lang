use byteorder::{LittleEndian, WriteBytesExt};
use log::debug;

/// Magic number for LUMI programs.
pub const LUMI_HEADER_PREFIX: [u8; 4] = [0x4C, 0x55, 0x4D, 0x49];
/// Length of the LUMI header.
pub const LUMI_HEADER_LENGTH: usize = 64;

/// Get the header for a LUMI program.
pub fn get_lumi_header(read_only_data_length: usize) -> Vec<u8> {
  let mut header = vec![];
  for byte in LUMI_HEADER_PREFIX.into_iter() {
    header.push(byte.clone());
  }
  
  while header.len() <= LUMI_HEADER_LENGTH {
    header.push(0x11u8);
  }
  
  // calculate and write the starting offset for the VM to know where the RO section ends
  debug!("Header: {:?}", header);
  debug!("RO Length: {}", read_only_data_length);
  let mut wtr: Vec<u8> = vec![];
  wtr.write_u32::<LittleEndian>(read_only_data_length as u32)
      .unwrap();
  for byte in &wtr {
    debug!("Written offset bytes: {:02X}", byte);
  }
  
  header.append(&mut wtr);
  debug!("Header: {:?}", header);
  header
}

/// Verify the header of a LUMI program.
pub fn verify_header(program: &Vec<u8>) -> bool {
  if program[0..4] != LUMI_HEADER_PREFIX {
    return false;
  }

  true
}
