pub mod header_utils;
mod disassembler;

use crate::util::header_utils::LUMI_HEADER_LENGTH;
use byteorder::{LittleEndian, ReadBytesExt};
use colored::Colorize;
use std::io::Cursor;

pub fn to_hex(value: usize) -> String {
    format!("{:08x}", value)
}

fn get_byte_color(
    index: usize,
    current_instruction: Option<usize>,
    bytecode: &Vec<u8>,
) -> colored::Color {
    let mut highlight_current = false;
    let mut current_instruction_pos = 0;
    if current_instruction.is_some() {
        highlight_current = true;
        current_instruction_pos = current_instruction.unwrap();
    }

    const OFFSET_SIZE: usize = 4;
    let start_index = LUMI_HEADER_LENGTH + OFFSET_SIZE + 1;
    let mut rdr = Cursor::new(&bytecode[LUMI_HEADER_LENGTH + 1..LUMI_HEADER_LENGTH + 1 + 4]);
    let offset = rdr.read_u32::<LittleEndian>().unwrap() as usize;
    let end_index = start_index + offset;

    if index < 64 {
        // highlight the header
        colored::Color::BrightBlue
    } else if index >= 64 && index <= 67 {
        // highlight the RO offset
        colored::Color::BrightMagenta
    } else if index >= start_index && index <= end_index {
        colored::Color::BrightCyan
    } else if highlight_current && index == current_instruction_pos {
        // highlight current byte that PC is pointing to
        colored::Color::Red
    } else if highlight_current
        && index > current_instruction_pos
        && index <= current_instruction_pos + 3
    {
        // highlight the next 3 bytes (instruction is 32 bits long)
        colored::Color::Yellow
    } else {
        colored::Color::White
    }
}

fn print_ascii_representation(
    line: &[u8],
    line_start_index: usize,
    current_instruction: Option<usize>,
    bytecode: &Vec<u8>,
) {
    print!("  ");
    for (i, &byte) in line.iter().enumerate() {
        let index = line_start_index + i;
        let color = get_byte_color(index, current_instruction, bytecode);

        if byte >= 32 && byte <= 126 {
            print!("{}", (byte as char).to_string().color(color));
        } else {
            print!("{}", ".".color(color));
        }
    }
}

pub fn visualize_program(bytecode: &Vec<u8>, current_instruction: Option<usize>) {
    for (index, &byte) in bytecode.iter().enumerate() {
        // prints the address offset at the start of each line
        if index % 16 == 0 {
            if index != 0 {
                print_ascii_representation(
                    &bytecode[index - 16..index],
                    index - 16,
                    current_instruction,
                    bytecode,
                );
                println!();
            }
            print!("{} ", format!("{:08x}", index).green());
        }

        let color = get_byte_color(index, current_instruction, bytecode);

        print!("{}", format!("{:02X}", byte).color(color));

        // group 2 bytes together
        if index % 2 == 1 {
            print!(" ");
        }
    }

    // handle case where program does not end at 16-byte boundary
    let remainder = bytecode.len() % 16;
    if remainder != 0 {
        let padding = (16 - remainder) * 2 + (16 - remainder) / 2 + 1;
        print!("{:padding$}", "", padding = padding);
        print_ascii_representation(
            &bytecode[bytecode.len() - remainder..],
            bytecode.len() - remainder,
            current_instruction,
            bytecode,
        );
    } else {
        print_ascii_representation(
            &bytecode[bytecode.len() - 16..],
            bytecode.len() - 16,
            current_instruction,
            bytecode,
        );
    }
    println!();
}
