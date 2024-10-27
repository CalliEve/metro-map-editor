//! Contains the [`SettingsModal`] component.

use std::path::Path;

use ev::MouseEvent;
use leptos::*;
use wasm_bindgen::{
    closure::Closure,
    JsCast,
    JsValue,
};
use web_sys::HtmlInputElement;

use crate::{
    components::atoms::{
        Button,
        Modal,
        NumberInput,
        Toggle,
    },
    unwrap_or_return,
    Error,
    MapState,
};

/// A modal for changing the settings of the algorithm.
#[component]
pub fn SettingsModal<C>(
    /// If the modal should be shown.
    show: ReadSignal<bool>,
    /// Gets called if the modal is closed.
    on_close: C,
) -> impl IntoView
where
    C: Fn() + Clone + 'static,
{
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let update_square_size = move |mut n: f64| {
        if n < 1.0 {
            n = 1.0;
        }

        map_state
            .update(|state| state.update_canvas_state(|canvas| canvas.set_square_size(n as u32)));
    };

    view! {
        <Modal show=show on_close=on_close.clone()>
            // body
            <div class="p-4 md:p-5 space-y-4">
                <Toggle
                    text="Enable debug output."
                    value=move || map_state.get().get_algorithm_settings().debug
                    on_input=move |b| {
                        map_state
                            .update(|state| state.update_algorithm_settings(|settings| {
                                settings.debug = b;
                            }));
                    }/>
                <Toggle
                    text="Enable local search result optimization."
                    value=move || map_state.get().get_algorithm_settings().local_search
                    on_input=move |b| {
                        map_state
                            .update(|state| state.update_algorithm_settings(|settings| {
                                settings.local_search = b;
                            }));
                    }/>
                <Toggle
                    text="Enable station relocation (off equals setting the node-set radius to 0)."
                    value=move || map_state.get().get_algorithm_settings().allow_station_relocation
                    on_input=move |b| {
                        map_state
                            .update(|state| state.update_algorithm_settings(|settings| {
                                settings.allow_station_relocation = b;
                            }));
                    }/>
                <Toggle
                    text="On failure of the algorithm, output the map at point of failure anyway."
                    value=move || map_state.get().get_algorithm_settings().output_on_fail
                    on_input=move |b| {
                        map_state
                            .update(|state| state.update_algorithm_settings(|settings| {
                                settings.output_on_fail = b;
                            }));
                    }/>
                <NumberInput
                    text="Set canvas grid size."
                    min=2.0
                    max=100.0
                    value=move || f64::from(map_state.get().get_canvas_state().get_square_size())
                    on_input=update_square_size/>
                <NumberInput
                    text="Set maximum number of algorithm iterations."
                    min=1.0
                    max=20.0
                    value=move || map_state.get().get_algorithm_settings().edge_routing_attempts as f64
                    on_input=move |n| {
                        map_state
                            .update(|state| state.update_algorithm_settings(|settings| {
                                settings.edge_routing_attempts = n.round().abs() as usize;
                            }));
                    }/>
                <NumberInput
                    text="Set the node-set radius for possible station placement in algorithm pathfinding."
                    min=0.0
                    max=20.0
                    value=move || f64::from(map_state.get().get_algorithm_settings().node_set_radius)
                    on_input=move |n| {
                        map_state
                            .update(|state| state.update_algorithm_settings(|settings| {
                                settings.node_set_radius = n.round().abs() as i32;
                            }));
                    }/>
                <NumberInput
                    text="Set the cost for extending an edge with one node."
                    min=0.0
                    max=10.0
                    step=0.1
                    value=move || map_state.get().get_algorithm_settings().move_cost
                    on_input=move |n| {
                        map_state
                            .update(|state| state.update_algorithm_settings(|settings| {
                                settings.move_cost = n;
                            }));
                    }/>
            </div>
            // footer
            <div class="flex items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600">
                <Button text="Done" on_click=Box::new(move |_| on_close())/>
            </div>
        </Modal>
    }
}
