use shipyard::*;
use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent, MouseEvent};
use crate::systems;
use std::rc::Rc;

const ENTER_KEY:u32 = 13;

pub fn main_input_keypress(world:Rc<World>, elem:&HtmlInputElement, event:&KeyboardEvent) {
    if event.key_code() == ENTER_KEY {
        let value = elem.value();
        let value = value.trim();

        if !value.is_empty() {
            world.run_with_data(systems::append_todo, (value.to_string(), world.clone()));
            world.run_workload(systems::RENDER_VISIBLE);
        }
        elem.set_value("");
    }
}

pub fn todo_toggled(world:Rc<World>, event:&MouseEvent, id:EntityId) {
    world.run_with_data(systems::toggle_todo, id);
    log::info!("toggled!");
}
