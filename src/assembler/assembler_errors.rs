use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum AssemblerError {
    NoSegmentDeclarationFound { instruction: u32 },
    StringConstantDeclaredWithoutLabel { instruction: u32 },
    SymbolAlreadyDeclared,
    UnknownDirectiveFound { directive: String },
    NonOpcodeInOpcodeField,
    InsufficientSections,
    ParseError { error: String },
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
          AssemblerError::NoSegmentDeclarationFound { instruction } => f.write_str(&format!(
              "No segment declaration (e.g., .code, .data) prior to finding an opcode or other directive. Instruction # was: {}",
                instruction
          )),
          AssemblerError::StringConstantDeclaredWithoutLabel { instruction } => f.write_str(&format!(
              "Found a string constant without a corresponding label. Instruction # was: {}",
              instruction
          )),
          AssemblerError::SymbolAlreadyDeclared => f.write_str("This symbol was previously declared."),
          AssemblerError::UnknownDirectiveFound { ref directive } => {
              f.write_str(&format!("Invalid or unknown directive found. Directive name was: {}", directive))
          }
          AssemblerError::NonOpcodeInOpcodeField => f.write_str("A non-opcode was found in an opcode field"),
          AssemblerError::InsufficientSections => f.write_str("Less than two sections/segments were found in the code"),
          AssemblerError::ParseError { ref error } => f.write_str(&format!("There was an error parsing the code: {}", error))
        }
    }
}

impl Error for AssemblerError {
    fn description(&self) -> &str {
        match self {
          AssemblerError::NoSegmentDeclarationFound { .. } => "No segment declaration (e.g., .code, .data) prior to finding an opcode or other directive.",
          AssemblerError::StringConstantDeclaredWithoutLabel { .. } => "Found a string constant without a corresponding label.",
          AssemblerError::SymbolAlreadyDeclared { .. } => "This symbol was previously declared.",
          AssemblerError::UnknownDirectiveFound { .. } => "Invalid or unknown directive found.",
          AssemblerError::NonOpcodeInOpcodeField { .. } => "A non-opcode was found in an opcode field.",
          AssemblerError::InsufficientSections { .. } => "Less than two sections/segments were found in the code.",
          AssemblerError::ParseError { .. } => "There was an error parsing the code.",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::assembler_errors::AssemblerError;

    #[test]
    fn test_display_no_segment_declaration_found() {
        let error = AssemblerError::NoSegmentDeclarationFound { instruction: 1 };
        assert_eq!(
          format!("{}", error),
          "No segment declaration (e.g., .code, .data) prior to finding an opcode or other directive. Instruction # was: 1"
        );
    }

    #[test]
    fn test_display_string_constant_declared_without_label() {
        let error = AssemblerError::StringConstantDeclaredWithoutLabel { instruction: 2 };
        assert_eq!(
            format!("{}", error),
            "Found a string constant without a corresponding label. Instruction # was: 2"
        );
    }

    #[test]
    fn test_display_symbol_already_declared() {
        let error = AssemblerError::SymbolAlreadyDeclared;
        assert_eq!(format!("{}", error), "This symbol was previously declared.");
    }

    #[test]
    fn test_display_unknown_directive_found() {
        let error = AssemblerError::UnknownDirectiveFound {
            directive: "invalid_directive".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Invalid or unknown directive found. Directive name was: invalid_directive"
        );
    }

    #[test]
    fn test_display_non_opcode_in_opcode_field() {
        let error = AssemblerError::NonOpcodeInOpcodeField;
        assert_eq!(
            format!("{}", error),
            "A non-opcode was found in an opcode field"
        );
    }

    #[test]
    fn test_display_insufficient_sections() {
        let error = AssemblerError::InsufficientSections;
        assert_eq!(
            format!("{}", error),
            "Less than two sections/segments were found in the code"
        );
    }

    #[test]
    fn test_display_parse_error() {
        let error = AssemblerError::ParseError {
            error: "syntax error".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "There was an error parsing the code: syntax error"
        );
    }

    #[test]
    fn test_error_description() {
        let error = AssemblerError::NonOpcodeInOpcodeField;
        assert_eq!(
            error.to_string(),
            "A non-opcode was found in an opcode field"
        );
    }
}
