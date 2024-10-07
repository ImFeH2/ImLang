use crate::frontend::ast::ASTNode;
use crate::frontend::lexer::Lexer;
use crate::frontend::lexer::Tokens;
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn eat(&mut self, token: Tokens) {
        match self.lexer.next() {
            Some(t) => {
                if t != token {
                    panic!("Expected {:?}, but got {:?}", token, t);
                }
            }
            None => panic!("Expected {:?}, but got None", token),
        }
    }

    fn parse_assignment(&mut self) -> ASTNode {
        let identifier = match self.lexer.next().unwrap() {
            Tokens::Identifier(identifier) => identifier,
            _ => panic!("Expected identifier"),
        };
        self.eat(Tokens::Operator("=".parse().unwrap()));
        let expression = self.parse_expression();
        self.eat(Tokens::Newline);

        ASTNode::Assign(identifier, Box::new(expression))
    }

    fn parse_expression(&mut self) -> ASTNode {
        self.parse_term()
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();

        while let Some(token) = self.lexer.peek() {
            match token {
                Tokens::Operator(op) if op == "+" || op == "-" => {
                    let op = self.lexer.next().unwrap();
                    let right = self.parse_factor();
                    node = ASTNode::Expression(Box::new(ASTNode::BinaryOp(
                        match op {
                            Tokens::Operator(op_str) => op_str,
                            _ => unreachable!(),
                        },
                        Box::new(node),
                        Box::new(right),
                    )));
                }
                _ => break,
            }
        }

        node
    }

    fn parse_factor(&mut self) -> ASTNode {
        let mut node = self.parse_primary();

        while let Some(token) = self.lexer.peek() {
            match token {
                Tokens::Operator(op) if op == "*" || op == "/" => {
                    let op = self.lexer.next().unwrap();
                    let right = self.parse_primary();
                    node = ASTNode::Expression(Box::new(ASTNode::BinaryOp(
                        match op {
                            Tokens::Operator(op_str) => op_str,
                            _ => unreachable!(),
                        },
                        Box::new(node),
                        Box::new(right),
                    )));
                }
                _ => break,
            }
        }

        node
    }

    fn parse_primary(&mut self) -> ASTNode {
        match self.lexer.next().unwrap() {
            Tokens::Number(value) => ASTNode::Number(value),
            Tokens::Identifier(name) => ASTNode::Identifier(name),
            Tokens::Punctuation(p) if p == "(" => {
                let expr = self.parse_expression();
                self.eat(Tokens::Punctuation(")".parse().unwrap()));
                expr
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_print(&mut self) -> ASTNode {
        self.eat(Tokens::Keyword("print".parse().unwrap()));
        let expression = self.parse_expression();

        ASTNode::Print(Box::from(expression))
    }

    pub fn parse(&mut self) -> ASTNode {
        match self.lexer.peek().unwrap() {
            Tokens::Keyword(keyword) => match keyword.as_str() {
                "print" => self.parse_print(),
                _ => self.parse_assignment(),
            },
            Tokens::Identifier(_) => self.parse_assignment(),
            _ => {
                self.lexer.next();
                ASTNode::Empty
            }
        }
    }

    pub fn parse_all(&mut self) -> Vec<ASTNode> {
        let mut ast = Vec::new();
        while let Some(_) = self.lexer.peek() {
            ast.push(self.parse());
        }
        ast
    }
}
