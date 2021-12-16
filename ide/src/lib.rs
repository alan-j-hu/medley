use druid::widget::{Align, Flex};
use druid::Widget;

pub mod model;

pub fn build_root_widget<O: model::Operator>() -> impl Widget<model::Editor<O>> {
    let layout = Flex::column();
    Align::centered(layout)
}
