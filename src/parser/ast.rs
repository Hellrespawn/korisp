use crate::interpreter::Visitor;

#[derive(Debug, Clone)]
pub enum Node {
    Program(Program),
    IfExpression(IfExpression),
    SetExpression(SetExpression),
    List(List),
    Atom(Atom),
}

impl Node {
    pub fn accept<T: std::fmt::Debug>(
        &self,
        visitor: &mut dyn Visitor<T>,
    ) -> T {
        match self {
            Node::Program(program) => visitor.visit_program(program),
            Node::IfExpression(if_expression) => {
                visitor.visit_if_expression(if_expression)
            }
            Node::SetExpression(if_expression) => {
                visitor.visit_set_expression(if_expression)
            }
            Node::List(list) => visitor.visit_list(list),
            Node::Atom(atom) => visitor.visit_atom(atom),
        }
    }
}

impl From<Program> for Node {
    fn from(program: Program) -> Self {
        Self::Program(program)
    }
}

impl From<IfExpression> for Node {
    fn from(if_expression: IfExpression) -> Self {
        Self::IfExpression(if_expression)
    }
}

impl From<SetExpression> for Node {
    fn from(set_expression: SetExpression) -> Self {
        Self::SetExpression(set_expression)
    }
}

impl From<List> for Node {
    fn from(list: List) -> Self {
        Self::List(list)
    }
}

impl From<Atom> for Node {
    fn from(atom: Atom) -> Self {
        Self::Atom(atom)
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub expressions: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct List {
    pub elements: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Box<Node>,
    pub then_branch: Box<Node>,
    pub else_branch: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct SetExpression {
    pub symbol: String,
    pub expression: Box<Node>,
}

#[derive(Debug, Clone)]
pub enum Atom {
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    Nil,
}
