use yew::prelude::*;
use crate::ast::*;

pub struct View {
    link: ComponentLink<Self>,
    focus: Focus,
}

impl Component for View {
    type Message = ();
    type Properties = ();

    fn create((): Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            focus: Focus { zipper: Box::new(Zipper::Root) },
        }
    }

    fn update(&mut self, (): Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut html = html! {};
        let mut zipper = &*self.focus.zipper;
        loop {
            match *zipper {
                Zipper::AppL(ref parent) => {
                    html = html!{
                        <div>
                        {"Apply"}
                        {html}
                        </div>
                    };
                    zipper = &*parent;
                },
                Zipper::AppR(ref parent) => {
                    html = html!{
                        <div>
                        {"Apply"}
                        {html}
                        </div>
                    };
                    zipper = &*parent;
                },
                Zipper::Lam(ref parent) => {
                    html = html!{
                        <div>
                        {"Lambda"}
                        {html}
                        </div>
                    };
                    zipper = &*parent;
                },
                Zipper::Tup(ref parent, _, _) => {
                    html = html!{
                        <div>
                        {"Tuple"}
                        {html}
                        </div>
                    };
                    zipper =  &*parent;
                },
                Zipper::Root => {
                    break html;
                }
            }
        }
    }
}
