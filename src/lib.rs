//! This project provides an interactive editor for metro maps.
//! This file defines the crate and provides access to the algorithms, models,
//! and components.

// Deny all default lints and warn on pedantic ones by default
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
// Additionally warn when we forget to add documentation on things.
#![warn(clippy::missing_docs_in_private_items)]
// Lots of casts have to be done back and forth between js and rust, even if f64 to i32 might in
// theory truncate, same with usize to f64.
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]
// Wildcard imports are idiomatic with leptos.
#![allow(clippy::wildcard_imports)]
// Having file and component function names be the same is idiomatic for leptos component files.
#![allow(clippy::module_name_repetitions)]
// Having a lot of parameters in a function is idiomatic for leptos.
#![allow(clippy::fn_params_excessive_bools)]
// There is not need for must_use or panics docs on functions as this is lib is only for internal
// use.
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
// FIXME: Remove this once the issue with local search has been fixed and it's back in use
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// Import the necessary modules, these are more public than would be expected,
// but this is needed to access them for testing.
pub mod algorithm;
pub mod models;
pub mod utils;

mod components;
pub use components::{
    CanvasState,
    Home,
    MapState,
    StateProvider,
};
pub use utils::Error;
