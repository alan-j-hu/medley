use crate::model;
use druid::{Data, RenderContext};
use std::vec::Vec;

#[derive(Clone, Data)]
pub struct GlobalData {}

pub struct Editor<O, S> {
    snippets: Vec<druid::WidgetPod<(GlobalData, model::Expr<O>), Block<O, S>>>,
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
    children: Vec<druid::WidgetPod<(GlobalData, Option<model::Expr<O>>), Hole<O, S>>>,
    labels: Vec<druid::widget::Label<()>>,
    syntax: S,
}

impl<O, S> druid::Widget<(GlobalData, model::Expr<O>)> for Block<O, S>
where
    O: model::Operator,
    S: Syntax<O>,
{
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx<'_, '_>,
        event: &druid::Event,
        data: &mut (GlobalData, model::Expr<O>),
        env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx<'_, '_>,
        event: &druid::LifeCycle,
        data: &(GlobalData, model::Expr<O>),
        env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx<'_, '_>,
        old_data: &(GlobalData, model::Expr<O>),
        data: &(GlobalData, model::Expr<O>),
        env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx<'_, '_>,
        bc: &druid::BoxConstraints,
        (gd, expr): &(GlobalData, model::Expr<O>),
        env: &druid::Env,
    ) -> druid::Size {
        let (width, height) = self.syntax.production(expr.operator()).iter().fold(
            (0.0, 0.0),
            |(width, height), symbol| {
                let size = match symbol {
                    Symbol::Child(i) => self.children[*i].layout(
                        ctx,
                        bc,
                        &(gd.clone(), expr.children()[*i].clone()),
                        env,
                    ),
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
        (gd, expr): &(GlobalData, model::Expr<O>),
        env: &druid::Env,
    ) {
        let size = ctx.size();
        let rect = druid::Size::new(size.width - 2.0, size.height - 2.0)
            .to_rect()
            .to_rounded_rect(2.0);
        ctx.fill(rect, &druid::Color::rgb8(255, 0, 0));
        for (i, child) in self.children.iter_mut().enumerate() {
            child.paint(ctx, &(gd.clone(), expr.children().clone()[i].clone()), env)
        }
        for label in &mut self.labels {
            label.paint(ctx, &(), env)
        }
    }
}

pub struct Hole<O, S> {
    block: druid::WidgetPod<(GlobalData, model::Expr<O>), Block<O, S>>,
    cached_size: Option<druid::Size>,
}

impl<O, S> druid::Widget<(GlobalData, Option<model::Expr<O>>)> for Hole<O, S>
where
    O: model::Operator,
    S: Syntax<O>,
{
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx<'_, '_>,
        event: &druid::Event,
        data: &mut (GlobalData, Option<model::Expr<O>>),
        env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx<'_, '_>,
        event: &druid::LifeCycle,
        data: &(GlobalData, Option<model::Expr<O>>),
        env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx<'_, '_>,
        old_data: &(GlobalData, Option<model::Expr<O>>),
        data: &(GlobalData, Option<model::Expr<O>>),
        env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx<'_, '_>,
        bc: &druid::BoxConstraints,
        (gd, expr): &(GlobalData, Option<model::Expr<O>>),
        env: &druid::Env,
    ) -> druid::Size {
        match self.cached_size {
            Some(size) => size,
            None => {
                let size = match expr {
                    None => druid::Size::new(0.0, 0.0),
                    Some(expr) => self.block.layout(ctx, bc, &(gd.clone(), expr.clone()), env),
                };
                self.cached_size = Some(size);
                size
            }
        }
    }

    fn paint(
        &mut self,
        ctx: &mut druid::PaintCtx<'_, '_, '_>,
        (gd, expr): &(GlobalData, Option<model::Expr<O>>),
        env: &druid::Env,
    ) {
        match expr {
            None => {}
            Some(expr) => self.block.paint(ctx, &(gd.clone(), expr.clone()), env),
        }
    }
}
