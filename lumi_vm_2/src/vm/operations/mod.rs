use crate::vm::virtual_machine::{ExecutionStatus, VirtualMachine};

mod memory;
mod arithmetic;
mod bitwise;
mod comparison;
mod control;
mod logical;
mod system;

pub type InstructionHandler = fn(&mut VirtualMachine) -> ExecutionStatus;
