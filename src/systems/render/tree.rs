use shipyard::*;
use web_sys::Element;
use crate::{
    components::*,
    dom,
    dom::selector,
};

pub fn list(order: OrderView, doc:DocumentView, todos:View<Todo>, tm:TemplateManagerView) {

    if let Some(change) = &order.pending_render {
        match change {
            ListChange::Remove(id) => {
                let elem:Element = selector::todo(&doc, *id);
                elem.remove();
            },
            ListChange::RemoveMulti(ids) => {
                for id in ids {
                    let elem:Element = selector::todo(&doc, *id);
                    elem.remove();
                }
            }
            ListChange::Append(id) => {
                if let Ok(todo) = (&todos).get(*id) {
                    dom::prepend_to_id(&doc, "todo-list", tm.todo_item(&todo.label, *id));
                }
            }
            _ => {}
        }
    }
}
