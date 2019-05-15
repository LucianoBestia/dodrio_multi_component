//! Learning Rust Wasm/WebAssembly with Virtual Dom Dodrio.
//! read README.md
//! use Fold and Unfold regions to better read source code.

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
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export `run` not found 
    clippy::missing_inline_in_public_items,
    clippy::missing_docs_in_private_items,
)]
//endregion

//region: extern and use statements
extern crate console_error_panic_hook;
extern crate log;
extern crate web_sys;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Cached, Node, Render};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
//endregion

//region: enum, structs, const,...
struct AppData {
    title: String,
    description: String,
    author: String,
    counter1: i32,
    counter2: i32,
    counter3: i32,
}
struct HeaderRenderingComponent {
    ///shared mutable data
    rc_app_data: Rc<RefCell<AppData>>,
}
struct ContentRenderingComponent {
    ///shared mutable data
    rc_app_data: Rc<RefCell<AppData>>,
}
struct FooterRenderingComponent {
    ///shared mutable data
    rc_app_data: Rc<RefCell<AppData>>,
}
struct RootRenderingComponent {
    header_rendering_component: Cached<HeaderRenderingComponent>,
    content_rendering_component: Cached<ContentRenderingComponent>,
    footer_rendering_component: Cached<FooterRenderingComponent>,
    ///shared mutable data
    rc_app_data: Rc<RefCell<AppData>>,
}
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

    let root_rendering_component = RootRenderingComponent::new();

    // Mount the component to the `<div id="div_for_virtual_dom">`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, root_rendering_component);

    // Run the component forever. Forget to drop the memory.
    vdom.forget();

    Ok(())
}
//endregion

//region: impl
impl AppData {
    ///constructor
    pub fn new() -> Self {
        //return from constructor
        AppData {
            title: String::from("title"),
            description: String::from("description"),
            author: String::from("author"),
            counter1: 0,
            counter2: 0,
            counter3: 0,
        }
    }
}
impl RootRenderingComponent {
    pub fn new() -> Self {
        let app_data = AppData::new();
        let rc_app_data = Rc::new(RefCell::new(app_data));

        RootRenderingComponent {
            header_rendering_component: Cached::new(HeaderRenderingComponent {
                rc_app_data: Rc::<std::cell::RefCell<AppData>>::clone(&rc_app_data),
            }),
            content_rendering_component: Cached::new(ContentRenderingComponent {
                rc_app_data: Rc::<std::cell::RefCell<AppData>>::clone(&rc_app_data),
            }),
            footer_rendering_component: Cached::new(FooterRenderingComponent {
                rc_app_data: Rc::<std::cell::RefCell<AppData>>::clone(&rc_app_data),
            }),
            rc_app_data,
        }
    }

    fn update_from_header(&mut self) {
        {
            //some data is changed by the RootRenderingComponent
            let mut app_data = self.rc_app_data.borrow_mut();
            app_data.description.push('x');
        }
        //other data is changed by the sub RenderingComponent
        self.header_rendering_component.update_counter2();

        //invalidated another component:
        Cached::invalidate(&mut self.content_rendering_component);
    }

    fn update_from_content(&mut self) {
        {
            //some data is changed by the RootRenderingComponent
            let mut app_data = self.rc_app_data.borrow_mut();
            app_data.author.push('y');
        }
        //other data is changed by the sub RenderingComponent
        self.content_rendering_component.update_counter3();

        //invalidated another component for rerendering
        Cached::invalidate(&mut self.footer_rendering_component);
    }

    fn update_from_footer(&mut self) {
        {
            //some data is changed by the RootRenderingComponent
            let mut app_data = self.rc_app_data.borrow_mut();
            app_data.title.push('z');
        }
        //other data is changed by the sub RenderingComponent
        self.footer_rendering_component.update_counter1();
        //invalidated another component for rerendering
        Cached::invalidate(&mut self.header_rendering_component);
    }
}
impl HeaderRenderingComponent {
    fn update_counter2(&mut self) {
        let mut app_data = self.rc_app_data.borrow_mut();
        app_data.counter2 += 100;
    }
}
impl ContentRenderingComponent {
    fn update_counter3(&mut self) {
        let mut app_data = self.rc_app_data.borrow_mut();
        app_data.counter3 += 10;
    }
}
impl FooterRenderingComponent {
    fn update_counter1(&mut self) {
        let mut app_data = self.rc_app_data.borrow_mut();
        app_data.counter1 += 1;
    }
}
//endregion

//region: Render
impl Render for RootRenderingComponent {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        div(bump)
            .children([
                self.header_rendering_component.render(bump),
                self.content_rendering_component.render(bump),
                self.footer_rendering_component.render(bump),
            ])
            .finish()
    }
}
impl Render for HeaderRenderingComponent {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        let app_data = self.rc_app_data.borrow();
        div(bump)
            .children([h1(bump)
                .children([text(
                    bumpalo::format!(in bump, "click on me: {} {}", app_data.title,app_data.counter1)
                        .into_bump_str(),
                )])
                .on("click", move |root, vdom, _event| {
                    let root_rendering_component = root.unwrap_mut::<RootRenderingComponent>();
                    root_rendering_component.update_from_header();
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
                })
                .finish()])
            .finish()
    }
}
impl Render for ContentRenderingComponent {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        let app_data = self.rc_app_data.borrow();
        div(bump)
        .children([
            h1(bump)
            .children([text(
                    bumpalo::format!(in bump, "click on me: {} {}", app_data.description,app_data.counter2)
                        .into_bump_str(),
                )])
            .on("click", move |root, vdom, _event| {
                    let root_rendering_component = root.unwrap_mut::<RootRenderingComponent>();
                    root_rendering_component.update_from_content();
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
            })
            .finish()])
        .finish()
    }
}
impl Render for FooterRenderingComponent {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        let app_data = self.rc_app_data.borrow();
        div(bump)
        .children([
            h1(bump)
            .children([
                text(
                    bumpalo::format!(in bump, "click on me: {} {}", app_data.author, app_data.counter3).into_bump_str(),
                )])
            .on("click", move |root, vdom, _event| {
                        let root_rendering_component = root.unwrap_mut::<RootRenderingComponent>();
                        root_rendering_component.update_from_footer();
                        // Finally, re-render the component on the next animation frame.
                        vdom.schedule_render();
            })
            .finish()])
        .finish()
    }
}
//endregion
