#[derive(Debug)]
pub enum SymbolType {
  Label,
  Integer,
  LString,
}

#[derive(Debug)]
pub struct Symbol {
  name: String,
  offset: Option<u32>,
  symbol_type: SymbolType,
}

impl Symbol {
  pub fn new(name: String, symbol_type: SymbolType) -> Symbol {
    Symbol {
      name,
      symbol_type,
      offset: None,
    }
  }
  
  pub fn new_with_offset(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
    Symbol {
      name,
      symbol_type,
      offset: Some(offset),
    }
  }
}

pub struct SymbolTable {
  // todo: convert this to a table structure!
  symbols: Vec<Symbol>,
}

impl SymbolTable {
  pub fn new() -> SymbolTable {
    SymbolTable { symbols: vec![] }
  }
  
  pub fn add_symbol(&mut self, symbol: Symbol) {
    self.symbols.push(symbol);
  }
  
  pub fn get_symbols(&mut self) -> &Vec<Symbol> {
    &self.symbols
  }
  
  pub fn has_symbol(&self, s: &str) -> bool {
    for symbol in &self.symbols {
      if symbol.name == s {
        return true;
      }
    }
    false
  }
  
  pub fn set_symbol_offset(&mut self, s: &str, offset: u32) -> bool {
    for symbol in &mut self.symbols {
      if symbol.name == s {
        symbol.offset = Some(offset);
        return true;
      }
    }
    false
  }
  
  pub fn symbol_value(&self, s: &str) -> Option<u32> {
    for symbol in &self.symbols {
      if symbol.name == s {
        return symbol.offset;
      }
    }
    None
  }
}
