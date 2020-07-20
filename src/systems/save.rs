use shipyard::*;
use crate::{
    components::*,
    storage::save_data
};


pub fn save_todos (
    todos:View<Todo>,
    order:UniqueView<Order>,
) {
    //let output = todos.iter().with_id()

    let data:Vec<(&str, bool)> = 
        order
            .iter()
            .map(|id| todos.get(*id).unwrap())
            .map(|todo| (todo.label.as_ref(), todo.completed))
            .collect();

    save_data(data);
}
