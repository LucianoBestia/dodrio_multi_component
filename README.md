Things are changing fast. This is the situation on 2019-05-16. LucianoBestia  
# dodrio_multi_component
How to use dodrio vdom with multiple components?  
The components must be reusable and cacheable.  
They will always have a RootRenderingComponent above them. It is the only one who knows their relationship. The sub Components cannot know their relationship.  
Only the Root can be used for events (on click).  

# the question
Is the use of `Rc<RefCell<<AppData>>>` the best approach here?  
This means that the borrow checker is now dynamic at runtime.  
Is there a way to have here the static borrow checker in compile time?  
  
# trying new approaches
The old code is renamed to lib.rs_OldWithRcRefCell.  

16.05.2019 I tried to change the code following the suggestion of fitzgen:  
https://github.com/fitzgen/dodrio/issues/78  
But it does not allow for cache-able components. I cannot put a Component inside the RootComponent that has the data and then a reference to that same data. Then I get a self-referencing struct. That is not allowed in basic safe rust.  
Maybe it can be efficient with the new construct Pin<> for self-referencing structs?  
  
16.05.2019 Another approach I try is to put cache fields inside the components. They will be copied or cloned from the app_data. Having 2 copies enables to check if anything has changed and invalidates the Render Cache. 
It looks promising, but copying large amounts of data is not very nice.  
  
# just an example
I created a silly example.  
In the browser there are 3 sections (components) of text with 3 counters.  
When you click on the text, a counter is incremented.  
This is not the counter of that section, but of another.  
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
`cargo make`  
Sometimes you need to refresh the webpage in the browser, to get the new wasm.  
## VSCode
Use Fold and Unfold regions to read the source code easier.  








