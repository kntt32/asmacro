/// アセンブラモジュール
pub mod assembler {
    /// Simple parser for assembly
    pub mod parser;

    /// Line information of assembly
    pub mod line;

    /// Information of instructions
    pub use line::instruction;

    /// Types of registers
    pub mod register;
}

/// リンカモジュール
pub mod linker {
    /// オブジェクト形式関連
    pub mod object;
}

/// Functions
pub mod functions;
