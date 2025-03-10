use crate::dom;
use crate::dom::component::Component;
use crate::dom::component::Composable;
use crate::native;
use crate::task;
use std::any::Any;
use std::cell::RefCell;

thread_local!(static APP: RefCell<Option<App>> = RefCell::new(None));

struct App {
    root_component: Box<dyn Composable>,
    dom_renderer: dom::Renderer,
}

pub fn init<M, S, B>(mut root_component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
{
    let node = root_component.render_dom(None);
    let root = native::get_element_by_id(id);
    let dom_renderer = dom::Renderer::new(node, root.into());
    let root_component: Box<dyn Composable> = Box::new(root_component);
    APP.with(|app| {
        *app.borrow_mut() = Some(App {
            root_component,
            dom_renderer,
        })
    });
}

pub fn update(mut id: u128, mut msg: Box<dyn Any>) {
    APP.with(|app| {
        if let Some(app) = &mut (*app.borrow_mut()) {
            while let Some((new_msg, new_id)) = app.root_component.update(id, msg) {
                msg = new_msg;
                id = new_id;
            }
            let node = app.root_component.render_dom(Some(id));
            app.dom_renderer.update(node);
        }
    });
    task::dispatch();
}
