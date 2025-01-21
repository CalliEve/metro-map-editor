//! Contains the [`CanvasContext`] struct and all its methods used for drawing
//! to the html canvas.

#[cfg(all(not(test), not(feature = "benchmarking")))]
use std::borrow::Cow;
#[cfg(any(test, feature = "benchmarking"))]
use std::{
    cell::RefCell,
    collections::HashMap,
    marker::PhantomData,
};

#[cfg(any(test, feature = "benchmarking"))]
use wasm_bindgen::JsValue;
#[cfg(all(not(test), not(feature = "benchmarking")))]
use wasm_bindgen::{
    JsCast,
    JsValue,
};
#[cfg(all(not(test), not(feature = "benchmarking")))]
use web_sys::js_sys::Uint8Array;
use web_sys::{
    HtmlCanvasElement,
    OffscreenCanvas,
};

/// Either a [`web_sys::CanvasRenderingContext2d`] or
/// [`web_sys::OffscreenCanvasRenderingContext2d`].
#[cfg(all(not(test), not(feature = "benchmarking")))]
enum InnerCanvasContext<'a> {
    /// An offscreen canvas context.
    OffScreen(Cow<'a, web_sys::OffscreenCanvasRenderingContext2d>),
    /// An onscreen canvas context.
    OnScreen(Cow<'a, web_sys::CanvasRenderingContext2d>),
}

/// A wrapper around the [`web_sys::CanvasRenderingContext2d`] or
/// [`web_sys::OffscreenCanvasRenderingContext2d`]. This struct provides the
/// ability to mock and unit-test the drawing functions.
pub struct CanvasContext<'a> {
    /// The inner [`web_sys::CanvasRenderingContext2d`] object wrapped by this
    /// struct.
    #[cfg(all(not(test), not(feature = "benchmarking")))]
    inner: InnerCanvasContext<'a>,
    #[cfg(any(test, feature = "benchmarking"))]
    inner: PhantomData<&'a ()>,
    #[cfg(any(test, feature = "benchmarking"))]
    recorder: RefCell<HashMap<String, Vec<String>>>,
}

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl From<web_sys::CanvasRenderingContext2d> for CanvasContext<'static> {
    fn from(context: web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            inner: InnerCanvasContext::OnScreen(Cow::Owned(context)),
        }
    }
}

#[cfg(any(test, feature = "benchmarking"))]
impl From<web_sys::CanvasRenderingContext2d> for CanvasContext<'static> {
    fn from(_: web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            inner: PhantomData,
            recorder: RefCell::new(HashMap::new()),
        }
    }
}

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl<'a> From<&'a HtmlCanvasElement> for CanvasContext<'a> {
    fn from(canvas: &'a HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .expect("Failed to get 2d context")
            .expect("2d context is null")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Failed to convert to CanvasRenderingContext2d");

        Self {
            inner: InnerCanvasContext::OnScreen(Cow::Owned(context)),
        }
    }
}

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl<'a> From<&'a OffscreenCanvas> for CanvasContext<'a> {
    fn from(canvas: &'a OffscreenCanvas) -> Self {
        let context = canvas
            .get_context("2d")
            .expect("Failed to get 2d context")
            .expect("offscreen 2d context is null")
            .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
            .expect("Failed to convert to OffscreenCanvasRenderingContext2d");

        Self {
            inner: InnerCanvasContext::OffScreen(Cow::Owned(context)),
        }
    }
}

#[cfg(any(test, feature = "benchmarking"))]
impl<'a> From<&'a HtmlCanvasElement> for CanvasContext<'a> {
    fn from(_: &'a HtmlCanvasElement) -> Self {
        Self {
            inner: PhantomData,
            recorder: RefCell::new(HashMap::new()),
        }
    }
}

#[cfg(any(test, feature = "benchmarking"))]
impl<'a> From<&'a OffscreenCanvas> for CanvasContext<'a> {
    fn from(_: &'a OffscreenCanvas) -> Self {
        Self {
            inner: PhantomData,
            recorder: RefCell::new(HashMap::new()),
        }
    }
}

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl<'a> From<&'a web_sys::CanvasRenderingContext2d> for CanvasContext<'a> {
    fn from(context: &'a web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            inner: InnerCanvasContext::OnScreen(Cow::Borrowed(context)),
        }
    }
}

#[cfg(any(test, feature = "benchmarking"))]
impl<'a> From<&'a web_sys::CanvasRenderingContext2d> for CanvasContext<'a> {
    fn from(_: &'a web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            inner: PhantomData,
            recorder: RefCell::new(HashMap::new()),
        }
    }
}

/// Creates a method for both the onscreen and offscreen canvas context.
#[cfg(all(not(test), not(feature = "benchmarking")))]
macro_rules! impl_canvas_context_method {
    ($method:ident($($arg:ident: $arg_ty:ty),*) -> $res:ty) => {
        pub fn $method(&self, $($arg: $arg_ty),*) -> $res {
            match &self.inner {
                InnerCanvasContext::OffScreen(context) => {
                    context.$method($($arg),*)
                },
                InnerCanvasContext::OnScreen(context) => {
                    context.$method($($arg),*)
                },
            }
        }

    }
}

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl CanvasContext<'_> {
    impl_canvas_context_method!(move_to(x: f64, y: f64) -> ());

    impl_canvas_context_method!(line_to(x: f64, y: f64) -> ());

    impl_canvas_context_method!(stroke() -> ());

    impl_canvas_context_method!(begin_path() -> ());

    impl_canvas_context_method!(set_line_width(f: f64) -> ());

    impl_canvas_context_method!(set_stroke_style_str(s: &str) -> ());

    impl_canvas_context_method!(set_global_alpha(a: f64) -> ());

    impl_canvas_context_method!(rect(x: f64, y: f64, width: f64, height: f64) -> ());

    impl_canvas_context_method!(arc(x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Result<(), JsValue>);

    impl_canvas_context_method!(fill() -> ());

    impl_canvas_context_method!(set_fill_style_str(style: &str) -> ());

    pub fn set_line_dash(&self, segments: &[u8]) -> Result<(), JsValue> {
        let array = Uint8Array::from(segments);
        match &self.inner {
            InnerCanvasContext::OffScreen(context) => context.set_line_dash(&array),
            InnerCanvasContext::OnScreen(context) => context.set_line_dash(&array),
        }
    }

    pub fn is_onscreen(&self) -> bool {
        match &self.inner {
            InnerCanvasContext::OffScreen(_) => false,
            InnerCanvasContext::OnScreen(_) => true,
        }
    }
}

#[cfg(any(test, feature = "benchmarking"))]
impl<'a> CanvasContext<'a> {
    pub fn new() -> Self {
        Self {
            inner: PhantomData,
            recorder: RefCell::new(HashMap::new()),
        }
    }

    pub fn is_onscreen(&self) -> bool {
        true
    }

    pub fn move_to(&self, x: f64, y: f64) {
        self.record(
            "move_to",
            format!("{x:.1},{y:.1}").as_str(),
        );
    }

    pub fn line_to(&self, x: f64, y: f64) {
        self.record(
            "line_to",
            format!("{x:.1},{y:.1}").as_str(),
        );
    }

    pub fn stroke(&self) {
        self.record("stroke", "");
    }

    pub fn begin_path(&self) {
        self.record("begin_path", "");
    }

    pub fn set_line_width(&self, f: f64) {
        self.record(
            "set_line_width",
            format!("{f}").as_str(),
        );
    }

    pub fn set_stroke_style_str(&self, _: &str) {}

    pub fn set_global_alpha(&self, _: f64) {}

    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.record(
            "rect",
            format!("{x:.1},{y:.1},{width:.1},{height:.1}").as_str(),
        );
    }

    pub fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue> {
        self.record(
            "arc",
            format!("{x:.1},{y:.1},{radius},{start_angle},{end_angle}").as_str(),
        );
        Ok(())
    }

    pub fn set_line_dash(&self, _: &[u8]) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn fill(&self) {
        self.record("fill", "");
    }

    pub fn set_fill_style_str(&self, style: &str) {
        self.record("set_fill_style", style);
    }

    fn record(&self, name: &str, value: &str) {
        self.recorder
            .borrow_mut()
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(value.to_string());
    }

    pub fn get_record(&self, name: &str) -> Option<Vec<String>> {
        self.recorder
            .borrow()
            .get(name)
            .cloned()
    }
}
