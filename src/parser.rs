use crate::{ast::{Ast, BinaryExpr, BinaryOperator, Node}, token::{Token, TokenKind}};

pub struct Parser<'a> {
    pub ast: Ast,
    src: &'a Vec<Token>,
    idx: usize,
    current: &'a Token,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a Vec<Token>) -> Self {
        Self {
            src,
            idx: 0usize,
            ast: Ast { nodes: Vec::new() },
            current: src.get(0).unwrap(),
            offset: 0usize,
        }
    }

    pub fn parse(&mut self) {
        'parse: loop {
            if let Some(n) = self.parse_expr() {
                self.add_node(n);
            }

            if self.idx + 1 < self.src.len() {
                self.idx += 1;
                self.idx += self.offset;
                self.offset = 0;
                self.current = self.src.get(self.idx).unwrap();
                continue 'parse;
            } else {
                break 'parse;
            }
        }
    }
    
    fn parse_expr(&mut self) -> Option<Node> {
        match &self.current.kind {
            TokenKind::Plus => {
                if let Some(be) = self.parse_binary_expr() {
                    return Some(Node::BinaryExpr(be))
                } else {
                    return None;
                }
            },
            TokenKind::Literal(_) => {
                if let Some(result) = self.parse_literal() {
                    return Some(result)
                } else {
                    return None;
                }
            },
            _ => None,
        }
    }

    fn parse_binary_expr(&mut self) -> Option<BinaryExpr> {
        // Get operator
        let op: BinaryOperator;
        match &self.current.kind {
            TokenKind::Plus => op = BinaryOperator::Plus,
            TokenKind::Minus => op = BinaryOperator::Minus,
            TokenKind::Star => op = BinaryOperator::Multiply,
            TokenKind::Slash => op = BinaryOperator::Divide,
            _ => panic!("Expected a binary operator")
        }

        // Get leaf nodes
        let option_lhs = self.expect_number(1, "bwd");
        let option_rhs = self.expect_number(1, "fwd");
        self.offset += 1;

        // Evaluate validity of numbers and create expr
        if option_lhs.is_some() && option_rhs.is_some() {
            Some(BinaryExpr { 
                lhs: Box::new(option_lhs.unwrap()),
                rhs: Box::new(option_rhs.unwrap()),
                op,
            })
        } else {
            None
        }
    }   

    fn parse_number(&mut self) -> Option<Node> {
        match &self.current.kind {
            TokenKind::Number(value) => {
                let val: i32 = value.parse().expect("Error parsing number value");
                Some(Node::Integer(val))
            },
            _ => panic!("{:#?}, Not a number!", &self.current.kind)
        }
    }

    fn parse_literal(&mut self) -> Option<Node> {
        match &self.current.kind {
            TokenKind::Literal(value) => {
                Some(Node::String(value.to_string()))
            },
            _ => panic!("{:#?}, Not a literal!", &self.current.kind)
        }
    }

    fn expect_number(&mut self, offset: usize, dir: &str) -> Option<Node> {
        match dir {
            "fwd" => {
                if self.idx + offset <= self.src.len() {
                    self.current = self.src.get(self.idx + offset).unwrap();
                    if let Some(n) = self.parse_number() {
                        Some(n)
                    } else {
                        None
                    }
                } else {
                    return None;
                }
            },
            "bwd" => {
                self.current = self.src.get(self.idx.saturating_sub(offset)).unwrap();
                if let Some(n) = self.parse_number() {
                    Some(n)
                } else {
                    None
                }
            },
            _ => panic!("Invalid direction")
        }
    }

    fn add_node(&mut self, node: Node) {
        self.ast.add(node)
    }
}