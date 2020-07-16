use shipyard::*;
use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent, MouseEvent};
use std::rc::Rc;
use crate::systems::{actions, RENDER};
const ENTER_KEY:&'static str = "Enter";
const ESCAPE_KEY:&'static str = "Escape";


pub fn location_change(world:Rc<World>) {
    world.run(actions::update_filter);
    world.run_workload(RENDER);
}

pub fn main_input_keydown(world:Rc<World>, elem:&HtmlInputElement, event:&KeyboardEvent) {
    if event.key() == ENTER_KEY {
        let value = elem.value();
        let value = value.trim();

        if !value.is_empty() {
            world.run_with_data(actions::append_todo, value.to_string());
            world.run_workload(RENDER);
        }
        elem.set_value("");
    }
}

pub fn todo_edit_keydown(world:Rc<World>, id:EntityId, elem:&HtmlInputElement, event:&KeyboardEvent) {
    let key = event.key();

    if key == ENTER_KEY {
        let value = elem.value();
        let value = value.trim();

        if value.is_empty() {
            todo_delete(world.clone(), id);
            return;
        } else {
            world.run_with_data(actions::save_edit, id);
        }
    } 
    if key == ESCAPE_KEY || key == ENTER_KEY {
        todo_finish_editing(world, id)
    }
}

pub fn clear_completed(world:Rc<World>) {
    world.run(actions::clear_completed);
    world.run_workload(RENDER);
}
pub fn todo_delete(world:Rc<World>, id:EntityId) {
    world.run_with_data(actions::delete_todo, id);
    world.run_workload(RENDER);
}

pub fn todo_toggled(world:Rc<World>, _event:&MouseEvent, id:EntityId) {
    world.run_with_data(actions::toggle_todo, id);
    world.run_workload(RENDER);
}

pub fn todo_start_editing(world:Rc<World>, id:EntityId) {
    world.run_with_data(actions::editing_todo, (id, true));
    world.run_workload(RENDER);
}

pub fn todo_finish_editing(world:Rc<World>, id:EntityId) {
    world.run_with_data(actions::editing_todo, (id, false));
    world.run_workload(RENDER);
}
pub fn all_toggled(world:Rc<World>) {
    world.run(actions::toggle_all);
    world.run_workload(RENDER);
}
