use druid::{AppLauncher, Data, LocalizedString, Scale, WindowDesc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq, Eq, Data)]
enum Sort {
    Expr,
}

#[derive(Clone, PartialEq, Eq, Data)]
enum Operator {
    Expr,
}

const EMPTY: &'static [Sort] = &[];

impl medley_ide::model::Operator for Operator {
    type Sort = Sort;

    fn arity(self: &Self) -> (&'static [Self::Sort], Self::Sort) {
        (EMPTY, Sort::Expr)
    }
}

#[wasm_bindgen]
pub fn main() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let scale = Scale::new(1.0, 1.0);
    let window = WindowDesc::new(medley_ide::build_root_widget::<Operator>)
        .title(LocalizedString::new("Medley"))
        .window_size(scale.px_to_dp_xy(canvas.width(), canvas.height()))
        .resizable(true);

    AppLauncher::with_window(window)
        .launch(std::default::Default::default())
        .expect("Could not launch application.");
}
