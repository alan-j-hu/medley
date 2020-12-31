use js_sys::{Function, Object, Reflect, WebAssembly};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

mod ast;
mod emit;
mod view;

#[wasm_bindgen(start)]
pub fn run() {
    spawn_local(async {
        let _three = go(&emit::emit()).await;
    });
    App::<view::View>::new().mount_to_body();
}

async fn go(bytes: &[u8]) -> Result<JsValue, JsValue> {
    // Code adapted from https://rustwasm.github.io/docs/wasm-bindgen/examples/wasm-in-wasm.html
    let a = JsFuture::from(WebAssembly::instantiate_buffer(bytes, &Object::new())).await?;
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    let c = b.exports();

    let add = Reflect::get(c.as_ref(), &"f".into())?
        .dyn_into::<Function>()
        .expect("f export wasn't a function");

    let three = add.call2(&JsValue::undefined(), &1.into(), &2.into())?;
    Ok(three)
}
