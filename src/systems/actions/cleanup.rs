use shipyard::*;
use crate::components::*;


//Not exactly rendering, but part of the RENDER workflow
pub fn clear_dirty(
    mut dirty_toggles:ViewMut<DirtyToggle>,
    mut dirty_editing:ViewMut<DirtyEditing>,
    mut dirty_labels:ViewMut<DirtyLabel>,
    mut order:OrderViewMut,
) {
    dirty_toggles.clear();
    dirty_editing.clear();
    dirty_labels.clear();
    order.pending_render = None;
}
