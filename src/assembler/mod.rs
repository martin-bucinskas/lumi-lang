use crate::assembler::assembler_errors::AssemblerError;
use crate::assembler::instruction_parsers::AssemblerInstruction;
use crate::assembler::program_parsers::{parse_program, Program};
use crate::assembler::symbols::{Symbol, SymbolTable, SymbolType};
use crate::instruction::Opcode;
use crate::util::visualize_program;
use byteorder::{LittleEndian, WriteBytesExt};
use colored::Colorize;
use log::{debug, error, info};
use nom::error::{VerboseError, VerboseErrorKind};
use nom::Offset;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

mod assembler_errors;
mod comment_parsers;
mod directive_parsers;
mod instruction_parsers;
mod label_parsers;
mod opcode_parsers;
mod operand_parsers;
pub mod program_parsers;
mod register_parsers;
mod separator_parsers;
mod symbols;

pub const LUMI_HEADER_PREFIX: [u8; 4] = [0x4C, 0x55, 0x4D, 0x49];
pub const LUMI_HEADER_LENGTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    FloatOperand { value: f64 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    LString { name: String },
    Separator,
    Comment,
}

#[derive(Debug, PartialEq, Clone)]
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
    fn from(value: &'a str) -> Self {
        match value {
            "data" => AssemblerSection::Data {
                starting_instruction: None,
            },
            "code" => AssemblerSection::Code {
                starting_instruction: None,
            },
            _ => AssemblerSection::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Assembler {
    /// Tracks which phase the assembler is in
    pub phase: AssemblerPhase,
    /// Symbol table for constants and variables
    pub symbols: SymbolTable,
    /// The read-only data section where constants are placed
    pub ro: Vec<u8>,
    /// The compiled bytecode generated from the assembly instructions
    pub bytecode: Vec<u8>,
    /// Tracks the current offset of the read-only section
    ro_offset: u32,
    /// A list of all the sections in code
    sections: Vec<AssemblerSection>,
    /// The current section the assembler is in
    current_section: Option<AssemblerSection>,
    /// The current instruction the assembler is working on
    current_instruction: u32,
    /// Errors found whilst assembling
    errors: Vec<AssemblerError>,
    /// Scratch buffer
    buf: [u8; 4],
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            ro: vec![],
            bytecode: vec![],
            ro_offset: 0,
            sections: vec![],
            current_section: None,
            current_instruction: 0,
            errors: vec![],
            buf: [0, 0, 0, 0],
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        match parse_program(raw) {
            Ok((_remainder, program)) => {
                self.process_first_phase(&program);

                if !self.errors.is_empty() {
                    error!(
                        "Errors were found in the first parsing phase: {:?}",
                        self.errors
                    );
                    return Err(self.errors.clone());
                }

                if self.sections.len() != 2 {
                    error!("Did not find at least two sections.");
                    self.errors.push(AssemblerError::InsufficientSections);
                    return Err(self.errors.clone());
                }

                let mut body = self.process_second_phase(&program);
                let mut assembled_program = self.write_lumi_header();
                assembled_program.append(&mut self.ro);
                assembled_program.append(&mut body);

                info!("Program Length: {}", assembled_program.len());
                visualize_program(&assembled_program, None);

                Ok(assembled_program)
            }
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                let formatted_error = self.format_nom_error(raw, e);
                error!(
                    "There was an error assembling the code: {}",
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

    pub fn assemble_file(&mut self, filepath: &str) -> bool {
        let raw = self.read_file(filepath);
        let result = self.assemble(&raw);

        if result.is_err() {
            return false;
        }

        // write file
        let mut assembled_file_path = self.strip_extension(filepath);
        assembled_file_path.push_str(".bin");

        let binary = result.unwrap();

        self.create_binary_file(&assembled_file_path, &binary)
            .expect("Failed writing binary file");
        true
    }

    fn write_lumi_header(&self) -> Vec<u8> {
        let mut header = vec![];
        for byte in LUMI_HEADER_PREFIX.into_iter() {
            header.push(byte.clone());
        }
        while header.len() <= LUMI_HEADER_LENGTH {
            header.push(0u8);
        }

        // calculate and write the starting offset for the VM to know where the RO section ends
        debug!("RO Length: {}", self.ro.len());
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u32::<LittleEndian>(self.ro.len() as u32).unwrap();
        for byte in &wtr {
            debug!("Written offset bytes: {:02X}", byte);
        }
        header.append(&mut wtr);
        header
    }

    fn strip_extension(&self, input: &str) -> String {
        if input.ends_with(".lumi") {
            input.strip_suffix(".lumi").unwrap().to_string()
        } else if input.ends_with(".iasm") {
            input.strip_suffix(".iasm").unwrap().to_string()
        } else {
            input.to_string()
        }
    }

    fn create_binary_file(&self, file_name: &str, data: &[u8]) -> io::Result<()> {
        let mut file_handle = File::create(file_name).expect("Unable to create a file");
        file_handle
            .write_all(data)
            .expect("Failed to write to file");
        Ok(())
    }

    fn read_file(&mut self, tmp: &str) -> String {
        let filename = Path::new(tmp);
        debug!("Reading file: {}", filename.to_str().unwrap());
        match File::open(&filename) {
            Ok(mut file_handle) => {
                let mut contents = String::new();
                match file_handle.read_to_string(&mut contents) {
                    Ok(_) => {
                        return contents;
                    }
                    Err(err) => {
                        error!("There was an error reading file: {:?}", err);
                        std::process::exit(1);
                    }
                }
            }
            Err(err) => {
                error!("File not found: {:?}", err);
                std::process::exit(1);
            }
        }
    }

    /// Attempts to run the first pass assembling process.
    /// This will look for label declarations and store them in
    /// the symbol table.
    fn process_first_phase(&mut self, program: &Program) {
        for instruction in program.get_instructions() {
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

    /// Attempts to run the second part of the assembling process.
    /// This converts the tokens to bytecode.
    fn process_second_phase(&mut self, program: &Program) -> Vec<u8> {
        self.current_instruction = 0;
        let mut bytecode = vec![];
        debug!("Symbol Table: {:?}", self.symbols.get_symbols());
        debug!("Read-Only Data: {:?}", self.ro);
        debug!("Read-Only Offset: {:?}", self.ro_offset);
        for instruction in program.get_instructions() {
            debug!("Instruction: {}", instruction);
            if instruction.is_directive() {
                debug!(
                    "Found a directive in second phase: {:?}, bypassing",
                    instruction.directive
                );
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
                self.errors
                    .push(AssemblerError::StringConstantDeclaredWithoutLabel {
                        instruction: self.current_instruction,
                    });
                return;
            }
        };

        debug!(
            "Label declaration: {} on line {}",
            name, self.current_instruction
        );
        if self.symbols.has_symbol(&name) {
            self.errors.push(AssemblerError::SymbolAlreadyDeclared);
            return;
        }

        let symbol = Symbol::new_with_offset(
            name,
            SymbolType::Label,
            (self.current_instruction * 4) + LUMI_HEADER_LENGTH as u32 + 1,
        );
        info!(
            "Added a new symbol to table: {:?} with offset: {:?}",
            symbol,
            (self.current_instruction * 4) + 60
        );
        self.symbols.add_symbol(symbol);
    }

    /// Process directive.
    /// If directive has operands, then figure out what sort of directive it is.
    /// If there are no operands, then process it as a section header.
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
                "asciiz" => {
                    self.handle_asciiz(instruction);
                }
                "integer" => {
                    self.handle_integer(instruction);
                }
                _ => {
                    self.errors.push(AssemblerError::UnknownDirectiveFound {
                        directive: directive_name.clone(),
                    });
                    return;
                }
            }
        } else {
            self.process_section_header(&directive_name);
        }
    }

    /// Handle declaration of a null-terminated string:
    /// hello: .asciiz 'Hello!'
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
                        error!("Found a string constant with no associated label");
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
                error!("String constant following a .asciiz was empty");
            }
        }
    }

    fn handle_integer(&mut self, i: &AssemblerInstruction) {
        if self.phase != AssemblerPhase::First {
            return;
        }

        match i.get_i32_constant() {
            Some(s) => {
                match i.get_label_name() {
                    Some(name) => {
                        self.symbols.set_symbol_offset(&name, self.ro_offset);
                    }
                    None => {
                        // e.g. someone types .integer 50
                        error!("Found a string constant with no associated label");
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
                // someone types `.integer`
                error!("integer constant following a .integer was empty")
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
                error!("Found a section header that is unknown: {:?}", new_section)
            }
        }

        // TODO: check if we need to keep a list of all sections seen
        self.sections.push(new_section.clone());
        self.current_section = Some(new_section);
    }

    fn format_nom_error(&self, input: &str, err: VerboseError<&str>) -> String {
        let mut error_string = String::new();
        let lines: Vec<_> = input.split('\n').collect();
        let mut line_number = 1;
        let mut column_number = 1;

        for (subinput, kind) in err.errors {
            // Update line and column numbers
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
                        .take(20) // Show first 20 characters of the error fragment
                        .collect::<String>();
                    error_string.push_str(&format!(
                        "{}: at line {}, column {}: near `{}`\n",
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
    use crate::assembler::program_parsers::parse_program;
    use crate::assembler::{Assembler, Symbol, SymbolTable, SymbolType};
    use crate::vm::VM;

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
        let mut vm = VM::new();
        assert_eq!(program.len(), 97);
        vm.add_bytes(program);
        assert_eq!(vm.get_program().len(), 97);
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
        let mut vm = VM::new();
        assert_eq!(program.len(), 103);
        vm.add_bytes(program);
        assert_eq!(vm.get_program().len(), 103);
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

    // #[test]
    // fn test_ro_data_i32() {
    //     let mut asm = Assembler::new();
    //     let test_program = r".data
    //     test: .integer #300
    //     .code
    //     ";
    //     let program = asm.assemble(test_program);
    //     assert_eq!(program.is_ok(), true);
    // }

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
}
