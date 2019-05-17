//the Component knows about app_data only as a parameter
use crate::appdata::AppData;
//in events (on click) we have to use the RootRenderingComponent,
//because it is the only one to be unwraped from vdom.
//TODO: but we want be compatible with different RootRenderingComponent.
use crate::rootrenderingcomponent::RootRenderingComponent;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;

#[derive(Default)]
pub struct HeaderRenderingComponent {}
impl HeaderRenderingComponent {
    pub const fn new() -> Self {
        Self {}
    }
    //updates only app_data
    pub fn update_counter2(&self, app_data: &mut AppData) {
        app_data.counter2 += 100;
    }
}
//cannot implement trait Render, because then it has not access to app_data
//I will make it a simple function,
//but so I loose the possiblity of dodrio::cache
pub fn render<'a, 'bump>(app_data: &'a AppData, bump: &'bump Bump) -> Node<'bump>
where
    'a: 'bump,
{
    //only internal cached values are rendered in this component
    div(bump)
        .children([h1(bump)
            .children([text(
                bumpalo::format!(in bump, "click on me: {} {}", app_data.title,app_data.counter1)
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
