//! Contains the [`EdgeInfoBox`] component.

use leptos::*;

use crate::{
    components::atoms::{
        CanvasInfoBox,
        TextWithEdit,
    },
    models::LineID,
    utils::{
        color_to_hex,
        parse_color,
    },
    MapState,
};

#[component]
fn LineInfo(line_id: LineID, i: usize) -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let line = move || {
        map_state
            .get()
            .get_map()
            .get_line(line_id)
            .cloned()
            .expect("Can't find line.")
    };

    let edit_line_color = move |line_id: LineID, new_color: String| {
        if let Ok(color) = parse_color(&new_color) {
            map_state.update(|state| {
                if let Some(line) = state
                    .get_mut_map()
                    .get_mut_line(line_id)
                {
                    line.set_color(color);
                }
            });
        }
    };

    view! {
        {
            if i > 0 {view!{
                <hr class="my-0.5"/>
            }.into_view()} else {
                view!{
                <></>
            }.into_view()}
        }
        <p class="text-md font-semibold"><b>"Name:\n"</b> {
            if line().get_name().is_empty() {"Unnamed".to_owned()} else {line().get_name().to_owned()}
        }</p>
        <p class="text-md font-semibold">
            <b>"Color:\n"</b>
            <span style:color=color_to_hex(line().get_color())>
                <TextWithEdit
                    edit_label={"Edit line color".to_owned()}
                    text=color_to_hex(line().get_color())
                    on_edit=move |s| edit_line_color(line_id, s)/>
            </span>
        </p>
    }
}

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
    let edge_lines = Signal::derive(move || {
        let state = map_state.get();
        state
            .get_clicked_on_edge()
            .map_or(Vec::new(), |e| {
                e.get_lines()
                    .to_vec()
            })
            .into_iter()
            .enumerate()
    });

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
                    <For each=edge_lines
                        key=|(_, line)| *line
                        children=move |(i, line_id)| {
                            view!{
                                <LineInfo line_id=line_id i=i/>
                            }
                        }
                    />
                </div>
            </CanvasInfoBox>
        </Show>
    }
}
