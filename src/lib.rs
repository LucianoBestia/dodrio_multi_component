//! Learning Rust Wasm/WebAssembly with Virtual Dom Dodrio.
//! read README.md
//! use Fold and Unfold regions to better read source code.
//! this is the approach suggested by fitzgen 2019-05-16
//! 'Make `AppData` be the root rendering component,'
//! For less confusion with names I will put the fields of `AppData` inside `RootRenderingComponent`.
//! My goal is to have reuseable components for other.
//! On another webpage, the content can change, but the Header and Footer component are the same.  

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

//region: extern and use statements
extern crate console_error_panic_hook;
extern crate log;
extern crate web_sys;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Cached, Node, Render};
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
    ///cached local values are copied from app_data
    title: String,
    counter1: i32,
}
struct ContentRenderingComponent {
    ///cached local values are copied from app_data
    description: String,
    counter2: i32,
}
struct FooterRenderingComponent {
    ///cached local value is copied from app_data
    author: String,
    counter3: i32,
}
struct RootRenderingComponent {
    app_data: AppData,

    header_rendering_component: Cached<HeaderRenderingComponent>,
    content_rendering_component: Cached<ContentRenderingComponent>,
    footer_rendering_component: Cached<FooterRenderingComponent>,
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
        Self {
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
        let header_rendering_component = Cached::new(HeaderRenderingComponent::new(&app_data));
        let content_rendering_component = Cached::new(ContentRenderingComponent::new(&app_data));
        let footer_rendering_component = Cached::new(FooterRenderingComponent::new(&app_data));

        Self {
            app_data,
            header_rendering_component,
            content_rendering_component,
            footer_rendering_component,
        }
    }
    //RootRenderingComponent must know the relations between Components.
    //The sub Components don't know anything about their relationships.
    fn update_from_header(&mut self) {
        //some change made from RootRenderingComponent
        self.app_data.description.push('x');

        //some changed made by the sub RenderingComponent method
        self.header_rendering_component
            .update_counter2(&mut self.app_data);

        self.invalidate_components();
    }

    fn update_from_content(&mut self) {
        //some change made from RootRenderingComponent
        self.app_data.author.push('y');

        //some changed made by the sub RenderingComponent method
        self.content_rendering_component
            .update_counter3(&mut self.app_data);

        self.invalidate_components();
    }

    fn update_from_footer(&mut self) {
        self.app_data.title.push('z');
        //other data is changed by the sub RenderingComponent
        self.footer_rendering_component
            .update_counter1(&mut self.app_data);

        self.invalidate_components();
    }

    fn invalidate_components(&mut self) {
        //app_data can change any time anywhere.
        //Components must update their cached values and return true if they changed.
        if self
            .header_rendering_component
            .update_cache_from_app_data(&self.app_data)
        {
            Cached::invalidate(&mut self.header_rendering_component);
        }
        if self
            .content_rendering_component
            .update_cache_from_app_data(&self.app_data)
        {
            Cached::invalidate(&mut self.content_rendering_component);
        }
        if self
            .footer_rendering_component
            .update_cache_from_app_data(&self.app_data)
        {
            Cached::invalidate(&mut self.footer_rendering_component);
        }
    }
}
impl HeaderRenderingComponent {
    fn new(app_data: &AppData) -> Self {
        //default values
        let mut header = Self {
            title: "".to_string(),
            counter1: 0,
        };
        header.update_cache_from_app_data(app_data);
        header
    }
    //updates only app_data
    fn update_counter2(&self, app_data: &mut AppData) {
        app_data.counter2 += 100;
    }
    //the only place where the internal cached values are updated
    //only internal cached values are rendered in this component
    fn update_cache_from_app_data(&mut self, app_data: &AppData) -> bool {
        let mut retvalue = false;
        if self.title != app_data.title {
            self.title = app_data.title.clone();
            retvalue = true;
        }
        if self.counter1 != app_data.counter1 {
            self.counter1 = app_data.counter1;
            retvalue = true;
        }
        retvalue
    }
}
impl ContentRenderingComponent {
    fn new(app_data: &AppData) -> Self {
        //default values
        let mut content = Self {
            description: "".to_string(),
            counter2: 0,
        };
        content.update_cache_from_app_data(app_data);
        content
    }
    //updates only app_data
    fn update_counter3(&self, app_data: &mut AppData) {
        app_data.counter3 += 10;
    }
    //the only place where the internal cached values are updated
    //only internal cached values are rendered in this component
    fn update_cache_from_app_data(&mut self, app_data: &AppData) -> bool {
        let mut retvalue = false;

        if self.description != app_data.description {
            self.description = app_data.description.clone();
            retvalue = true;
        }
        if self.counter2 != app_data.counter2 {
            self.counter2 = app_data.counter2;
            retvalue = true;
        }
        retvalue
    }
}
impl FooterRenderingComponent {
    fn new(app_data: &AppData) -> Self {
        //default values
        let mut footer = Self {
            author: "".to_string(),
            counter3: 0,
        };
        footer.update_cache_from_app_data(app_data);
        footer
    }
    //updates only app_data
    fn update_counter1(&self, app_data: &mut AppData) {
        app_data.counter1 += 100;
    }
    //the only place where the internal cached values are updated
    //only internal cached values are rendered in this component
    fn update_cache_from_app_data(&mut self, app_data: &AppData) -> bool {
        let mut retvalue = false;
        if self.author != app_data.author {
            self.author = app_data.author.clone();
            retvalue = true;
        }
        if self.counter3 != app_data.counter3 {
            self.counter3 = app_data.counter3;
            retvalue = true;
        }
        retvalue
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
        div(bump)
            .children([h1(bump)
                .children([text(
                    bumpalo::format!(in bump, "click on me: {} {}", self.title,self.counter1)
                        .into_bump_str(),
                )])
                .on("click", move |root, vdom, _event| {
                    //we should access only the RootRenderingComponent here. It is the only componente
                    //that knows all the other components. A subComponent is always a part of a RootRenderingComponent.
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
        div(bump)
            .children([h1(bump)
                .children([text(
                    bumpalo::format!(in bump, "click on me: {} {}", self.description,self.counter2)
                        .into_bump_str(),
                )])
                .on("click", move |root, vdom, _event| {
                    //we should access only the RootRenderingComponent here. It is the only componente
                    //that knows all the other components. A subComponent is always a part of a RootRenderingComponent.
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
        div(bump)
            .children([h1(bump)
                .children([text(
                    bumpalo::format!(in bump, "click on me: {} {}", self.author, self.counter3)
                        .into_bump_str(),
                )])
                .on("click", move |root, vdom, _event| {
                    //we should access only the RootRenderingComponent here. It is the only componente
                    //that knows all the other components. A subComponent is always a part of a RootRenderingComponent.
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
