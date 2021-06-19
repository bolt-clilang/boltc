use crate::Token;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Size {
    Int,
    Byte,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub size: Size,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Import {
    pub name: Vec<String>,
}

#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub func: Vec<Function>,
    pub globals: Vec<Statement>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Bool,
    Str,
    Void,
    Int,
    Mlstr,
    Char,
}

#[derive(Debug)]
pub struct Function {
    pub is_async: bool,
    pub name: String,
    pub return_type: Type,
    pub arguments: Vec<Variable>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expression {
    BinOp(BinOp, Box<Expression>, Box<Expression>),
    UnOp(UnOp, Box<Expression>),
    Int(u32),
    Char(u64),
    MLStr(u64),
    FunctionCall(String, Vec<Expression>),
    Variable(String),
    VariableRef(String),
    Assign(String, Box<Expression>),
    AssignPostfix(String, Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Statement {
    Declare(Variable, Option<Expression>),
    Return(Expression),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Exp(Expression),
    Compound(Vec<Statement>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnOp {
    Negation,
    BitComp,
    LogicalNeg,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BinOp {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
    BitwiseLeft,
    BitwiseRight,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    Comma,
}

impl From<Token> for BinOp {
    fn from(token: Token) -> Self {
        match token {
            Token::Multiplication => BinOp::Multiplication,
            Token::Division => BinOp::Division,
            Token::Modulus => BinOp::Modulus,
            Token::Addition => BinOp::Addition,
            Token::Negation => BinOp::Subtraction,
            Token::LessThan => BinOp::LessThan,
            Token::LessThanOrEqual => BinOp::LessThanOrEqual,
            Token::GreaterThan => BinOp::GreaterThan,
            Token::GreaterThanOrEqual => BinOp::GreaterThanOrEqual,
            Token::Equal => BinOp::Equal,
            Token::NotEqual => BinOp::NotEqual,
            Token::And => BinOp::And,
            Token::Or => BinOp::Or,
            Token::BitwiseLeft => BinOp::BitwiseLeft,
            Token::BitwiseRight => BinOp::BitwiseRight,
            Token::BitwiseAnd => BinOp::BitwiseAnd,
            Token::BitwiseXor => BinOp::BitwiseXor,
            Token::BitwiseOr => BinOp::BitwiseOr,
            Token::Comma => BinOp::Comma,
            other => panic!("Token {:?} cannot be converted into a BinOp", other),
        }
    }
}

impl From<Token> for UnOp {
    fn from(token: Token) -> Self {
        match token {
            Token::Negation => UnOp::Negation,
            Token::LogicalNeg => UnOp::LogicalNeg,
            Token::BitComp => UnOp::BitComp,
            other => panic!("Unsupported token {:?}, can only use: ! ~ -", other),
        }
    }
}
