//! Contains large, complex components that make up major elements of the page
//! and consist of multiple child components.

mod canvas_controls;
mod navbar;
mod sidebar;

pub use canvas_controls::CanvasControls;
pub use navbar::Navbar;
pub use sidebar::Sidebar;
