use crate::appdata::AppData;
use crate::rootrenderingcomponent::RootRenderingComponent;

extern crate console_error_panic_hook;
extern crate log;
extern crate web_sys;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};

pub struct ContentRenderingComponent {
    ///cached local values are copied from app_data
    description: String,
    counter2: i32,
}
impl ContentRenderingComponent {
    pub fn new(app_data: &AppData) -> Self {
        //default values
        let mut content = Self {
            description: "".to_string(),
            counter2: 0,
        };
        content.update_cache_from_app_data(app_data);
        content
    }
    //updates only app_data
    pub fn update_counter3(&self, app_data: &mut AppData) {
        app_data.counter3 += 10;
    }
    //the only place where the internal cached values are updated
    //only internal cached values are rendered in this component
    pub fn update_cache_from_app_data(&mut self, app_data: &AppData) -> bool {
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
