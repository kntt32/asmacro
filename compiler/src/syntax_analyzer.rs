use crate::parser::{BracketType, Parser, Token, TokenTree, TokenType};
use util::Offset;

#[derive(Clone, PartialEq, Debug)]
pub struct SyntaxTree {
    tree: Vec<SyntaxNode>,
}

impl SyntaxTree {
    pub fn new(src: &str) -> Self {
        let parser = Parser::new(src);
        Self::from_parser(parser)
    }

    /// Parserから生成
    pub fn from_parser(mut p: Parser<'_>) -> Self {
        let mut tree = Vec::new();

        loop {
            let Some(node) = Self::analyze_statement(&mut p) else {
                break;
            };
            tree.push(node);
        }

        SyntaxTree { tree: tree }
    }

    // 文を分析
    pub fn analyze_statement(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        if p.clone().next().is_some() {
            let analyzers = &[
                Self::analyze_function_declaration,
                Self::analyze_function_declaration_with_return_value,
                Self::analyze_variable_declaration,
                Self::analyze_mutable_variable_declaration,
                Self::analyze_assignment_statement,
                Self::analyze_expr_as_statement,
            ];
            let p_copy = *p;
            for f in analyzers {
                if let Some(node) = f(p) {
                    return Some(node);
                }
                *p = p_copy;
            }
            Some(SyntaxNode::Error {
                msg: "".to_string(),
                offset: Offset { column: 0, row: 0 },
            })
        } else {
            None
        }
    }

    // 関数定義の引数を分析
    fn _analyze_arguments(mut p: Parser<'_>) -> Option<Vec<Variable>> {
        let mut arguments = Vec::new();
        loop {
            let (name, offset) = match p.next() {
                Some(Token::Token {
                    r#type: TokenType::Keyword,
                    src: name,
                    offset: offset,
                }) => (name, offset),
                None => break,
                _ => return None,
            };
            let Some(Token::Token {
                r#type: TokenType::Symbol,
                src: ":",
                offset: _,
            }) = p.next()
            else {
                return None;
            };
            let Some(Token::Token {
                r#type: TokenType::Keyword,
                src: r#type,
                offset: _,
            }) = p.next()
            else {
                return None;
            };
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
                src: register,
                offset: _,
            }) = p.next()
            else {
                return None;
            };
            arguments.push(Variable {
                mutable: false,
                name: name.to_string(),
                r#type: r#type.to_string(),
                register: register.to_string(),
            });

            match p.next() {
                Some(Token::Token {
                    r#type: TokenType::Symbol,
                    src: ",",
                    offset: _,
                })
                | None => (),
                _ => return None,
            }
        }
        Some(arguments)
    }

    // 戻り値のない関数定義を分析
    fn analyze_function_declaration(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "fn",
            offset: offset,
        }) = p.next()
        else {
            return None;
        };
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: name,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "missing function name".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Block {
            r#type: BracketType::Bracket,
            parser: args_parser,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "missing function arguments".to_string(),
                offset: offset,
            });
        };
        let arguments = Self::_analyze_arguments(args_parser)?;
        let Some(Token::Block {
            r#type: BracketType::CurlyBracket,
            parser: proc_parser,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "missing function procedure".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::FunctionDeclaration {
            name: name.to_string(),
            args: arguments,
            r#type: "_".to_string(),
            syntax_tree: SyntaxTree::from_parser(proc_parser),
            offset: offset,
        })
    }

    // 戻り値のある関数定義を分析
    fn analyze_function_declaration_with_return_value(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "fn",
            offset: offset,
        }) = p.next()
        else {
            return None;
        };
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: name,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "missing function name".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Block {
            r#type: BracketType::Bracket,
            parser: args_parser,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "missing function arguments".to_string(),
                offset: offset,
            });
        };
        let arguments = Self::_analyze_arguments(args_parser)?;
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "->",
            offset: _,
        }) = p.next()
        else {
            return None;
        };
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: r#type,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "missing function return value type".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Block {
            r#type: BracketType::CurlyBracket,
            parser: proc_parser,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "missing function procedure".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::FunctionDeclaration {
            name: name.to_string(),
            args: arguments,
            r#type: r#type.to_string(),
            syntax_tree: SyntaxTree::from_parser(proc_parser),
            offset: offset,
        })
    }

    // 変数定義を分析
    fn analyze_variable_declaration(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "let",
            offset: offset,
        }) = p.next()
        else {
            return None;
        };
        let name = match p.next() {
            Some(Token::Token {
                r#type: TokenType::Keyword,
                src: name,
                offset: _,
            }) => name,
            Some(Token::Token {
                r#type: TokenType::Symbol,
                src: "mut",
                offset: _,
            }) => return None,
            _ => {
                return Some(SyntaxNode::Error {
                    msg: "expected variable name".to_string(),
                    offset: offset,
                });
            }
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: ":",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \":\"".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: r#type,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected type".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "@",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \"@\"".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: register,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected register".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "=",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \"=\"".to_string(),
                offset: offset,
            });
        };
        let Some(expr) = Self::analyze_expr(p) else {
            return Some(SyntaxNode::Error {
                msg: "unknown expression".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: ";",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \";\"".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::VariableDeclaration {
            variable: Variable {
                mutable: false,
                name: name.to_string(),
                r#type: r#type.to_string(),
                register: register.to_string(),
            },
            expr: Box::new(expr),
            offset: offset,
        })
    }

    // ミュータブルな変数定義を分析
    fn analyze_mutable_variable_declaration(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "let",
            offset: offset,
        }) = p.next()
        else {
            return None;
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "mut",
            offset: _,
        }) = p.next()
        else {
            return None;
        };
        let name = match p.next() {
            Some(Token::Token {
                r#type: TokenType::Keyword,
                src: name,
                offset: _,
            }) => name,
            _ => {
                return Some(SyntaxNode::Error {
                    msg: "expected variable name".to_string(),
                    offset: offset,
                });
            }
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: ":",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \":\"".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: r#type,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected type".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "@",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \"@\"".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: register,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected register".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "=",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \"=\"".to_string(),
                offset: offset,
            });
        };
        let Some(expr) = Self::analyze_expr(p) else {
            return Some(SyntaxNode::Error {
                msg: "unknown expression".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: ";",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \";\"".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::VariableDeclaration {
            variable: Variable {
                mutable: true,
                name: name.to_string(),
                r#type: r#type.to_string(),
                register: register.to_string(),
            },
            expr: Box::new(expr),
            offset: offset,
        })
    }

    // 変数への代入分を分析
    fn analyze_assignment_statement(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: name,
            offset: offset,
        }) = p.next()
        else {
            return None;
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "=",
            offset: _,
        }) = p.next()
        else {
            return None;
        };
        let Some(expr) = Self::analyze_expr(p) else {
            return Some(SyntaxNode::Error {
                msg: "expected expression".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: ";",
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected symbol \";\"".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::AssignmentStatement {
            name: name.to_string(),
            offset: offset,
            expr: Box::new(expr),
        })
    }

    // 式を文として分析
    fn analyze_expr_as_statement(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        let node = Self::analyze_expr(p)?;
        if let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: ";",
            offset: _,
        }) = p.next()
        {
            Some(node)
        } else {
            None
        }
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
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: name,
            offset: offset,
        }) = p.next()
        else {
            return None;
        };
        let Some(Token::Block {
            r#type: BracketType::Bracket,
            parser: args_parser,
            offset: _,
        }) = p.next()
        else {
            return None;
        };
        let Some(arguments) = Self::_analyze_calling_arguments(args_parser) else {
            return Some(SyntaxNode::Error {
                msg: "invalid arguments expression".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::CallingExpr {
            name: name.to_string(),
            arguments: arguments,
        })
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
        name: String,
        args: Vec<Variable>,
        r#type: String,
        syntax_tree: SyntaxTree,
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

#[derive(Clone, PartialEq, Debug)]
pub struct Variable {
    mutable: bool,
    name: String,
    r#type: String,
    register: String,
}
