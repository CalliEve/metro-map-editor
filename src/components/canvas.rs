use leptos::html::Canvas;
use leptos::logging::log;
use leptos::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::algorithm::redraw_canvas;

fn redraw(canvas_node: &HtmlElement<Canvas>) {
    // To have a canvas resize dynamically, we need to manually adjust its size
    // CSS will NOT work, as it will just make everything blurry
    let doc = window().document().expect("should have document");
    let win_height = window().inner_height().unwrap().as_f64().unwrap();
    let win_width = window().inner_width().unwrap().as_f64().unwrap();

    // the navbar borders the top, so the height is `window - navbar`
    let nav = doc
        .get_element_by_id("navbar")
        .expect("navbar should exist");
    let nav_height_px = window()
        .get_computed_style(&nav)
        .unwrap()
        .expect("should have style")
        .get_property_value("height")
        .expect("should have height property");

    let height = (win_height
        - nav_height_px
            .trim_end_matches("px")
            .parse::<f64>()
            .expect("height should be an integer")) as u32;
    canvas_node.set_height(height);

    // the sidebar borders its side, so width is `window - sidebar`
    let side = doc
        .get_element_by_id("sidebar")
        .expect("sidebar should exist");
    let side_width_px = window()
        .get_computed_style(&side)
        .unwrap()
        .expect("should have style")
        .get_property_value("width")
        .expect("should have width property");

    let width = (win_width
        - side_width_px
            .trim_end_matches("px")
            .parse::<f64>()
            .expect("width should be an integer")) as u32;
    canvas_node.set_width(width);

    // Now the canvas is the correct size, we can draw it
    log!("redrawing canvas");
    redraw_canvas(&*canvas_node, (height, width));
}

#[component]
pub fn Canvas() -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();

    create_effect(move |_| {
        let canvas_node = canvas_ref.get().expect("should be loaded now");

        redraw(&canvas_node);

        let f = Closure::<dyn Fn()>::new(move || redraw(&canvas_node));
        window().set_onresize(Some(f.as_ref().unchecked_ref()));
        f.forget();
    });

    view! {
        <div class="grow overflow-hidden bg-zinc-50 dark:bg-neutral-700 text-black dark:text-white">
            <canvas _ref=canvas_ref id="canvas" class="object-contain"/>
        </div>
    }
}
