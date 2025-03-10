use crate::parser::{Parser, TokenTree, Token};
use util::Offset;

#[derive(Clone, PartialEq, Debug)]
pub struct SyntaxTree<'a> {
    tree: Vec<SyntaxNode<'a>>,
}

impl<'a> SyntaxTree<'a> {
    pub fn new(p: Parser<'a>) -> Self {
        todo!()
    }

    fn analyze_function_declaration(mut p: Parser<'a>) -> Option<SyntaxNode<'a>> {
        let Some(Token::Keyword {src: "fn", offset: _}) = p.next() else {
            return None;
        };

        let name;

        match p.next() {
            Some(Token::Keyword { src: s, offset: _}) => name = s,
            Some(_) => todo!(),//return Some(SyntaxNode::Err { msg: format!("expected identifier, found \"{}\"", ), })
            None => todo!()
        }

        let Some(Token::Keyword {src: name, offset: _}) = p.next() else {
            todo!()
        };

        todo!()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SyntaxNode<'a> {
    FunctionDeclaration { name: &'a str, args: SyntaxTree<'a>, r#type: &'a str, assembly: bool, offset: Offset, },
    VariableDeclaration { left: SyntaxTree<'a>, right: SyntaxTree<'a>, mutability: bool, r#type: &'a str, storage: Storage<'a>, offset: Offset },
    AssignmentStatement { left: SyntaxTree<'a>, right: SyntaxTree<'a>, offset: Offset },
    LoopExpression { code: SyntaxTree<'a>, offset: Offset, },
    BranchExpression { condition: SyntaxTree<'a>, if_branch: SyntaxTree<'a>, else_branch: SyntaxTree<'a>, offset: Offset },
    TypeCast { left: SyntaxTree<'a>, r#type: &'a str, offset: Offset, },
    StorageCast { left: SyntaxTree<'a>, r#storage: Storage<'a>, offset: Offset, },
    ConditionExpression { left: SyntaxTree<'a>, right: SyntaxTree<'a>, lt: bool, eq: bool, gt: bool, offset: Offset, },
    ReturnStatement { code: SyntaxTree<'a>, offset: Offset, },
    Err { msg: String, offset: Offset, },
}

#[derive(Clone, PartialEq, Debug)]
pub enum Storage<'a> {
    Register(Vec<&'a str>),
    Stack { offset: usize, size: usize, },
}

