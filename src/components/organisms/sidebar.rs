//! Contains the [`Sidebar`] component.

use leptos::*;

use crate::{
    components::{
        atoms::{
            Button,
            ButtonGroup,
            ButtonProps,
            NumberInput,
        },
        state::RemoveType,
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

    let add_station = move |_| {
        map_state.update(|state| {
            state.set_selected_station(SelectedStation::new_station());
        });
    };

    let add_line = move |_| {
        map_state.update(|state| {
            let line = SelectedLine::new_line(state.get_mut_map());
            state.set_selected_line(line);
        });
    };

    let remove_station = move |_| {
        map_state.update(|state| {
            state.set_selected_remove(RemoveType::Station);
        });
    };
    let remove_station_selected = move || {
        map_state
            .get()
            .get_selected_remove()
            == Some(RemoveType::Station)
    };

    let remove_line = move |_| {
        map_state.update(|state| {
            state.set_selected_remove(RemoveType::Line);
        });
    };
    let remove_line_selected = move || {
        map_state
            .get()
            .get_selected_remove()
            == Some(RemoveType::Line)
    };

    view! {
        <div id="sidebar" class="h-full w-full flex flex-col gap-y-4 bg-zinc-100 py-2 shadow-right shadow-dark-mild dark:shadow-black dark:bg-neutral-750 text-black dark:text-white px-2">
            <Button
                on_click=Box::new(move |_| map_state.update(|state| state.set_map(testmap())))
                text="reset map" />
            <ButtonGroup
                children={vec![
                    ButtonProps::builder()
                        .text("Add Station")
                        .on_click(Box::new(add_station))
                        .build(),
                    ButtonProps::builder()
                        .text("Remove Station")
                        .on_click(Box::new(remove_station))
                        .active(Signal::derive(remove_station_selected))
                        .danger(true)
                        .build(),
                ]}/>
            <ButtonGroup
                children={vec![
                    ButtonProps::builder()
                        .text("Add Line")
                        .on_click(Box::new(add_line))
                        .build(),
                    ButtonProps::builder()
                        .text("Remove Line")
                        .on_click(Box::new(remove_line))
                        .active(Signal::derive(remove_line_selected))
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
