use syntax_tree::SyntaxTree;
use util::Offset;

/// 文法構造あつかうモジュール
pub mod syntax_tree;

pub mod types;

/// コンパイルを行う関数
pub fn compile(src: &str) -> Result<String, Vec<(Offset, String)>> {
    let syntax_tree = SyntaxTree::new(src);
    syntax_tree.compile()
}
