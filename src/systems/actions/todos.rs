use shipyard::*;
use web_sys::HtmlInputElement;
use crate::{
    components::*,
    dom::selector,
};

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


    //push the entitity onto the order
    order.list.push_front(id);

    order.pending_render = Some(ListChange::Append(id));

    entities.add_component(&mut todos, Todo::new(label), id);
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
