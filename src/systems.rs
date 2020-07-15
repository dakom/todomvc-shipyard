use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo_events::EventListener;
use std::rc::Rc;
use crate::{
    components::*,
    dom,
    dom::selector,
    events,
};
pub const RENDER_VISIBLE:&'static str = "render-visible";

pub fn toggle_main_visible(root:DomRootView, order: OrderView) {

    let display = if order.0.is_empty() { "none" } else { "block" };

    dom::set_style_by_class(&root.0, "main", "display", display);
    dom::set_style_by_class(&root.0, "footer", "display", display);
}

pub fn append_todo(
    data: (String, Rc<World>), 
    mut entities:EntitiesViewMut, 
    mut order:OrderViewMut,
    mut todos:ViewMut<Todo>,
    mut event_listeners:EventListenersViewMut,
    tm:TemplateManagerView,
    doc:DocumentView,
) {
    let (label, world) = data;

    //create the entity
    let id = entities.add_entity((), ());

    //render dom fragment
    dom::prepend_to_id(&doc, "todo-list", tm.todo_item(&label, id));

    //push the entitity onto the order
    order.0.push_front(id);


    //add the components to the entity: Todo and EventListeners
    entities.add_component(
        (&mut todos, &mut *event_listeners), 
        (
            Todo::new(label),
            EventListeners(vec![
                EventListener::new(&dom::select(&doc, &selector::todo_toggle(id)).unwrap_throw(), "click", move |event| {

                    events::todo_toggled(world.clone(), event.dyn_ref().unwrap_throw(), id)
                })
            ])
        ), 
        id
    );
}

pub fn toggle_todo(id:EntityId, mut todos:ViewMut<Todo>) {
    if let Ok(todo) = (&mut todos).get(id) {
        todo.status = {
            if todo.status == Status::Active 
                { Status::Completed } 
            else
                { Status::Active }
        };
    }
}
