use druid::Data;

#[derive(Clone, Data)]
pub struct Editor<O> {
    snippets: im::vector::Vector<Snippet<O>>,
}

impl<O: Clone> Default for Editor<O> {
    fn default() -> Self {
        Editor {
            snippets: std::default::Default::default(),
        }
    }
}

#[derive(Clone, Data)]
pub struct Snippet<O> {
    expr: Expr<O>,
}

#[derive(Clone, Data)]
pub struct Expr<O> {
    operator: O,
    children: im::vector::Vector<Expr<O>>,
}

pub enum Symbol {
    Child(usize),
    Label(&'static str),
}

pub trait Operator: Eq + druid::Data {
    type Sort: Eq;

    fn arity(self: &Self) -> (&'static [Self::Sort], Self::Sort);
}

pub trait Syntax<O: Operator> {
    fn production(operator: &O) -> &'static [Symbol];
}
