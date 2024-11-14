//! Contains the [`CanvasContext`] struct and all its methods used for drawing
//! to the html canvas.

#[cfg(all(not(test), not(feature = "benchmarking")))]
use std::{
    borrow::Cow,
    ops::Deref,
};
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
use web_sys::HtmlCanvasElement;

/// A wrapper around the [`web_sys::CanvasRenderingContext2d`]. This struct
/// provides the ability to mock and unit-test the drawing functions.
pub struct CanvasContext<'a> {
    /// The inner [`web_sys::CanvasRenderingContext2d`] object wrapped by this
    /// struct.
    #[cfg(all(not(test), not(feature = "benchmarking")))]
    inner: Cow<'a, web_sys::CanvasRenderingContext2d>,
    #[cfg(any(test, feature = "benchmarking"))]
    inner: PhantomData<&'a ()>,
    #[cfg(any(test, feature = "benchmarking"))]
    recorder: RefCell<HashMap<String, Vec<String>>>,
}

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl From<web_sys::CanvasRenderingContext2d> for CanvasContext<'static> {
    fn from(context: web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            inner: Cow::Owned(context),
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
            inner: Cow::Owned(context),
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

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl<'a> From<&'a web_sys::CanvasRenderingContext2d> for CanvasContext<'a> {
    fn from(context: &'a web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            inner: Cow::Borrowed(context),
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

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl AsRef<web_sys::CanvasRenderingContext2d> for CanvasContext<'_> {
    fn as_ref(&self) -> &web_sys::CanvasRenderingContext2d {
        &self.inner
    }
}

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl Deref for CanvasContext<'_> {
    type Target = web_sys::CanvasRenderingContext2d;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(all(not(test), not(feature = "benchmarking")))]
impl CanvasContext<'_> {
    pub fn set_line_dash(&self, segments: &[u8]) -> Result<(), JsValue> {
        self.inner
            .set_line_dash(&Uint8Array::from(segments))
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
