//the Content Component will change often and I don't need the render-cache.
//so I don't need the Render trait that dodrio::cache must have.
//I can simply add app_data as a parameter to the `render_with_app_data` function.
use crate::appdata::AppData;
//in events (on click) we have to use the RootRenderingComponent,
//because it is the only one to be unwraped from vdom.
//TODO: but we want be compatible with different RootRenderingComponent.
use crate::rootrenderingcomponent::RootRenderingComponent;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;

#[derive(Default)]
pub struct ContentRenderingComponent {}
impl ContentRenderingComponent {
    pub const fn new() -> Self {
        Self {}
    }
    //updates only app_data
    pub fn update_counter3(&self, app_data: &mut AppData) {
        app_data.counter3 += 10;
    }
    pub fn render_with_app_data<'a, 'bump>(
        &'a self,
        bump: &'bump Bump,
        app_data: &AppData,
    ) -> Node<'bump>
    where
        'a: 'bump,
    {
        //only internal cached values are rendered in this component
        div(bump)
            .children([h1(bump)
                .children([text(
                    bumpalo::format!(in bump, "click on me: {} {}", app_data.description,app_data.counter2)
                        .into_bump_str(),
                )])
                .on("click", move |root, vdom, _event| {
                    //we should access only the RootRenderingComponent here.
                    //It is the only componente that knows about app_data and other components and their relationship.
                    //A subComponent is always a part of a RootRenderingComponent.
                    let root_rendering_component = root.unwrap_mut::<RootRenderingComponent>();
                    root_rendering_component.update_from_content();
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
                })
                .finish()])
            .finish()
    }
}
