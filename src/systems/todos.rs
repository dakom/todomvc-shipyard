use shipyard::*;
use crate::components::*;

pub fn list (
    list_changes: UniqueView<ListChanges>, 
    entities:EntitiesViewMut, 
    mut order:UniqueViewMut<Order>,
    mut delete_me:ViewMut<DeleteMe>,
) {

    for change in list_changes.iter() {
        match change {
            ListChange::Remove(id) => {
                entities.add_component(&mut delete_me, DeleteMe{}, *id);
                order.retain(|order_id| *order_id != *id);
            },
            ListChange::Append(id) => {
                order.push_front(*id);
            }
        }
    }
}
