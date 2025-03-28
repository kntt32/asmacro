use crate::parser::{BracketType, Parser, Token, TokenTree, TokenType};
use asm::assembler::register::Register;
use util::{EResult, Offset, SResult};

#[derive(Clone, PartialEq, Debug)]
pub struct SyntaxTree {
    tree: Vec<SyntaxNode>,
}

impl SyntaxTree {
    pub fn new(src: &str) -> Self {
        let parser = Parser::new(src);
        Self::from_parser(parser)
    }

    /// 正しい構文ツリーであるかチェック
    pub fn check(&mut self) -> SResult<()> {
        todo!()
    }

    /// グローバルの構文チェックして関数リストを返す
    pub fn check_global(&self) -> SResult<Vec<&Function>> {
        todo!()
    }

    // 関数の構文チェック
    pub fn check_function(function_list: &[&Function]) -> SResult<()> {
        todo!()
    }

    /// Parserから生成
    pub fn from_parser(mut p: Parser<'_>) -> Self {
        todo!()
    }

    // 文を分析
    pub fn analyze_statement(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        todo!()
    }

    // 関数定義の引数を分析
    fn _analyze_arguments(mut p: Parser<'_>) -> Option<Vec<Variable>> {
        todo!()
    }

    // 戻り値のない関数定義を分析
    fn analyze_function_declaration(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        todo!()
    }

    // 戻り値のある関数定義を分析
    fn analyze_function_declaration_with_return_value(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        todo!()
    }

    // 変数定義を分析
    fn analyze_variable_declaration(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        todo!()
    }

    // ミュータブルな変数定義を分析
    fn analyze_mutable_variable_declaration(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        todo!()
    }

    // 変数への代入分を分析
    fn analyze_assignment_statement(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        todo!()
    }

    // 式を文として分析
    fn analyze_expr_as_statement(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        todo!()
    }

    // 式を分析
    fn analyze_expr(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        let analyzers = &[
            Self::analyze_number_literal,
            Self::analyze_string_literal,
            Self::analyze_caling_function,
        ];
        let p_copy = *p;
        for f in analyzers {
            if let Some(node) = f(p) {
                return Some(node);
            }
            *p = p_copy;
        }
        None
    }

    // 数値リテラルを分析
    fn analyze_number_literal(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        if let Some(Token::Token {
            r#type: TokenType::NumberLiteral,
            src: src,
            offset: offset,
        }) = p.next()
        {
            Some(SyntaxNode::NumberLiteral {
                src: src.to_string(),
                offset: offset,
            })
        } else {
            None
        }
    }

    // 文字列リテラルを分析
    fn analyze_string_literal(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        if let Some(Token::Token {
            r#type: TokenType::StringLiteral,
            src: src,
            offset: offset,
        }) = p.next()
        {
            Some(SyntaxNode::StringLiteral {
                src: src.to_string(),
                offset: offset,
            })
        } else {
            None
        }
    }

    // 関数呼び出しを分析
    fn analyze_caling_function(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        todo!()
    }

    // 関数呼び出し時の引数を分析
    fn _analyze_calling_arguments(mut p: Parser<'_>) -> Option<Vec<SyntaxNode>> {
        let p_copy = p;
        let mut arguments = Vec::new();
        loop {
            if p.clone().next().is_none() {
                break;
            }
            let arg = Self::analyze_expr(&mut p)?;
            arguments.push(arg);
            match p.next() {
                Some(Token::Token {
                    r#type: TokenType::Symbol,
                    src: ",",
                    offset: _,
                }) => (),
                None => break,
                _ => return None,
            }
        }
        Some(arguments)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SyntaxNode {
    NumberLiteral {
        src: String,
        offset: Offset,
    },
    StringLiteral {
        src: String,
        offset: Offset,
    },
    FunctionDeclaration {
        function: Function,
        offset: Offset,
    },
    VariableDeclaration {
        variable: Variable,
        expr: Box<SyntaxNode>,
        offset: Offset,
    },
    AssignmentStatement {
        name: String,
        expr: Box<SyntaxNode>,
        offset: Offset,
    },
    CallingExpr {
        name: String,
        arguments: Vec<SyntaxNode>,
    },
    Error {
        msg: String,
        offset: Offset,
    },
}

impl SyntaxNode {
    /// 返却されるDataを返す
    pub fn as_data<'a>(&self, function_list: &[&'a Function]) -> Option<Data> {
        todo!()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Variable {
    mutable: bool,
    name: String,
    r#type: String,
    register: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    name: String,
    args: Vec<Variable>,
    r#return_data: Data,
    syntax_tree: SyntaxTree,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Data {
    Some { r#type: String, register: Register },
    None,
}

impl Data {
    /// Parserから型データを取得
    pub fn from_parser(p: &mut Parser<'_>) -> Option<Self> {
        let mut p_copy = *p;
        match p_copy.next() {
            Some(Token::Token {
                r#type: TokenType::Keyword,
                src: name,
                offset: offset,
            }) => {
                let Some(Token::Token {
                    r#type: TokenType::Symbol,
                    src: "@",
                    offset: _,
                }) = p.next()
                else {
                    return None;
                };
                let Some(Token::Token {
                    r#type: TokenType::Keyword,
                    src: register_str,
                    offset: _,
                }) = p_copy.next()
                else {
                    return None;
                };
                let Ok(register) = register_str.parse() else {
                    return None;
                };
                Some(Self::Some {
                    r#type: name.to_string(),
                    register: register,
                })
            }
            Some(Token::Block {
                r#type: BracketType::Bracket,
                parser: mut inner_parser,
                offset: _,
            }) => {
                if inner_parser.next() == None {
                    Some(Self::None)
                } else {
                    todo!()
                }
            }
            _ => None,
        }
    }

    pub fn empty() -> Self {
        Self::None
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Type {
    name: String,
    allowed_registers: Vec<Register>,
}
