use shipyard::*;
use crate::components::*;

pub fn log_todos (
    todos:View<Todo>,
    order:UniqueView<Order>,
) {
    //let output = todos.iter().with_id()

    let output = 
        order
            .iter()
            .map(|id| {
                (id, todos.get(*id).unwrap())
            })
            .fold(String::new(), |acc, (id, todo)| {
            let spacer = if acc.is_empty() { "" } else { "\n" };

            format!("{}{}{:?}\n{:?}", acc, spacer, id, todo)
        });

    log::info!("{}", output);
}
