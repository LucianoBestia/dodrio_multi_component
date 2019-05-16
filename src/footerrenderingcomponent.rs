use crate::appdata::AppData;
use crate::rootrenderingcomponent::RootRenderingComponent;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};

pub struct FooterRenderingComponent {
    ///cached local value is copied from app_data
    author: String,
    counter3: i32,
}
impl FooterRenderingComponent {
    pub fn new(app_data: &AppData) -> Self {
        //default values
        let mut footer = Self {
            author: "".to_string(),
            counter3: 0,
        };
        footer.update_cache_from_app_data(app_data);
        footer
    }
    //updates only app_data
    pub fn update_counter1(&self, app_data: &mut AppData) {
        app_data.counter1 += 100;
    }
    //the only place where the internal cached values are updated
    //only internal cached values are rendered in this component
    pub fn update_cache_from_app_data(&mut self, app_data: &AppData) -> bool {
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
