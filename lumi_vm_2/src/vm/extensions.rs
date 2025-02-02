use std::error::Error;
use std::fs;
use libloading::{Library, Symbol};
use lumi_vm_sdk::LumiVmPlugin;

pub fn load_extensions(path: &str) -> Vec<Box<dyn LumiVmPlugin>> {
  let mut extensions = vec![];
  if let Ok(entries) = fs::read_dir(path) {
    for entry in entries {
      if let Ok(file) = entry {
        if let Some(ext) = load_extension(file.path().to_str().unwrap()).ok() {
          extensions.push(ext);
        }
      }
    }
  }
  
  extensions
}

fn load_extension(path: &str) -> Result<Box<dyn LumiVmPlugin>, Box<dyn Error>> {
  unsafe {
    let lib = Library::new(path)?;
    let on_load_fn: Symbol<extern "C" fn() -> Box<dyn LumiVmPlugin>> = lib.get(b"on_load")?;
    let extension = on_load_fn();
    Ok(extension)
  }
}