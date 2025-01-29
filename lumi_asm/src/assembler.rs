use byteorder::{LittleEndian, WriteBytesExt};
use colored::Colorize;
use log::{debug, error, info};
use nom::error::{VerboseError, VerboseErrorKind};
use crate::assembler_errors::AssemblerError;
use crate::header_utils::{get_lumi_header, LUMI_HEADER_LENGTH};
use crate::instruction::Opcode;
use crate::parser_combinators::instruction_parser::AssemblerInstruction;
use crate::parser_combinators::program_parser::{parse_program, Program};
use crate::symbols::{Symbol, SymbolTable, SymbolType};

#[derive(Debug, PartialEq)]
pub enum Token {
  Op { code: Opcode },
  Register { reg_num: u8 },
  IntegerOperand { value: i32 },
  FloatOperand { value: f64 },
  LabelDeclaration { name: String },
  LabelUsage { name: String },
  Directive { name: String },
  LString { value: String },
  Separator,
  Comment,
}

#[derive(Debug, PartialEq)]
pub enum AssemblerPhase {
  First,
  Second,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerSection {
  Data { starting_instruction: Option<u32> },
  Code { starting_instruction: Option<u32> },
  Unknown,
}

impl Default for AssemblerSection {
  fn default() -> Self {
    AssemblerSection::Unknown
  }
}

impl<'a> From<&'a str> for AssemblerSection {
  fn from(s: &'a str) -> Self {
    match s {
      "data" => AssemblerSection::Data { starting_instruction: None },
      "code" => AssemblerSection::Code { starting_instruction: None },
      _ => AssemblerSection::Unknown,
    }
  }
}

pub struct Assembler {
  /// Tracks which phase the assembler is in
  pub phase: AssemblerPhase,
  /// Symbol table for constants and variables
  pub symbols: SymbolTable,
  /// Read-only section where constants are placed
  pub ro: Vec<u8>,
  /// Compiled bytecode generated from the assembly code
  pub bytecode: Vec<u8>,
  /// Offset of the read-only section
  ro_offset: u32,
  /// List of all the sections in code
  sections: Vec<AssemblerSection>,
  /// Current section being processed
  current_section: Option<AssemblerSection>,
  /// Current instruction being processed
  current_instruction: u32,
  /// Errors encountered during assembly
  errors: Vec<AssemblerError>,
  /// Scratch buffer
  buf: [u8; 4],
}

impl Assembler {
  pub fn new() -> Self {
    Self {
      phase: AssemblerPhase::First,
      symbols: SymbolTable::new(),
      ro: Vec::new(),
      bytecode: Vec::new(),
      ro_offset: 0,
      sections: Vec::new(),
      current_section: None,
      current_instruction: 0,
      errors: Vec::new(),
      buf: [0; 4],
    }
  }

  pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
    match parse_program(raw) {
      Ok((_remainder, program)) => {
        self.process_first_phase(&program);

        if !self.errors.is_empty() {
          error!("There were errors during the first phase of assembly: {:?}", self.errors);
          return Err(self.errors.clone());
        }

        if self.sections.len() != 2 {
          error!("There should be exactly two sections in the program: .data and .code");
          self.errors.push(AssemblerError::InsufficientSections);
          return Err(self.errors.clone());
        }

        let mut body = self.process_second_phase(&program);
        let mut assembled_program = get_lumi_header(self.ro.len());
        assembled_program.append(&mut self.ro);
        assembled_program.append(&mut body);

        info!("Assembled program length: {}", assembled_program.len());
        Ok(assembled_program)
      }
      Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
        let formatted_error = self.format_nom_error(raw, e);
        error!(
          "There was an error assembling the program: {}",
          formatted_error
        );
        Err(vec![AssemblerError::ParseError {
          error: formatted_error,
        }])
      }
      Err(nom::Err::Incomplete(_)) => {
        error!("Parsing error, incomplete input");
        Err(vec![AssemblerError::ParseError {
          error: "Incomplete input".to_string(),
        }])
      }
    }
  }

  /// Run the first pass assembly process.
  /// This will look for label declarations and store them in the symbol table.
  fn process_first_phase(&mut self, program: &Program) {
    for instruction in program.get_instructions() {
      debug!("Processing instruction: {:?}", instruction);
      if instruction.is_label() {
        if self.current_section.is_some() {
          self.process_label_declaration(instruction);
        } else {
          self.errors.push(AssemblerError::NoSegmentDeclarationFound {
            instruction: self.current_instruction,
          });
        }
      }

      if instruction.is_directive() {
        self.process_directive(instruction);
      }

      self.current_instruction += 1;
    }

    self.phase = AssemblerPhase::Second;
  }

  /// Run the second pass assembly process.
  /// This will generate the bytecode for the program.
  fn process_second_phase(&mut self, program: &Program) -> Vec<u8> {
    self.current_instruction = 0;
    let mut bytecode = vec![];
    debug!("Symbol table: {:?}", self.symbols.get_symbols());
    debug!("Read-Only data: {:?}", self.ro);
    debug!("Read-Only offset: {}", self.ro_offset);

    for instruction in program.get_instructions() {
      debug!("Processing instruction: {:?}", instruction);
      if instruction.is_directive() {
        debug!("Found a directive in second phase: {:?}, skipping...", instruction.directive);
        continue;
      }

      if instruction.is_opcode() {
        let mut bytes = instruction.to_bytes(&self.symbols);
        bytecode.append(&mut bytes);
      }

      self.current_instruction += 1;
    }

    bytecode
  }

  fn process_label_declaration(&mut self, instruction: &AssemblerInstruction) {
    let name = match instruction.get_label_name() {
      Some(name) => name,
      None => {
        self.errors.push(AssemblerError::StringConstantDeclaredWithoutLabel {
          instruction: self.current_instruction,
        });
        return;
      }
    };

    debug!("Processing label declaration: {} on line {}", name, self.current_instruction);
    if self.symbols.has_symbol(&name) {
      self.errors.push(AssemblerError::SymbolAlreadyDeclared {
        symbol: name.to_string(),
      });
    }

    let symbol = Symbol::new_with_offset(
      name,
      SymbolType::Label,
      (self.current_instruction * 4) + LUMI_HEADER_LENGTH as u32 + 1,
    );
    info!(
      "Added a new symbol to table: {:?} with offset: {}",
      symbol, (self.current_instruction * 4) + LUMI_HEADER_LENGTH as u32 + 1
    ); // TODO: this was set to + 60 for some reason
    self.symbols.add_symbol(symbol);
  }

  /// Process a directive.
  /// If directive has operands, then figure out what sort of directive it is.
  /// If there are no operands, then process the directive as a section header.
  fn process_directive(&mut self, instruction: &AssemblerInstruction) {
    let directive_name = match instruction.get_directive_name() {
      Some(name) => name,
      None => {
        error!("Directive has an invalid name: {:?}", instruction);
        return;
      }
    };

    if instruction.has_operands() {
      match directive_name.as_ref() {
        "asciiz" => { // todo: check if I need the . at the start
          self.handle_asciiz(instruction);
        }
        "integer" => {
          self.handle_integer(instruction);
        }
        _ => {
          self.errors.push(AssemblerError::UnknownDirectiveFound {
            directive: directive_name.to_string(),
          });
          return;
        }
      }
    } else {
      self.process_section_header(&directive_name);
    }
  }

  fn handle_asciiz(&mut self, instruction: &AssemblerInstruction) {
    if self.phase != AssemblerPhase::First {
      return;
    }

    match instruction.get_string_constant() {
      Some(s) => {
        match instruction.get_label_name() {
          Some(name) => {
            self.symbols.set_symbol_offset(&name, self.ro_offset);
          }
          None => {
            error!("Foudn a string constant with no associated label");
            return;
          }
        };

        for byte in s.as_bytes() {
          self.ro.push(*byte);
          self.ro_offset += 1;
        }

        self.ro.push(0);
        self.ro_offset += 1;
      }
      None => {
        error!("String constant following a .asciiz directive is missing");
      }
    }
  }

  fn handle_integer(&mut self, instruction: &AssemblerInstruction) {
    if self.phase != AssemblerPhase::First {
      return;
    }

    match instruction.get_i32_constant() {
      Some(s) => {
        match instruction.get_label_name() {
          Some(name) => {
            self.symbols.set_symbol_offset(&name, self.ro_offset);
          }
          None => {
            // e.g. someone types .integer 50
            error!("Found an integer constant with no associated label");
            return;
          }
        };

        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(s).unwrap();
        for byte in &wtr {
          self.ro.push(*byte);
          self.ro_offset += 1;
        }
      }
      None => {
        // someone types .integer
        error!("Integer constant following a .integer directive is missing");
      }
    }
  }

  fn process_section_header(&mut self, header_name: &str) {
    let mut new_section: AssemblerSection = header_name.into();

    if new_section == AssemblerSection::Unknown {
      error!("Found a section header that is unknown: {:#?}", header_name);
      return;
    }

    match new_section {
      AssemblerSection::Code {
        ref mut starting_instruction,
      } => {
        debug!("Code section starts at: {}", self.current_instruction);
        *starting_instruction = Some(self.current_instruction)
      }
      AssemblerSection::Data {
        ref mut starting_instruction,
      } => {
        debug!("Data section starts at: {}", self.current_instruction);
        *starting_instruction = Some(self.current_instruction)
      }
      AssemblerSection::Unknown => {
        error!("Found an unknown section header: {:#?}", header_name);
      }
    }

    self.sections.push(new_section.clone());
    self.current_section = Some(new_section);
  }

  // todo: move this out of here
  fn format_nom_error(&self, input: &str, error: VerboseError<&str>) -> String {
    let mut error_string = String::new();
    let lines: Vec<_> = input.split('\n').collect();
    let mut line_number = 1;
    let mut column_number = 1;

    for (subinput, kind) in error.errors {
      // update line and column numbers
      for line in lines
        .iter()
        .take_while(|&&line| !subinput.starts_with(line))
      {
        line_number += 1;
        column_number = line.len() + 1;
      }

      match kind {
        VerboseErrorKind::Context(ctx) => {
          let error_fragment = subinput
            .chars()
            .take(20) // show first 20 characters of the error fragment
            .collect::<String>();

          error_string.push_str(&format!(
            "{}: at line {}, column: {}: near: `{}`\n",
            ctx.red(),
            line_number.to_string().yellow(),
            column_number.to_string().yellow(),
            error_fragment.green()
          ));
        }
        _ => (),
      }
    }

    error_string
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_symbol_table() {
    let mut sym = SymbolTable::new();
    let new_symbol = Symbol::new_with_offset("test".to_string(), SymbolType::Label, 12);
    sym.add_symbol(new_symbol);
    assert_eq!(sym.get_symbols().len(), 1);

    let v = sym.symbol_value("test");
    assert_eq!(true, v.is_some());

    let v = v.unwrap();
    assert_eq!(v, 12);

    let v = sym.symbol_value("does_not_exist");
    assert_eq!(v.is_some(), false);
  }

  #[test]
  fn test_assemble_program() {
    let mut asm = Assembler::new();
    let test_string = r".data
        .code
        load $0 #100
        load $1 #1
        load $2 #0
        test: inc $0
        neq $0 $2
        jeq @test
        hlt
        ";
    let program = asm.assemble(test_string).unwrap();
    assert_eq!(program.len(), 97);
  }

  #[test]
  fn test_assemble_program_with_start_offset_written() {
    let mut asm = Assembler::new();
    let test_string = r".data
        test1: .asciiz 'Hello'
        .code
        load $0 #100
        load $1 #1
        load $2 #0
        test: inc $0
        neq $0 $2
        jmpe @test
        hlt
        ";
    let program = asm.assemble(test_string).unwrap();
    assert_eq!(program.len(), 103);
  }

  #[test]
  fn test_ro_data_asciiz() {
    let mut asm = Assembler::new();
    let test_program = r".data
        test: .asciiz 'this is a test'
        .code
        ";
    let program = asm.assemble(test_program);
    assert_eq!(program.is_ok(), true);
  }

  #[test]
  fn test_ro_data_i32() {
    let mut asm = Assembler::new();
    let test_program = r".data
        test: .integer #300
        .code
        ";
    let program = asm.assemble(test_program);
    assert_eq!(program.is_ok(), true);
  }

  #[test]
  fn test_ro_bad_data() {
    let mut asm = Assembler::new();
    let test_program = r".data
        test: .asciiz 'this is a test'
        .wrong
        ";
    let program = asm.assemble(test_program);
    assert_eq!(program.is_ok(), false);
  }

  #[test]
  fn test_first_phase_no_segment() {
    let mut asm = Assembler::new();
    let test_program = "hello: .asciiz 'Fail'";
    let result = parse_program(test_program);
    assert_eq!(result.is_ok(), true);
    let (_, mut program) = result.unwrap();
    asm.process_first_phase(&mut program);
    assert_eq!(asm.errors.len(), 1);
  }

  #[test]
  /// Tests that code inside a proper segment works
  fn test_first_phase_inside_segment() {
    let mut asm = Assembler::new();
    let test_program = r"
        .data
        test: .asciiz 'Hello'
        ";
    let result = parse_program(test_program);
    assert_eq!(result.is_ok(), true);
    let (_, mut program) = result.unwrap();
    asm.process_first_phase(&mut program);
    assert_eq!(asm.errors.len(), 0);
  }

  #[test]
  fn test_assemble_program_all() {
    let _ = env_logger::builder().is_test(true).try_init();
    let mut asm = Assembler::new();
    let test_string = r".data
        test1: .asciiz 'Hello'
        .code
        load $0 #100
        load $1 #1
        add $2 $0 $1
        sub $3 $0 $1
        mul $4 $0 $1
        div $5 $0 $1
        hlt
        jmp $0
        jmpf $1
        jmpb $2
        eq $0 $1
        neq $0 $1
        gt $0 $1
        lt $0 $1
        gte $0 $1
        lte $0 $1
        jmpe $0
        djmpe @directjump
        directjump: prts @test1
        aloc $0
        inc $0
        dec $0
        nop
        prts @test1
        loadf64 $0 #100.0
        loadf64 $1 #1.0
        addf64 $2 $0 $1
        subf64 $3 $0 $1
        mulf64 $4 $0 $1
        divf64 $5 $0 $1
        eqf64 $0 $1
        neqf64 $0 $1
        gtf64 $0 $1
        ltf64 $0 $1
        gtef64 $0 $1
        ltef64 $0 $1
        shl $0 #123
        shr $0 #123
        and $0 $1 $2
        or $0 $1 $2
        xor $0 $1 $2
        not $0 $1
        lui $0 #100 #200
        cloop #10
        loopstart: add $0 $0 #1
        loop @loopstart
        loadm $0 $1
        setm $0 $1
        push $0
        subroutine: pop $0
        call @subroutine
        djmp @subroutine
        bkpt
        igl
        ";
    let program = asm.assemble(test_string).unwrap();
    assert_eq!(program.len(), 175);
  }
}