use crate::frontend::ast::ASTNode;
use std::collections::HashMap;

pub struct Interpreter {
    context: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    // Update the interpret method to handle the Empty variant
    pub fn interpret(&mut self, node: &ASTNode) -> i32 {
        match node {
            ASTNode::Number(value) => *value,
            ASTNode::Identifier(name) => *self.context.get(name).expect("Undefined variable"),
            ASTNode::Assign(identifier, expression) => {
                let value = self.interpret(expression);
                self.context.insert(identifier.clone(), value);
                value
            }
            ASTNode::Expression(expression) => self.interpret(expression),
            ASTNode::Print(expression) => {
                let value = self.interpret(expression);
                println!("{} = {}", expression.to_string(), value);
                value
            }
            ASTNode::BinaryOp(operator, left, right) => {
                let left = self.interpret(left);
                let right = self.interpret(right);
                match operator.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => panic!("Unknown operator"),
                }
            }
            ASTNode::Empty => 0, // Return 0 for Empty variant
        }
    }

    pub fn interpret_all(&mut self, nodes: &[ASTNode]) {
        for node in nodes {
            self.interpret(node);
        }
    }

    #[allow(dead_code)]
    pub fn print_context(&self) {
        println!("\n[+] Context:");
        for (key, value) in &self.context {
            println!("[ ] {}: {}", key, value);
        }
    }
}
