use druid::Widget;

pub mod model;
pub mod view;

pub fn build_root_widget<O: model::Operator, S: view::Syntax<O>>() -> impl Widget<model::Editor<O>>
{
    view::Editor::<O, S>::new()
}
