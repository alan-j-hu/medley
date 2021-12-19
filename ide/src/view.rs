use crate::model;
use druid::{Data, RenderContext};
use std::vec::Vec;

pub struct Editor<O, S> {
    snippets: Vec<druid::WidgetPod<model::Expr<O>, Block<O, S>>>,
}

impl<O, S> Editor<O, S> {
    pub fn new() -> Self {
        Editor {
            snippets: Vec::new(),
        }
    }
}

impl<O, S> druid::Widget<model::Editor<O>> for Editor<O, S> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx<'_, '_>,
        event: &druid::Event,
        _: &mut model::Editor<O>,
        env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx<'_, '_>,
        event: &druid::LifeCycle,
        _: &model::Editor<O>,
        env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx<'_, '_>,
        _: &model::Editor<O>,
        _: &model::Editor<O>,
        env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx<'_, '_>,
        bc: &druid::BoxConstraints,
        _: &model::Editor<O>,
        env: &druid::Env,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(
        &mut self,
        ctx: &mut druid::PaintCtx<'_, '_, '_>,
        _: &model::Editor<O>,
        env: &druid::Env,
    ) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &druid::Color::grey8(100));
    }
}

pub enum Symbol {
    Child(usize),
    Label(&'static str),
}

pub trait Syntax<O> {
    fn production(&self, operator: &O) -> &'static [Symbol];
}

pub struct Block<O, S> {
    children: Vec<druid::WidgetPod<Option<model::Expr<O>>, Hole<O, S>>>,
    labels: Vec<druid::widget::Label<()>>,
    syntax: S,
}

impl<O, S> druid::Widget<model::Expr<O>> for Block<O, S>
where
    O: model::Operator,
    S: Syntax<O>,
{
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx<'_, '_>,
        event: &druid::Event,
        data: &mut model::Expr<O>,
        env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx<'_, '_>,
        event: &druid::LifeCycle,
        data: &model::Expr<O>,
        env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx<'_, '_>,
        old_data: &model::Expr<O>,
        data: &model::Expr<O>,
        env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx<'_, '_>,
        bc: &druid::BoxConstraints,
        expr: &model::Expr<O>,
        env: &druid::Env,
    ) -> druid::Size {
        let (width, height) = self.syntax.production(expr.operator()).iter().fold(
            (0.0, 0.0),
            |(width, height), symbol| {
                let size = match symbol {
                    Symbol::Child(i) => {
                        self.children[*i].layout(ctx, bc, &expr.children()[*i], env)
                    }
                    Symbol::Label(_) => druid::Size::new(0.0, 0.0),
                };
                (
                    width + size.width,
                    if height > size.height {
                        height
                    } else {
                        size.height
                    },
                )
            },
        );
        druid::Size::new(width + 2.0, height + 2.0)
    }

    fn paint(
        &mut self,
        ctx: &mut druid::PaintCtx<'_, '_, '_>,
        expr: &model::Expr<O>,
        env: &druid::Env,
    ) {
        let size = ctx.size();
        let rect = druid::Size::new(size.width - 2.0, size.height - 2.0)
            .to_rect()
            .to_rounded_rect(2.0);
        ctx.fill(rect, &druid::Color::rgb8(255, 0, 0));
        for (i, child) in self.children.iter_mut().enumerate() {
            child.paint(ctx, &expr.children().clone()[i], env)
        }
        for label in &mut self.labels {
            label.paint(ctx, &(), env)
        }
    }
}

pub struct Hole<O, S> {
    block: druid::WidgetPod<model::Expr<O>, Block<O, S>>,
    cached_size: Option<druid::Size>,
}

impl<O, S> druid::Widget<Option<model::Expr<O>>> for Hole<O, S>
where
    O: model::Operator,
    S: Syntax<O>,
{
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx<'_, '_>,
        event: &druid::Event,
        data: &mut Option<model::Expr<O>>,
        env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx<'_, '_>,
        event: &druid::LifeCycle,
        data: &Option<model::Expr<O>>,
        env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx<'_, '_>,
        old_data: &Option<model::Expr<O>>,
        data: &Option<model::Expr<O>>,
        env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx<'_, '_>,
        bc: &druid::BoxConstraints,
        expr: &Option<model::Expr<O>>,
        env: &druid::Env,
    ) -> druid::Size {
        match self.cached_size {
            Some(size) => size,
            None => {
                let size = match expr {
                    None => druid::Size::new(0.0, 0.0),
                    Some(expr) => self.block.layout(ctx, bc, expr, env),
                };
                self.cached_size = Some(size);
                size
            }
        }
    }

    fn paint(
        &mut self,
        ctx: &mut druid::PaintCtx<'_, '_, '_>,
        expr: &Option<model::Expr<O>>,
        env: &druid::Env,
    ) {
        match expr {
            None => {}
            Some(expr) => self.block.paint(ctx, expr, env),
        }
    }
}
