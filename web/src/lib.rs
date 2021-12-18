use druid::{AppLauncher, Data, LocalizedString, Scale, WindowDesc};
use std::rc::Rc;
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

const EMPTY_SORT: &'static [Sort] = &[];
const EMPTY_SYMBOL: &'static [medley_ide::view::Symbol] = &[];

impl medley_ide::model::Operator for Operator {
    type Sort = Sort;

    fn arity(self: &Self) -> (&'static [Self::Sort], Self::Sort) {
        (EMPTY_SORT, Sort::Expr)
    }
}

struct Syntax;

impl medley_ide::view::Syntax<Operator> for Syntax {
    fn production(&self, _: &Operator) -> &'static [medley_ide::view::Symbol] {
        EMPTY_SYMBOL
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
    let window = WindowDesc::new(medley_ide::build_root_widget::<Operator, Syntax>)
        .title(LocalizedString::new("Medley"))
        .window_size(scale.px_to_dp_xy(canvas.width(), canvas.height()))
        .resizable(true);

    AppLauncher::with_window(window)
        .launch(medley_ide::model::Editor::new())
        .expect("Could not launch application.");
}
