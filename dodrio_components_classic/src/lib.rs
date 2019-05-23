//! Learning Rust Wasm/WebAssembly with Virtual Dom Dodrio.
//! read README.md
//! use Fold and Unfold regions to better read source code.
//! this is the classic approach
//! cannot use `dodrio::cache`. It requires the trait Render for sub Components.
//! this trait method Render doesn't allow to send `app_data` as parameter.
//! `app_data` must not be inside of the subComponent because then we have a self-referenced struct.
//! It is possible to use some other type of cache? But then we have the lifetime problem.

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated
)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    clippy::implicit_return,
    //clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export `run` not found 
    clippy::missing_inline_in_public_items,
    clippy::missing_docs_in_private_items,
    clippy::useless_let_if_seq
)]
//endregion

//region: mod, extern and use statements
pub mod appdata;
pub mod contentrenderingcomponent;
pub mod footerrenderingcomponent;
pub mod headerrenderingcomponent;
pub mod rootrenderingcomponent;

extern crate console_error_panic_hook;
extern crate log;
extern crate web_sys;

use wasm_bindgen::prelude::*;
//endregion

//region: wasm_bindgen(start) is where everything starts
#[wasm_bindgen(start)]
///wasm_bindgen runs this functions at start
pub fn run() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's container to render the virtual dom component.
    let window = web_sys::window().expect("error: web_sys::window");
    let document = window.document().expect("error: window.document");
    let div_for_virtual_dom = document
        .get_element_by_id("div_for_virtual_dom")
        .expect("No #div_for_virtual_dom");

    let root_rendering_component = rootrenderingcomponent::RootRenderingComponent::new();

    // Mount the component to the `<div id="div_for_virtual_dom">`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, root_rendering_component);

    // Run the component forever. Forget to drop the memory.
    vdom.forget();

    Ok(())
}
//endregion
