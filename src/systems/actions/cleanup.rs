use shipyard::*;
use crate::components::*;


pub fn clear_dirty(
    mut dirty_toggles:ViewMut<DirtyToggle>,
    mut dirty_editing:ViewMut<DirtyEditing>,
    mut dirty_labels:ViewMut<DirtyLabel>,
) {
    dirty_toggles.clear();
    dirty_editing.clear();
    dirty_labels.clear();
}


pub fn clear_list_change(
    mut list_change:UniqueViewMut<TodoListChange>,
) {
    list_change.0 = None;
}
