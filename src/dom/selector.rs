use shipyard::EntityId;
use crate::dom::entity_id;

pub fn todo_toggle(id:EntityId) -> String {
    format!("#{} .toggle", entity_id(id))
}
