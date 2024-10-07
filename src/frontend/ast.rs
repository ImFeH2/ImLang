#[allow(dead_code)]
#[derive(Debug)]
pub enum ASTNode {
    Identifier(String),
    Number(i32),
    Assign(String, Box<ASTNode>),
    Expression(Box<ASTNode>),
    Print(Box<ASTNode>),
    BinaryOp(String, Box<ASTNode>, Box<ASTNode>),
    Empty,
}

impl ASTNode {
    pub fn to_string(&self) -> String {
        match self {
            ASTNode::Number(value) => value.to_string(),
            ASTNode::Identifier(name) => name.clone(),
            ASTNode::Assign(identifier, expression) => {
                format!("{} = {}", identifier, expression.to_string())
            }
            ASTNode::Expression(expression) => expression.to_string(),
            ASTNode::Print(expression) => format!("print({})", expression.to_string()),
            ASTNode::BinaryOp(operator, left, right) => {
                format!("({} {} {})", left.to_string(), operator, right.to_string())
            }
            ASTNode::Empty => "".to_string(),
        }
    }
}
