use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlElement, Document};
use gloo_events::EventListener;
use std::rc::Rc;
use crate::{
    components::*,
    dom,
    dom::selector,
    events,
    templates::TemplateManager
};
use super::render;

pub fn update_filter(mut filter:UniqueViewMut<BottomFilter>) {
    filter.reset();
}

pub fn append_todo(
    label:String, 
    mut entities:EntitiesViewMut, 
    mut order:OrderViewMut,
    mut todos:ViewMut<Todo>,
    mut event_listeners:LocalViewMut<EventListeners>,
    world: WorldView,
    tm:TemplateManagerView,
    doc:DocumentView,
) {
    //create the entity
    let id = entities.add_entity((), ());

    //render dom fragment - we need to do this here so that the selectors are valid
    render::new_todo((id,&label),&doc, &tm);

    //push the entitity onto the order
    order.list.push_front(id);

    //add the components to the entity: Todo and EventListeners
    entities.add_component(
        (&mut todos, &mut *event_listeners), 
        (
            Todo::new(label),
            EventListeners(vec![
                EventListener::new(&selector::todo_toggle(&doc, id), "click", {
                    let world = world.clone();
                    move |event| {
                        events::todo_toggled(world.clone(), event.dyn_ref().unwrap_throw(), id)
                    }
                }),
                EventListener::new(&selector::todo(&doc, id), "dblclick", {
                    let world = world.clone();
                    move |event| {
                        events::todo_start_editing(world.clone(), id)
                    }
                }),
                EventListener::new(&selector::todo_delete(&doc, id), "click", {
                    let world = world.clone();
                    move |event| {
                        events::todo_delete(world.clone(), id)
                    }
                }),
            ])
        ), 
        id
    );
}

pub fn toggle_todo(
    id:EntityId, 
    mut todos:ViewMut<Todo>, 
    mut dirty_toggles:ViewMut<DirtyToggle>,
    mut entities:EntitiesViewMut, 
) {
    if let Ok(todo) = (&mut todos).get(id) {
        todo.completed = !todo.completed;
        entities.add_component(&mut dirty_toggles, DirtyToggle{}, id);
    }
}

pub fn clear_completed(
    mut storages:AllStoragesViewMut,
) {

    let to_delete = {
        let todos:View<Todo> = storages.borrow();
        todos.iter().with_id().fold(Vec::new(), |mut acc, (id, todo)| {
            if todo.completed {
                acc.push(id);
            }
            acc
        })
    };

    {
        for id in to_delete.iter() {
            storages.delete(*id);
        }
    }

    {
        let mut order:OrderViewMut = storages.borrow();
        order.list.retain(|order_id| !to_delete.contains(&order_id));
        order.pending_render = Some(ListChange::RemoveMulti(to_delete));
    }
}

pub fn delete_todo(
    id:EntityId, 
    mut storages:AllStoragesViewMut,
) {
    {
        let mut order:OrderViewMut = storages.borrow();
        order.list.retain(|order_id| *order_id != id);
        order.pending_render = Some(ListChange::Remove(id));
    }

    storages.delete(id);
}

pub fn save_edit(
    id: EntityId,
    mut todos:ViewMut<Todo>, 
    mut dirty_labels:ViewMut<DirtyLabel>,
    mut entities:EntitiesViewMut, 
    doc:DocumentView,
) {
    if let Ok(todo) = (&mut todos).get(id) {
        let elem:HtmlInputElement = selector::todo_edit(&doc, id);
        todo.label = elem.value();
        entities.add_component(&mut dirty_labels, DirtyLabel{}, id);
    }
}

pub fn editing_todo(
    (id, editing):(EntityId, bool), 
    mut todos:ViewMut<Todo>, 
    mut todo_editing:LocalViewMut<Editing>, 
    mut dirty_editing:ViewMut<DirtyEditing>,
    mut entities:EntitiesViewMut, 
    doc:DocumentView,
    world:WorldView,
) {
    if let Ok(todo) = (&mut todos).get(id) {
        if todo.editing != editing {
            todo.editing = editing;
            entities.add_component(&mut dirty_editing, DirtyEditing{}, id);
            if editing {
                let elem:HtmlInputElement = selector::todo_edit(&doc, id);
                entities.add_component(
                    &mut *todo_editing,
                    Editing {
                        on_blur: EventListener::new(&elem, "blur", {
                            let world = world.clone();
                            move |event| {
                                events::todo_finish_editing(world.clone(), id)
                            }
                        }),
                        on_keydown: EventListener::new(&elem, "keydown", {
                            let world = world.clone();
                            let elem = elem.clone();
                            move |event| {
                                events::todo_edit_keydown(world.clone(), id, &elem, event.dyn_ref().unwrap_throw());
                            }
                        }),
                    }, 
                    id
                );
            } else {
                todo_editing.delete(id);
            }
        }
    }
}
pub fn toggle_all(
    mut entities:EntitiesViewMut, 
    mut todos:ViewMut<Todo>,
    mut dirty_toggles:ViewMut<DirtyToggle>,
) {
    let all_completed = todos.iter().into_iter().all(|todo| todo.completed);
    let completed = !all_completed;

    for (id, todo) in (&mut todos).iter().with_id() {
        if todo.completed != completed {
            todo.completed = completed;
            entities.add_component(&mut dirty_toggles, DirtyToggle{}, id);
        }
    }
}
