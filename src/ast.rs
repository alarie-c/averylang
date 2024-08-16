#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Node>
}

impl Ast {
    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

#[derive(Debug)]
pub enum Node {
    BinaryExpr(BinaryExpr),
    Integer(i32),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: BinaryOperator,
}