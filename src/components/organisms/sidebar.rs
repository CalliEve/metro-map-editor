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
        MapState,
    },
    models::{
        Line,
        Map,
        Station,
    },
};

/// The sidebar component with all the tools on there for editing the canvas.
#[component]
pub fn Sidebar() -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    view! {
        <div id="sidebar" class="h-full w-full flex flex-col gap-y-4 bg-zinc-100 py-2 shadow-right shadow-dark-mild dark:shadow-black dark:bg-neutral-750 text-black dark:text-white px-2">
            <Button
                on_click=Box::new(move |_| map_state.update(|state| state.set_map(testmap())))
                text="reset map" />
            <NumberInput
                text="Set grid size"
                min=2.0
                max=f64::from(u32::MAX)
                value=move || f64::from(map_state.get().get_square_size())
                on_input=move |n| map_state.update(|state| state.set_square_size(n.abs() as u32))/>
            <ButtonGroup
                children={vec![
                    ButtonProps::builder().text("Add Station").on_click(Box::new(|_| {})).build(),
                    ButtonProps::builder().text("Remove Station").on_click(Box::new(|_| {})).danger(true).build(),
                ]}/>
            <ButtonGroup
                children={vec![
                    ButtonProps::builder().text("Add Line").on_click(Box::new(|_| {})).build(),
                    ButtonProps::builder().text("Remove Line").on_click(Box::new(|_| {})).danger(true).build(),
                ]}/>
        </div>
    }
}

/// Temporary function to load in a test metro map.
fn testmap() -> Map {
    let mut map = Map::new();

    map.add_line(Line::new(
        vec![
            Station::new((10, 10), None),
            Station::new((15, 15), None),
            Station::new((20, 25), None),
        ],
        &"line 1",
    ));

    map.add_line(Line::new(
        vec![
            Station::new((20, 12), None),
            Station::new((25, 12), None),
            Station::new((30, 20), None),
        ],
        &"line 2",
    ));

    map.add_line(Line::new(
        vec![Station::new((7, 5), None)],
        &"line 3",
    ));

    map
}
