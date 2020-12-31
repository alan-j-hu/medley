pub enum Type {
    Product(Vec<Type>),
    Exponent(Box<Type>, Box<Type>),
}

pub enum Expr {
    App(Box<Expr>, Box<Expr>),
    Lam(String, Box<Expr>),
    Tup(Vec<Expr>),
    Var(String),
}

pub enum Zipper {
    AppL(Box<Zipper>),
    AppR(Box<Zipper>),
    Lam(Box<Zipper>),
    Tup(Box<Zipper>, Vec<Expr>, usize),
    Root,
}

pub struct Focus {
    pub zipper: Box<Zipper>,
}
