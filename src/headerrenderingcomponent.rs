use crate::appdata::AppData;
use crate::rootrenderingcomponent::RootRenderingComponent;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};

pub struct HeaderRenderingComponent {
    ///cached local values are copied from app_data
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
    //only internal cached values are rendered in this component
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
