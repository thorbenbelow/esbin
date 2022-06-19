use crate::Interpreter::Interpreter;
use crate::Value::Value;

pub trait ASTNode {
    fn execute(&self, interpreter: &Interpreter) -> Value;
}

pub struct Program {
    pub child_nodes: Vec<Box<dyn ASTNode>>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            child_nodes: vec![],
        }
    }
}

impl ASTNode for Program {
    fn execute(&self, interpreter: &Interpreter) -> Value {
        Value {}
    }
}

enum NodeType {}

struct FunctionDeclaration;
struct BlockStatement;
struct BinaryExpression;
struct Literal;
struct ExpressionStatement;
