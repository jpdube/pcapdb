use std::fmt;

pub struct Select {
    pub fields: Vec<String>
}

impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SELECT : {:#?}", self.fields)
    }
}

pub struct From {
    pub sources: Vec<String>
}

impl fmt::Display for From {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FROM : {:#?}", self.sources)
    }
}

pub struct Where {
    pub expressions: Vec<Expression>
}

impl fmt::Display for Where {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WEHRE : {:#?}", self.expressions)
    }
}

#[derive(Debug)]
pub struct Expression {
    pub expressions: Vec<Expression>
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EXPRESSION : {:#?}", self.expressions)
    }
}

pub struct BinOp {
    pub op: String,
    pub left: Expression,
    pub right: Expression
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BinOP op: {}, left: {}, right: {}", self.op, self.left, self.right)
    }
}
