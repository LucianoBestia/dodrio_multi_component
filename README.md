# dodrio_multi_component
How to use dodrio vdom with multiple components?  
# the question
Is the use of `Rc<RefCell<<AppData>>>` the best approach here?  
This means that the borrow checker is now dynamic at runtime.  
Is there a way to have here the static borrow checker in compile time?  
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
If I understand correctly, there must be only one vdom with only one RootRenderingComponent.  
The RootRenderingComponent has to be moved into the vdom. So this is the only struct, that we have access from the vdom events - on click.  
This means that this struct must have access to header, content and footer RenderComponents. The easiest way is to create them inside the RootRenderingComponent.  
They will all need access to app_data.  
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








