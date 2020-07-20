use crate::components::*;
use shipyard::*;

pub fn delete_pending(
    mut all_storages:AllStoragesViewMut
) {
    all_storages.delete_any::<(DeleteMe,)>();
}
