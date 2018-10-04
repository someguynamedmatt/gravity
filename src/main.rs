extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

/*
 * F = G(m1 * m2)/r^2
 */

#[wasm_bindgen]
pub fn what() {
    let document = web_sys::window().unwrap().document().unwrap();

    // This is how you create a new element and add attributes
    // to it (css). There are other way to do it, but this works
    // too
    let p1 = document.create_element("div").unwrap();
    p1.set_attribute("style", "height:50px;width:50px;border-radius:50%;background:pink;");


    // This is how you put a new element on the document
    (document.body().unwrap().as_ref() as &web_sys::Node)
        .append_child(p1.as_ref() as &web_sys::Node)
        .unwrap();

    let universe = document.get_element_by_id("universe").unwrap();
    let planet = document.get_element_by_id("planet").unwrap();
    planet.set_attribute("style", "top:15px;left:100px;");
}

fn main() {}
