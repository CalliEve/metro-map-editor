//! Contains the [`EdgeInfoBox`] component.

use leptos::*;

use crate::{
    components::atoms::CanvasInfoBox,
    utils::color_to_hex,
    MapState,
};

/// A canvas info box that shows information about an edge and lets you change
/// the name and color of the lines that follow it.
#[component]
pub fn EdgeInfoBox() -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let edge_was_clicked = move || {
        map_state
            .get()
            .get_clicked_on_edge()
            .is_some()
    };
    let position = Signal::derive(move || {
        map_state
            .get()
            .get_clicked_on_edge_location()
    });
    let edge_lines = move || {
        map_state
            .get()
            .get_clicked_on_edge()
            .map_or(Vec::new(), |e| {
                e.get_lines()
                    .iter()
                    .map(|l| {
                        map_state
                            .get()
                            .get_map()
                            .get_line(*l)
                            .expect("Can't find line on edge.")
                            .clone()
                    })
                    .collect()
            })
    };

    view! {
        <Show when=edge_was_clicked>
            <CanvasInfoBox
                title="Edge Info"
                click_position=position
                on_close=move || {
                    map_state.update(|state| {
                        state.clear_clicked_on_station();
                    });
                }>
                <div>
                    <For each=move || edge_lines().into_iter().enumerate()
                        key=|(_, line)| line.get_id()
                        children=move |(i, line)| {view!{
                            {if i > 0 {view!{
                                <hr class="my-0.5"/>
                            }.into_view()} else {view!{
                                <></>
                            }.into_view()}}
                            <p class="leading-tight text-md font-semibold"><b>Name: </b> {
                                if line.get_name().is_empty() {"Unnamed".to_owned()} else {line.get_name().to_owned()}
                            }</p>
                            <p class="leading-tight text-md font-semibold"><b>Color: </b> <span style:color=color_to_hex(line.get_color())>{color_to_hex(line.get_color())}</span></p>
                        }}/>
                </div>
            </CanvasInfoBox>
        </Show>
    }
}
