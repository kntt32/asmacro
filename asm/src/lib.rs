/*
/// Assembler
pub mod assembler;

pub use assembler::*;
*/
/// Simple parser for assembly
pub mod parser;

/// Line information of assembly
pub mod line;

pub mod object;

/// Information of instructions
pub use line::instruction;

/// Types of registers
pub mod register;

/// Functions
pub mod functions;
