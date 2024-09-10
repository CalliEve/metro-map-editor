//! This project provides an interactive editor for metro maps.
//! This file defines the crate and provides access to the algorithms, models,
//! and components.

// Deny all default lints and warn on pedantic ones by default
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
// lots of casts have to be done back and forth between js and rust, even if f64 to i32 might in
// theory truncate, same with usize to f64.
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]
// wildcard imports are idiomatic with leptos.
#![allow(clippy::wildcard_imports)]
// having file and component function names be the same is idiomatic for leptos component files.
#![allow(clippy::module_name_repetitions)]
// having a lot of parameters in a function is idiomatic for leptos.
#![allow(clippy::fn_params_excessive_bools)]

pub mod algorithm;
mod components;
pub mod models;
pub mod utils;

pub use components::{
    Home,
    StateProvider,
};
