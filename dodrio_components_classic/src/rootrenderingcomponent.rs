//RootRenderingComponent is the only one who really knows about app_data, the other comonents and their relationship.
//cannot use `dodrio::cache`. It requires the trait Render for sub Components.
//this trait method Render doesn't allow to send `app_data` as parameter.
//`app_data` must not be inside of the subComponent because then we have a self-referenced struct.
//It is possible to use some other type of cache?

use crate::appdata::AppData;
use crate::contentrenderingcomponent;
use crate::contentrenderingcomponent::ContentRenderingComponent;
use crate::footerrenderingcomponent;
use crate::footerrenderingcomponent::FooterRenderingComponent;
use crate::headerrenderingcomponent;
use crate::headerrenderingcomponent::HeaderRenderingComponent;

use dodrio::{Node, Render, RenderContext};

pub struct RootRenderingComponent {
    app_data: AppData,

    header_rendering_component: HeaderRenderingComponent,
    content_rendering_component: ContentRenderingComponent,
    footer_rendering_component: FooterRenderingComponent,
}

impl Default for RootRenderingComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl RootRenderingComponent {
    pub fn new() -> Self {
        let app_data = AppData::new();
        let header_rendering_component = HeaderRenderingComponent::new();
        let content_rendering_component = ContentRenderingComponent::new();
        let footer_rendering_component = FooterRenderingComponent::new();

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
    }

    pub fn update_from_content(&mut self) {
        //some non-reusable changes can be made by RootRenderingComponent
        self.app_data.author.push('y');

        //other reusable changes can be made by the sub RenderingComponent
        self.content_rendering_component
            .update_counter3(&mut self.app_data);
    }

    pub fn update_from_footer(&mut self) {
        //some non-reusable changes can be made by RootRenderingComponent
        self.app_data.title.push('z');

        //other reusable changes can be made by the sub RenderingComponent
        self.footer_rendering_component
            .update_counter1(&mut self.app_data);
    }
}
impl Render for RootRenderingComponent {
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        use dodrio::builder::div;
        //TODO: don't know how to use cache ???
        div(&cx)
            .children([
                headerrenderingcomponent::render(&self.app_data, cx),
                contentrenderingcomponent::render(&self.app_data, cx),
                footerrenderingcomponent::render(&self.app_data, cx),
            ])
            .finish()
    }
}
