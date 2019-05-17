//the Component must know about app_data because it receives it as a method parameter
//but also app_data can change with another RootRenderingComonent.
//TODO: how to achieve reusability ? We can send only the required fields in the fn call.
use crate::appdata::AppData;
//in events (on click) we have to use the RootRenderingComponent,
//because it is the only one to be unwraped from vdom.
//TODO: but we want be compatible with different RootRenderingComponent.
use crate::rootrenderingcomponent::RootRenderingComponent;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};

// cached local values are copied from app_data
// these values are rendered in the Render method,
// because only they are accessible from this structs methods.
// they are used to check if the app_data has changes.
// That invalidates the render cache.
pub struct HeaderRenderingComponent {
    title: String,
    counter1: i32,
}
impl HeaderRenderingComponent {
    pub fn new(app_data: &AppData) -> Self {
        //default values
        let mut header = Self {
            title: "".to_string(),
            counter1: 0,
        };
        header.update_cache_from_app_data(app_data);
        header
    }
    //updates only app_data
    pub fn update_counter2(&self, app_data: &mut AppData) {
        app_data.counter2 += 100;
    }
    //the only place where the internal cached values are updated
    pub fn update_cache_from_app_data(&mut self, app_data: &AppData) -> bool {
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
impl Render for HeaderRenderingComponent {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        //only internal cached values are rendered in this component
        div(bump)
            .children([h1(bump)
                .children([text(
                    bumpalo::format!(in bump, "click on me: {} {}", self.title,self.counter1)
                        .into_bump_str(),
                )])
                .on("click", move |root, vdom, _event| {
                    //we should access only the RootRenderingComponent here.
                    //It is the only componente that knows about app_data and other components and their relationship.
                    //A subComponent is always a part of a RootRenderingComponent.
                    let root_rendering_component = root.unwrap_mut::<RootRenderingComponent>();
                    root_rendering_component.update_from_header();
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
                })
                .finish()])
            .finish()
    }
}
