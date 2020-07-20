use shipyard::*;
use web_sys::Element;
use awsm_web::dom;
use crate::{
    components::*,
    dom::selector,
};

pub fn list(
    list_changes: UniqueView<ListChanges>, 
    doc:DocumentView, 
    todos:View<Todo>, 
    tm:TemplateManagerView
) {

    for change in list_changes.iter() {
        match change {
            ListChange::Remove(id) => {
                let elem:Element = selector::todo(&doc, *id);
                elem.remove();
            },
            ListChange::Append(id) => {
                if let Ok(todo) = (&todos).get(*id) {
                    dom::prepend_to_id(&doc, "todo-list", tm.todo_item(&todo.label, *id));
                }
            }
        }
    }
}
