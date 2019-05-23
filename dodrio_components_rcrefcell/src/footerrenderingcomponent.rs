//the Component has a shared mutable reference to AppData
use crate::appdata::AppData;
//in events (on click) we have to use the RootRenderingComponent,
//because it is the only one to be unwraped from vdom.
//TODO: but we want be compatible with different RootRenderingComponent.
use crate::rootrenderingcomponent::RootRenderingComponent;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use std::cell::RefCell;
use std::rc::Rc;

pub struct FooterRenderingComponent {
    ///shared mutable data
    pub rc_app_data: Rc<RefCell<AppData>>,
}
impl FooterRenderingComponent {
    pub const fn new(rc_app_data: Rc<RefCell<AppData>>) -> Self {
        //default values
        Self { rc_app_data }
    }
    //updates only app_data
    pub fn update_counter1(&self) {
        let mut app_data = self.rc_app_data.borrow_mut();
        app_data.counter1 += 100;
    }
}
impl Render for FooterRenderingComponent {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        let app_data = self.rc_app_data.borrow();
        div(bump)
            .children([h1(bump)
                .children([text(
                    bumpalo::format!(in bump, "click on me: {} {} {}", app_data.author,app_data.title, app_data.counter3)
                        .into_bump_str(),
                )])
                .on("click", move |root, vdom, _event| {
                    //we should access only the RootRenderingComponent here.
                    //It is the only componente that knows about app_data and other components and their relationship.
                    //A subComponent is always a part of a RootRenderingComponent.
                    let root_rendering_component = root.unwrap_mut::<RootRenderingComponent>();
                    root_rendering_component.update_from_footer();
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
                })
                .finish()])
            .finish()
    }
}
