//RootRenderingComponent is the only one who really knows about app_data, the other comonents and their relationship.
use crate::appdata::AppData;
use crate::contentrenderingcomponent::ContentRenderingComponent;
use crate::footerrenderingcomponent::FooterRenderingComponent;
use crate::headerrenderingcomponent::HeaderRenderingComponent;

use dodrio::builder::*;
use dodrio::bumpalo::Bump;
use dodrio::{Cached, Node, Render};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RootRenderingComponent {
    //header and footer are cached because they change rarely.
    //content change often and it does not need cache.
    pub header_rendering_component: Cached<HeaderRenderingComponent>,
    pub content_rendering_component: ContentRenderingComponent,
    pub footer_rendering_component: Cached<FooterRenderingComponent>,
    ///shared mutable data
    pub rc_app_data: Rc<RefCell<AppData>>,
}

impl Default for RootRenderingComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl RootRenderingComponent {
    pub fn new() -> Self {
        let app_data = AppData::new();
        let rc_app_data = Rc::new(RefCell::new(app_data));
        let header_rendering_component = Cached::new(HeaderRenderingComponent {
            rc_app_data: Rc::<std::cell::RefCell<AppData>>::clone(&rc_app_data),
        });
        let content_rendering_component = ContentRenderingComponent {
            rc_app_data: Rc::<std::cell::RefCell<AppData>>::clone(&rc_app_data),
        };
        let footer_rendering_component = Cached::new(FooterRenderingComponent {
            rc_app_data: Rc::<std::cell::RefCell<AppData>>::clone(&rc_app_data),
        });

        Self {
            header_rendering_component,
            content_rendering_component,
            footer_rendering_component,
            rc_app_data,
        }
    }
    //RootRenderingComponent must know the relations between Components.
    //The sub Components don't know anything about their relationships.
    pub fn update_from_header(&mut self) {
        //some non-reusable changes can be made by RootRenderingComponent
        {
            let mut app_data = self.rc_app_data.borrow_mut();
            app_data.description.push('x');
        }
        // some reusable changes by the sub RenderingComponent
        self.header_rendering_component.update_counter2();

        //nothing to invalidate. content is not cached.
    }

    pub fn update_from_content(&mut self) {
        //some non-reusable changes can be made by RootRenderingComponent
        {
            let mut app_data = self.rc_app_data.borrow_mut();
            app_data.author.push('y');
        }
        //some reusable changes by the sub RenderingComponent
        self.content_rendering_component.update_counter3();

        //invalidated another component for rerendering
        //this is a problem to know what change invalidates what component
        Cached::invalidate(&mut self.header_rendering_component);
        Cached::invalidate(&mut self.footer_rendering_component);
    }

    pub fn update_from_footer(&mut self) {
        //some non-reusable changes can be made by RootRenderingComponent
        {
            let mut app_data = self.rc_app_data.borrow_mut();
            app_data.title.push('z');
        }
        //some reusable changes by the sub RenderingComponent
        self.footer_rendering_component.update_counter1();
        //invalidated another component for rerendering
        //this is a problem to know what change invalidates what component
        Cached::invalidate(&mut self.header_rendering_component);
        Cached::invalidate(&mut self.footer_rendering_component);
    }
}
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
