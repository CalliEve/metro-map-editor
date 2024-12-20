//! Contains the [`Sidebar`] component.

use leptos::prelude::*;

use crate::{
    components::{
        atoms::{
            Button,
            ButtonGroup,
            ButtonProps,
        },
        state::ActionType,
        MapState,
    },
    models::{
        Line,
        Map,
        SelectedLine,
        SelectedStation,
        Station,
    },
};

/// The sidebar component with all the tools on there for editing the canvas.
#[component]
pub fn Sidebar() -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let action_selected = move |action| {
        Signal::derive(move || {
            map_state
                .get()
                .get_selected_action()
                == Some(action)
        })
    };

    let update_action = move |action| {
        if map_state
            .get_untracked()
            .get_selected_action()
            == Some(action)
        {
            map_state.update(|state| {
                state.clear_selected_action();
            });
        } else {
            map_state.update(|state| {
                state.clear_all_selections();
                state.set_selected_action(action);
            });
        }
    };

    let add_station = move |_| {
        map_state.update(|state| {
            state.clear_all_selections();
            state.select_station(SelectedStation::new_station());
        });
    };

    let add_checkpoint = move |_| {
        map_state.update(|state| {
            state.clear_all_selections();
            state.select_station(SelectedStation::new_checkpoint());
        });
    };

    let add_line = move |_| {
        map_state.update(|state| {
            state.clear_all_selections();
            let line = SelectedLine::new_line(state.get_mut_map());
            state.set_selected_lines(vec![line]);
        });
    };

    let remove_station = move |_| update_action(ActionType::RemoveStation);
    let remove_station_selected = action_selected(ActionType::RemoveStation);

    let remove_checkpoint = move |_| update_action(ActionType::RemoveCheckpoint);
    let remove_checkpoint_selected = action_selected(ActionType::RemoveCheckpoint);

    let remove_line = move |_| update_action(ActionType::RemoveLine);
    let remove_line_selected = action_selected(ActionType::RemoveLine);

    let lock = move |_| {
        let state = map_state.get();
        if !state
            .get_selected_edges()
            .is_empty()
            || !state
                .get_selected_stations()
                .is_empty()
        {
            map_state.update(|state| {
                state.lock_selected();
                state.clear_all_selections();
            });
            return;
        }
        update_action(ActionType::Lock);
    };
    let lock_selected = action_selected(ActionType::Lock);

    let unlock = move |_| {
        let state = map_state.get();
        if !state
            .get_selected_edges()
            .is_empty()
            || !state
                .get_selected_stations()
                .is_empty()
        {
            map_state.update(|state| {
                state.unlock_selected();
                state.clear_all_selections();
            });
            return;
        }
        update_action(ActionType::Unlock);
    };
    let unlock_selected = action_selected(ActionType::Unlock);

    view! {
        <div id="sidebar" class="h-full w-full flex flex-col gap-y-4 bg-zinc-100 py-2 shadow-right shadow-dark-mild dark:shadow-black dark:bg-neutral-750 text-black dark:text-white px-2">
            <Button
                on_click=Box::new(move |_| map_state.update(|state| {
                    state.clear_all_selections();
                    state.set_map(state.get_last_loaded().cloned().unwrap_or_else(testmap));
                }))
                text="reset map" />
            <ButtonGroup
                children={vec![
                    ButtonProps::builder()
                        .text("Add Station")
                        .on_click(Box::new(add_station))
                        .can_focus(true)
                        .build(),
                    ButtonProps::builder()
                        .text("Remove Station")
                        .on_click(Box::new(remove_station))
                        .active(remove_station_selected)
                        .danger(true)
                        .build(),
                ]}/>
            <ButtonGroup
                children={vec![
                    ButtonProps::builder()
                        .text("Add Line")
                        .on_click(Box::new(add_line))
                        .can_focus(true)
                        .build(),
                    ButtonProps::builder()
                        .text("Remove Line")
                        .on_click(Box::new(remove_line))
                        .active(remove_line_selected)
                        .danger(true)
                        .build(),
                ]}/>
            <ButtonGroup
                children={vec![
                    ButtonProps::builder()
                        .text("Lock")
                        .on_click(Box::new(lock))
                        .active(lock_selected)
                        .build(),
                    ButtonProps::builder()
                        .text("Unlock")
                        .on_click(Box::new(unlock))
                        .active(unlock_selected)
                        .danger(true)
                        .build(),
                ]}/>
                <ButtonGroup
                children={vec![
                    ButtonProps::builder()
                        .text("Add Checkpoint")
                        .on_click(Box::new(add_checkpoint))
                        .can_focus(true)
                        .build(),
                    ButtonProps::builder()
                        .text("Remove Checkpoint")
                        .on_click(Box::new(remove_checkpoint))
                        .active(remove_checkpoint_selected)
                        .danger(true)
                        .build(),
                ]}/>
        </div>
    }
}

/// Temporary function to load in a test metro map.
fn testmap() -> Map {
    let mut map = Map::new();

    let station1 = Station::new((10, 10).into(), None);
    let station1_id = station1.get_id();
    let station2 = Station::new((15, 15).into(), None);
    let station2_id = station2.get_id();
    let station3 = Station::new((20, 25).into(), None);
    let station3_id = station3.get_id();
    let station4 = Station::new((20, 12).into(), None);
    let station4_id = station4.get_id();
    let station5 = Station::new((25, 12).into(), None);
    let station5_id = station5.get_id();
    let station6 = Station::new((30, 20).into(), None);
    let station6_id = station6.get_id();
    let station7 = Station::new((7, 5).into(), None);
    let station7_id = station7.get_id();

    map.add_station(station1);
    map.add_station(station2);
    map.add_station(station3);
    map.add_station(station4);
    map.add_station(station5);
    map.add_station(station6);
    map.add_station(station7);

    let mut line1 = Line::new(None);
    let mut line2 = Line::new(None);
    let mut line3 = Line::new(None);

    line1.add_station(
        &mut map,
        station1_id,
        Some(station2_id),
        None,
    );
    line1.add_station(
        &mut map,
        station2_id,
        Some(station3_id),
        None,
    );
    line1.add_station(&mut map, station3_id, None, None);
    map.add_line(line1);

    line2.add_station(
        &mut map,
        station4_id,
        Some(station5_id),
        None,
    );
    line2.add_station(
        &mut map,
        station5_id,
        Some(station6_id),
        None,
    );
    line2.add_station(&mut map, station6_id, None, None);
    map.add_line(line2);

    line3.add_station(&mut map, station7_id, None, None);
    map.add_line(line3);

    map
}
