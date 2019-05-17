//RootRenderingComponent is the only one who really knows about app_data, the other comonents and their relationship.
use crate::appdata::AppData;
use crate::contentrenderingcomponent::ContentRenderingComponent;
use crate::footerrenderingcomponent::FooterRenderingComponent;
use crate::headerrenderingcomponent::HeaderRenderingComponent;

use dodrio::builder::*;
use dodrio::bumpalo::Bump;
use dodrio::{Cached, Node, Render};

pub struct RootRenderingComponent {
    app_data: AppData,

    header_rendering_component: Cached<HeaderRenderingComponent>,
    content_rendering_component: Cached<ContentRenderingComponent>,
    footer_rendering_component: Cached<FooterRenderingComponent>,
}

impl Default for RootRenderingComponent {
    fn default() -> Self {
        Self::new()
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
    pub fn update_from_header(&mut self) {
        //some non-reusable changes can be made by RootRenderingComponent
        self.app_data.description.push('x');

        //other reusable changes can be made by the sub RenderingComponent
        self.header_rendering_component
            .update_counter2(&mut self.app_data);

        //what components need rendering
        self.invalidate_components();
    }

    pub fn update_from_content(&mut self) {
        //some non-reusable changes can be made by RootRenderingComponent
        self.app_data.author.push('y');

        //other reusable changes can be made by the sub RenderingComponent
        self.content_rendering_component
            .update_counter3(&mut self.app_data);

        //what components need rendering
        self.invalidate_components();
    }

    pub fn update_from_footer(&mut self) {
        //some non-reusable changes can be made by RootRenderingComponent
        self.app_data.title.push('z');

        //other reusable changes can be made by the sub RenderingComponent
        self.footer_rendering_component
            .update_counter1(&mut self.app_data);

        //what components need rendering
        self.invalidate_components();
    }

    fn invalidate_components(&mut self) {
        //app_data can change any time anywhere.
        //Components must update their cached values and return true if they changed
        //to invalidate the Render Cache.
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
