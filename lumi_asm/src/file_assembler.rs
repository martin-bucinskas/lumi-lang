use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use log::{debug, error};
use crate::assembler::Assembler;
use crate::assembler_errors::AssemblerError;

type Error = anyhow::Error;

pub fn assemble_file(file_path: &str) -> Result<(), Vec<AssemblerError>> {
  let mut assembler: Assembler = Assembler::new();
  let raw = read_file(file_path)
    .map_err(|err| vec![
      AssemblerError::FailedToReadFile { error: err.to_string() }
    ])?;
  
  let result = assembler.assemble(&raw);
  
  if let Err(errors) = result {
    return Err(errors);
  }
  
  let mut assembled_file_path = strip_extension(file_path);
  assembled_file_path.push_str(".bin");
  
  let binary = result.unwrap();
  create_binary_file(&assembled_file_path, &binary)
    .map_err(|err| vec![
      AssemblerError::FailedToWriteBinaryFile { error: err.to_string() }
    ])?;
  
  Ok(())
}

fn strip_extension(input: &str) -> String {
  if input.ends_with(".lumi") {
    input.strip_suffix(".lumi").unwrap().to_string()
  } else {
    input.to_string()
  }
}

fn create_binary_file(file_name: &str, data: &[u8]) -> std::io::Result<()> {
  let mut file_handle = File::create(file_name)
    .expect("Unable to create a file");
  file_handle
    .write_all(data)
    .expect("Failed to write to file");
  Ok(())
}

fn read_file(tmp: &str) -> Result<String, Error> {
  let filename = Path::new(tmp);
  debug!("Reading file: {}", filename.to_str().unwrap());
  match File::open(&filename) {
    Ok(mut file_handle) => {
      let mut contents = String::new();
      match file_handle.read_to_string(&mut contents) {
        Ok(_) => {
          Ok(contents)
        }
        Err(err) => {
          error!("There was an error reading file: {:?}", err);
          Err(Error::new(err))
        }
      }
    }
    Err(err) => {
      error!("File not found: {:?}", err);
      Err(Error::new(err))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_strip_extension() {
    let result = strip_extension("test.lumi");
    assert_eq!(result, "test");
    
    let result = strip_extension("test");
    assert_eq!(result, "test");
  }
  
  #[test]
  fn test_create_binary_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test-create-file.bin");
    
    let result = create_binary_file(
      file_path.to_str().unwrap(),
      &[0, 1, 2, 3]
    );
    assert_eq!(result.is_ok(), true);
  }
  
  #[test]
  fn test_read_file() {
    let result = read_file("test/test.lumi");
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().is_empty(), false);
  }
  
  #[test]
  fn test_assemble_file() {
    let result = assemble_file("test/test.lumi");
    assert_eq!(result.is_ok(), true);
  }
  
  #[test]
  fn test_assemble_file_not_found() {
    let result = assemble_file("test/test_not_found.lumi");
    assert_eq!(result.is_ok(), false);
  }
  
  #[test]
  fn test_assemble_file_invalid() {
    let result = assemble_file("test/test_invalid.lumi");
    assert_eq!(result.is_ok(), false);
  }
  
  #[test]
  fn test_assemble_file_empty() {
    let result = assemble_file("test/test_empty.lumi");
    assert_eq!(result.is_ok(), false);
  }
  
  #[test]
  fn test_assemble_file_no_extension() {
    let result = assemble_file("test/test");
    assert_eq!(result.is_ok(), true);
  }
}