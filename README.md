Things are changing fast. This is the situation on 2019-05-16. LucianoBestia  
2019-05-17 copied the dodrio code from GitHub, to have new features - RenderContext
# dodrio_multi_component
How to use dodrio vdom with multiple components?  
The components must be reusable and cacheable. Think about Header and Footer.  
They will always have a RootRenderingComponent above them. It is the only one who knows the relationships between subComponents. The subComponents cannot know their relationship.  
For now there is only one level of nesting components. More then that becomes complicated.  
Only the Root can be used for events (on click).  
Rust does not have true OOP, so the approach must be different. There is a concept of modules in Rust, that isolates the code and data.  
Separate *.rs files are automatically separate modules.  
# different approaches
Now I found the RenderContext that I have to explore for this solution.  
In separate folders I created different working approaches to the same problem:  
- classic (good reuse, not good for dodrio::cache, maybe another cache?)
- Rc RefCell (good reuse+cache, but runtime borrow checker)
- cached local values (good reuse+cache, but copying data for cached values)
- ??maybe Pin<> for self-referencing struct?
## classic
It works well until I want to use dodrio::cache. This needs the Render trait. The Render function cannot accept the app_data parameter. Maybe using a different type of cache? Dodrio::cache is not the final version, I think.  
## Rc RefCell
Use of Rc RefCell means that the borrow checker is now dynamic at runtime.  
That is not ideal.  
## cached values
Cached values inside Components are copied or cloned from the app_data. Having 2 copies enables to check if anything has changed and invalidates the Render Cache.  
It looks promising, but copying large amounts of data is not very nice.  
## maybe Pin<>?
This is a new thing and needs a little bit of research.  
# just an example
I created a silly example.  
In the browser there are 3 sections (components) of text with 3 counters.  
When you click on the text, a counter is incremented.  
This is not the counter of that section, but of another. So we have a complicated subComponents relationship.  
The components don't know one about the other. But the Root knows them all.  
And all the components have access to the app_data.  
Some of the RenderingComponents are rerendered because the cache is invalidated.  
Other components are not rerendered.  
# rendering components (visual components)
I splitted the web page in 3 vertical RenderingComponents:  
- header
- content
- footer
# dodrio vdom - only one
If I understand correctly there must be only one vdom with only one RootRenderingComponent.
The RootRenderingComponent has to be moved into the vdom. So this is the only struct, that we have access must have access to header, content and footer RenderComponents. The easiest way is to create them inside the RootRenderingComponent.  
They will all need access to app_data. That data will be passed as function parameter.  
# cache
I use dodrio cache for components, because they will change rarely.

# struct model (object model)
This can vary greatly for different approach.  
All of the RenderingComponents need to have access to the same AppData struct.  
The events from vdom must mutate app_data.  
The rendering itself does not mutate app_data.  
```
  -------------------------                               -------------------   
  |RootRenderingComponent |                               |                 |   
  |                       |                         |     |     AppData     |   
  |  ------------------   |                         |     |                 |   
  |  |                |   |                         |     |    - Title      |   
  |  |  Header        | ---------------------->     |     |                 |   
  |  |                |   |                         |     |    - Description|   
  |  ------------------   |                         |     |                 |   
  |                       |                         |     |    - Author     |   
  |  ------------------   |------------------->     |     |                 |   
  |  |                |   |                         |     |    - Date       |   
  |  |  Content       | ---------------------->     |     |                 |   
  |  |                |   |                         |     |                 |   
  |  ------------------   |                         |     |                 |   
  |                       |                         |     |                 |   
  |  ------------------   |                         |     |                 |   
  |  |                |   |                         |     |                 |   
  |  |  Footer        |------------------------->   |     |                 |   
  |  |                |   |                         |     |                 |   
  |  ------------------   |                         |     |                 |   
  |                       |                         |     |                 |   
  -------------------------                               -------------------   
```
## prerequisites
`cargo install basic-http-server`  
`cargo install cargo-make`  
The Makefile.toml is prepared for Windows with Chrome.  

## server and run
Go into a subfolder for a single approach and  
`cargo make`  
Sometimes you need to refresh the webpage in the browser, to get the new wasm.  
## VSCode
Use Fold and Unfold regions to read the source code easier.  








