use ginto_diagnostics::Spanned;

#[derive(Clone, Debug)]
pub enum Type {
    Bool,
    U64,
    I64,
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEq,
    Less,
    Le,
    Greater,
    Ge,
    Or,
    And,
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Int(u64),
    Bool(bool),
    Unary {
        op: Spanned<UnaryOp>,
        expr: Box<Spanned<Expr>>,
    },
    Binary {
        op: Spanned<BinaryOp>,
        lhs: Box<Spanned<Expr>>,
        rhs: Box<Spanned<Expr>>,
    },
    Let {
        name: Spanned<String>,
        ty: Option<Spanned<Type>>,
        value: Box<Spanned<Expr>>,
    },
    Assign {
        name: Spanned<String>,
        value: Box<Spanned<Expr>>,
    },
    Ident {
        exprs: Vec<Spanned<Expr>>,
        tail: Box<Spanned<Expr>>,
    },
    Var(Spanned<String>),
}

#[derive(Clone, Debug)]
pub enum Param {
    Named {
        name: Spanned<String>,
        ty: Option<Spanned<Type>>,
    },
}

#[derive(Clone, Debug)]
pub struct Func {
    pub name: Spanned<String>,
    pub params: Vec<Spanned<Param>>,
    pub ty: Option<Spanned<Type>>,
    pub body: Spanned<Expr>,
}
