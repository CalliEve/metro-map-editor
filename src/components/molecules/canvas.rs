use std::sync::atomic::{AtomicBool, Ordering};

use ev::UiEvent;
use leptos::html::Canvas;
use leptos::logging::log;
use leptos::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::state::MapState;

static DOCUMENT_LOADED: AtomicBool = AtomicBool::new(false);

fn calc_canvas_size(map_state: &RwSignal<MapState>) {
    // To have a canvas resize dynamically, we need to manually adjust its size
    // CSS will NOT work, as it will just make everything blurry
    let doc = document();
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
            .expect("height should be a number")
            .round()) as u32;

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
            .expect("width should be a number")
            .round()) as u32;

    map_state.update(|state| state.set_size((height, width)));
}

fn on_mouse_down(
    canvas_ref: &NodeRef<Canvas>,
    map_state_signal: &RwSignal<MapState>,
    ev: &UiEvent,
) {
    let mut map_state = map_state_signal.get();
    let map = if let Some(m) = map_state.get_map().clone() {
        m
    } else {
        return;
    };

    let map_size = map_state.get_size();
    let win_height = window().inner_height().unwrap().as_f64().unwrap().round() as i32;
    let win_width = window().inner_width().unwrap().as_f64().unwrap().round() as i32;
    let pos = (
        ev.page_x() - (win_width - map_size.1 as i32),
        ev.page_y() - (win_height - map_size.0 as i32),
    );

    let mouse_pos = map.calc_nearest_grid_node(map_state.get_square_size(), pos);
    let selected_opt = map.station_at_pos(mouse_pos).map(|s| s.clone_non_ref());
    if selected_opt.is_none() {
        return;
    }
    let mut selected = selected_opt.unwrap();

    selected.set_pos(mouse_pos);
    selected.set_is_ghost(true);
    map_state.set_selected_station(selected);

    map_state.draw_to_canvas(canvas_ref);

    map_state_signal.set(map_state);
}

fn on_mouse_up(canvas_ref: &NodeRef<Canvas>, map_state_signal: &RwSignal<MapState>) {
    let mut map_state = map_state_signal.get();
    if !map_state.has_selected_station() {
        return;
    }

    let map = map_state.get_map().cloned().unwrap();
    let mut selected = map_state.get_selected_station().cloned().unwrap();
    selected.set_is_ghost(false);

    for station in map.get_stations() {
        if *station == selected {
            log!("{:?} -> {:?}", station.get_pos(), selected.get_pos());
            station.set_pos(selected.get_pos());
            break;
        }
    }

    map_state.set_map(map);
    map_state.clear_selected_station();

    map_state.draw_to_canvas(canvas_ref);

    map_state_signal.set(map_state);
}

fn on_mouse_move(
    canvas_ref: &NodeRef<Canvas>,
    map_state_signal: &RwSignal<MapState>,
    ev: &UiEvent,
) {
    let mut map_state = map_state_signal.get();
    let selected_opt = map_state.get_selected_station().clone();
    if selected_opt.is_none() {
        return;
    }
    let selected = selected_opt.cloned().unwrap();

    let map_size = map_state.get_size();
    let win_height = window().inner_height().unwrap().as_f64().unwrap().round() as i32;
    let win_width = window().inner_width().unwrap().as_f64().unwrap().round() as i32;
    let pos = (
        ev.page_x() - (win_width - map_size.1 as i32),
        ev.page_y() - (win_height - map_size.0 as i32),
    );

    let map = map_state.get_map().clone().unwrap();
    let mouse_pos = map.calc_nearest_grid_node(map_state.get_square_size(), pos);
    if mouse_pos == selected.get_pos() {
        return;
    }

    selected.set_pos(mouse_pos);
    map_state.set_selected_station(selected);

    map_state.draw_to_canvas(canvas_ref);

    map_state_signal.set(map_state);
}

fn on_mouse_out(canvas_ref: &NodeRef<Canvas>, map_state_signal: &RwSignal<MapState>) {
    let mut map_state = map_state_signal.get();
    if !map_state.has_selected_station() {
        return;
    }

    map_state.clear_selected_station();

    map_state.draw_to_canvas(canvas_ref);

    map_state_signal.set(map_state);
}

#[component]
pub fn Canvas() -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    create_effect(move |_| {
        calc_canvas_size(&map_state);

        if !DOCUMENT_LOADED.load(Ordering::Relaxed) {
            DOCUMENT_LOADED.store(true, Ordering::Release);
            let f = Closure::<dyn Fn()>::new(move || calc_canvas_size(&map_state));
            window().set_onresize(Some(f.as_ref().unchecked_ref()));
            f.forget();
        }
    });

    create_effect(move |_| {
        let canvas_node = &canvas_ref.get().expect("should be loaded now");
        let s = map_state.get().get_size();
        canvas_node.set_height(s.0);
        canvas_node.set_width(s.1);

        map_state.get().draw_to_canvas(&canvas_ref);
    });

    view! {
        <div class="grow overflow-hidden bg-zinc-50 dark:bg-neutral-700 text-black dark:text-white">
            <canvas
                _ref=canvas_ref
                on:mousedown=move |ev| on_mouse_down(&canvas_ref, &map_state, ev.as_ref())
                on:mouseup=move |_| on_mouse_up(&canvas_ref, &map_state)
                on:mousemove=move |ev| on_mouse_move(&canvas_ref, &map_state, ev.as_ref())
                on:mouseout=move |_| on_mouse_out(&canvas_ref, &map_state)
                on:touchstart=move |ev| on_mouse_down(&canvas_ref, &map_state, ev.as_ref())
                on:touchend=move |_| on_mouse_up(&canvas_ref, &map_state)
                on:touchmove=move |ev| on_mouse_move(&canvas_ref, &map_state, ev.as_ref())
                on:touchcancel=move |_| on_mouse_out(&canvas_ref, &map_state)
                id="canvas"
                style="touch-action: none;"
                class="object-contain"/>
        </div>
    }
}
