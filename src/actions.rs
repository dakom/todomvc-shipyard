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
    (label, completed):(String, bool), 
    mut entities:EntitiesViewMut, 
    mut todos:ViewMut<Todo>,
    mut list_changes: UniqueViewMut<ListChanges>, 
    mut dirty_toggles:ViewMut<DirtyToggle>,
) {

    let id = entities.add_entity(&mut todos, Todo::new(label, completed));
    list_changes.push(ListChange::Append(id));

    if completed {
        entities.add_component(&mut dirty_toggles, DirtyToggle{}, id);
    }
}

pub fn toggle_todo(
    id:EntityId, 
    mut todos:ViewMut<Todo>, 
    mut dirty_toggles:ViewMut<DirtyToggle>,
    entities:EntitiesViewMut, 
) {
    if let Ok(todo) = (&mut todos).get(id) {
        todo.completed = !todo.completed;
        entities.add_component(&mut dirty_toggles, DirtyToggle{}, id);
    }
}

pub fn clear_completed(
    mut list_changes:UniqueViewMut<ListChanges>,
    todos: View<Todo>,
) {
    todos.iter().with_id().for_each(|(id, todo)| {
        if todo.completed {
            list_changes.push(ListChange::Remove(id));
        }
    });
}

pub fn delete_todo(
    id: EntityId,
    mut list_changes:UniqueViewMut<ListChanges>,
) {
    list_changes.push(ListChange::Remove(id));
}

pub fn save_edit(
    id: EntityId,
    mut todos:ViewMut<Todo>, 
    mut dirty_labels:ViewMut<DirtyLabel>,
    entities:EntitiesViewMut, 
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
    mut dirty_editing:ViewMut<DirtyEditing>,
    entities:EntitiesViewMut, 
) {
    if let Ok(todo) = (&mut todos).get(id) {
        if todo.editing != editing {
            todo.editing = editing;
            entities.add_component(&mut dirty_editing, DirtyEditing{}, id);
        }
    }
}

pub fn toggle_all(
    entities:EntitiesViewMut, 
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
