//! Contains the [`StationInfoBox`] component.

use leptos::*;

use crate::{
    components::atoms::{
        CanvasInfoBox,
        TextWithEdit,
    },
    models::StationID,
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
                logging::log!(
                    "Name of station: {}: {}",
                    s.get_id(),
                    s.get_name()
                );
                if s.get_name()
                    .is_empty()
                {
                    return "Unnamed".to_string();
                }

                s.get_name()
                    .to_string()
            })
    };
    let station_id = move || {
        map_state
            .get()
            .get_clicked_on_station()
            .map(|s| s.get_id())
    };

    let edit_station_name = move |station_id_opt: Option<StationID>, new_name: String| {
        if let Some(station_id) = station_id_opt {
            map_state.update(|state| {
                let updated = if let Some(station) = state
                    .get_mut_map()
                    .get_mut_station(station_id)
                {
                    station.set_name(&new_name);
                    station.clone()
                } else {
                    return;
                };

                state.set_clicked_on_station(updated);
            });
        }
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
                    <span class="text-md font-semibold"><b>"Name:\n"</b>
                        <TextWithEdit
                            edit_label={"Edit station name".to_owned()}
                            text=station_name
                            on_edit=move |s| edit_station_name(station_id(), s)/>
                    </span>
                </div>
            </CanvasInfoBox>
        </Show>
    }
}
