//! # Example
//!
//! ```
//! extern crate kagura;
//! extern crate wasm_bindgen;
//!
//! use kagura::prelude::*;
//! use wasm_bindgen::prelude::*;
//!
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     kagura::run(Component::new(State, update, render), "app");
//! }
//!
//! struct State;
//!
//! struct Msg;
//!
//! struct Sub;
//!
//! fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {Cmd::none()}
//!
//! fn render(_: &State) -> Html<Msg> {
//!     Html::h1(
//!         Attributes::new(),
//!         Events::new(),
//!         vec![
//!             Html::text("hello kagura"),
//!         ],
//!     )
//! }
//! ```

extern crate js_sys;
extern crate rand;
extern crate serde_derive;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

mod dom;
mod event;
mod native;
mod state;
mod task;

pub use dom::component::Cmd;
pub use dom::component::Component;
pub use dom::html::Attributes;
pub use dom::html::Events;
pub use dom::html::Html;

/// Starts application with component
pub fn run<Msg, State, Sub>(component: Component<Msg, State, Sub>, id: &str)
where
    Msg: 'static,
    State: 'static,
    Sub: 'static,
{
    state::init(component, id);
}

pub mod prelude {
    pub use crate::Attributes;
    pub use crate::Cmd;
    pub use crate::Component;
    pub use crate::Events;
    pub use crate::Html;
}
