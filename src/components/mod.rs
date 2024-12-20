//! Contains all leptos (html) components used for building the webpage.

// Components can by necessity be quite large.
#![allow(clippy::too_many_lines)]

mod atoms;
mod canvas;
mod molecules;
mod organisms;
mod pages;
mod state;

pub use pages::Home;
pub use state::{
    CanvasState,
    ErrorState,
    HistoryState,
    MapState,
    StateProvider,
};
