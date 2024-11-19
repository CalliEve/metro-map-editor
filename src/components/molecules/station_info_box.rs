//! Contains the [`StationInfoBox`] component.

use leptos::*;

use crate::{
    components::atoms::CanvasInfoBox,
    MapState,
};

/// A canvas info box that shows information about a station and lets you change
/// its name.
#[component]
pub fn StationInfoBox() -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let station_was_clicked = move || {
        map_state
            .get()
            .get_clicked_on_station()
            .is_some()
    };
    let position = Signal::derive(move || {
        map_state
            .get()
            .get_clicked_on_station()
            .map(|s| {
                s.get_canvas_pos(
                    map_state
                        .get()
                        .get_canvas_state(),
                )
            })
    });
    let station_name = move || {
        map_state
            .get()
            .get_clicked_on_station()
            .map_or("".to_string(), |s| {
                if s.get_name()
                    .is_empty()
                {
                    return "Unnamed".to_string();
                }

                s.get_name()
                    .to_string()
            })
    };

    view! {
        <Show when=station_was_clicked>
            <CanvasInfoBox
                title="Station Info"
                click_position=position
                on_close=move || {
                    map_state.update(|state| {
                        state.clear_clicked_on_station();
                    });
                }>
                <div>
                    <span class="text-md font-semibold"><b>Name: </b> {station_name()}</span>
                </div>
            </CanvasInfoBox>
        </Show>
    }
}
