use druid::Data;

#[derive(Clone, Data)]
pub struct Editor<O> {
    snippets: im::vector::Vector<Snippet<O>>,
}

impl<O: Clone> Editor<O> {
    pub fn new() -> Self {
        Editor {
            snippets: std::default::Default::default(),
        }
    }
}

#[derive(Clone, Data)]
pub struct Snippet<O> {
    x: f64,
    y: f64,
    expr: Expr<O>,
}

#[derive(Clone, Data)]
pub struct Expr<O> {
    operator: O,
    children: im::vector::Vector<Option<Expr<O>>>,
}

impl<O> Expr<O> {
    pub fn operator(&self) -> &O {
        &self.operator
    }

    pub fn children(&self) -> &im::vector::Vector<Option<Expr<O>>> {
        &self.children
    }
}

pub trait Operator: Eq + druid::Data {
    type Sort: Eq;

    fn arity(self: &Self) -> (&'static [Self::Sort], Self::Sort);
}
