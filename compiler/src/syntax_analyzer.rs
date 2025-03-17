use crate::parser::{BracketType, Parser, Token, TokenTree, TokenType};
use util::Offset;

#[derive(Clone, PartialEq, Debug)]
pub struct SyntaxTree {
    tree: Vec<SyntaxNode>,
}

impl SyntaxTree {
    pub fn new(mut p: Parser<'_>) -> Option<Self> {
        let mut tree = Vec::new();
        loop {
            let Some(syntax_node) = Self::analyze_expr(&mut p) else {
                break;
            };
            tree.push(syntax_node);
        }
        Some(SyntaxTree { tree: tree })
    }

    fn analyze_expr(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        let analyzers: &[fn(&mut Parser<'_>) -> Option<SyntaxNode>] = &[
            Self::analyze_number_literal,
            Self::analyze_function_declaration_with_return_value,
            Self::analyze_function_declaration,
            Self::analyze_variable_declaration,
            Self::analyze_mutable_variable_declaration,
            Self::analyze_assignment_statement,
            Self::analyze_calling_expr,
        ];
        let p_copy = *p;
        for f in analyzers {
            if let Some(syntax_node) = (f)(p) {
                return Some(syntax_node);
            } else {
                *p = p_copy;
            }
        }
        None
    }

    pub fn analyze_number_literal(p: &mut Parser<'_>) -> Option<SyntaxNode> {
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

    pub fn analyze_function_declaration(p: &mut Parser<'_>) -> Option<SyntaxNode> {
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
                msg: "expected function name".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Block {
            r#type: BracketType::Bracket,
            parser: mut arg_parser,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected arguments".to_string(),
                offset: offset,
            });
        };
        let None = arg_parser.next() else {
            return None;
        }; // めんどくさいため関数の引数関連の処理はあとから実装
        let Some(Token::Block {
            r#type: BracketType::CurlyBracket,
            parser: inner_parser,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected expression".to_string(),
                offset: offset,
            });
        };
        let Some(syntax_tree) = Self::new(inner_parser) else {
            return Some(SyntaxNode::Error {
                msg: "expected expression".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::FunctionDeclaration {
            name: name.to_string(),
            args: Vec::new(),
            r#type: "()".to_string(),
            syntax_tree: syntax_tree,
            offset: offset,
        })
    }

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
                msg: "expected function name".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Block {
            r#type: BracketType::Bracket,
            parser: mut arg_parser,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected arguments".to_string(),
                offset: offset,
            });
        };
        let None = arg_parser.next() else {
            return None;
        }; // めんどくさいため関数の引数関連の処理はあとから実装
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
            src: return_value_type,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected returning value type".to_string(),
                offset: offset,
            });
        };
        let Some(Token::Block {
            r#type: BracketType::CurlyBracket,
            parser: inner_parser,
            offset: _,
        }) = p.next()
        else {
            return Some(SyntaxNode::Error {
                msg: "expected expression".to_string(),
                offset: offset,
            });
        };
        let Some(syntax_tree) = Self::new(inner_parser) else {
            return Some(SyntaxNode::Error {
                msg: "expected expression".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::FunctionDeclaration {
            name: name.to_string(),
            args: Vec::new(),
            r#type: return_value_type.to_string(),
            syntax_tree: syntax_tree,
            offset: offset,
        })
    }

    fn analyze_variable_declaration(p: &mut Parser<'_>) -> Option<SyntaxNode> {
        println!("called A");
        let Some(Token::Token {
            r#type: TokenType::Symbol,
            src: "let",
            offset: offset,
        }) = p.next()
        else {
            return None;
        };
        println!("called B");
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: name,
            offset: _,
        }) = p.next()
        else {
            return None;
        };
        println!("called C");
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
                msg: "expected expression".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::VariableDeclaration {
            variable: Variable {
                mutablity: false,
                name: name.to_string(),
                r#type: r#type.to_string(),
                storage: register.to_string(),
            },
            expr: Box::new(expr),
            offset: offset,
        })
    }

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
        let Some(Token::Token {
            r#type: TokenType::Keyword,
            src: name,
            offset: _,
        }) = p.next()
        else {
            return None;
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
                msg: "expected expression".to_string(),
                offset: offset,
            });
        };
        Some(SyntaxNode::VariableDeclaration {
            variable: Variable {
                mutablity: true,
                name: name.to_string(),
                r#type: r#type.to_string(),
                storage: register.to_string(),
            },
            expr: Box::new(expr),
            offset: offset,
        })
    }

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
        Some(SyntaxNode::AssignmentStatement {
            name: name.to_string(),
            expr: Box::new(expr),
            offset: offset,
        })
    }

    fn analyze_calling_expr(p: &mut Parser<'_>) -> Option<SyntaxNode> {
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
            parser: mut args_parser,
            offset: _,
        }) = p.next()
        else {
            return None;
        };
        if args_parser.next().is_some() {
            return None;
        } // めんどくさいため、引数関連の処理は後で実装予定
        Some(SyntaxNode::CallingExpr {
            name: name.to_string(),
            arguments: Vec::new(),
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SyntaxNode {
    NumberLiteral {
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
        arguments: Vec<SyntaxTree>,
    },
    Error {
        msg: String,
        offset: Offset,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub struct Variable {
    mutablity: bool,
    name: String,
    r#type: String,
    storage: String,
}
