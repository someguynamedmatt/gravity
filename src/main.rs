extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/*
 * F = G(m1 * m2)/r^2
 */


// I stole this from an example. It "just works".
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str, c: &str);
}

#[wasm_bindgen]
pub fn what() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    (document.body().unwrap().as_ref() as &web_sys::Node)
        .append_child(canvas.as_ref() as &web_sys::Node)
        .unwrap();
    canvas.set_width(640);
    canvas.set_height(480);
    (canvas.as_ref() as &web_sys::HtmlElement)
        .style()
        .set_property("border", "solid")
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let context = Rc::new(context);
    let closure: Closure<FnMut(_)> = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        log("Clicked it!");
        log_many("Creating arc at (x,y): ", &event.offset_x().to_string(), &event.offset_y().to_string());
        context.set_fill_style(&wasm_bindgen::JsValue::from_str("#efefef"));
        context.fill();

        // .arc(x-pos, y-pos, radius, ...who cares)
        context.arc(event.offset_x() as f64, event.offset_y() as f64, 5.0, 0.0, 3.14 * 2.0);
        context.stroke();
        context.begin_path();
    }));

    (canvas.as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

}

fn main() {}
