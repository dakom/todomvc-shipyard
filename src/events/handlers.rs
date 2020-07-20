use shipyard::*;
use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent, MouseEvent};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::systems::workloads::run_update;
use crate::actions;
const ENTER_KEY:&'static str = "Enter";
const ESCAPE_KEY:&'static str = "Escape";


pub fn location_change(world:Rc<World>) {
    world.run(actions::update_filter);
    run_update(&world);
}

pub fn main_input_keydown(world:Rc<World>, event:&KeyboardEvent) {
    if event.key() == ENTER_KEY {
        let elem:HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
        let value = elem.value();
        let value = value.trim();

        if !value.is_empty() {
            world.run_with_data(actions::append_todo, value.to_string());
            run_update(&world);
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
    run_update(&world);
}
pub fn todo_delete(world:Rc<World>, id:EntityId) {
    world.run_with_data(actions::delete_todo, id);
    run_update(&world);
}

pub fn todo_toggled(world:Rc<World>, _event:&MouseEvent, id:EntityId) {
    world.run_with_data(actions::toggle_todo, id);
    run_update(&world);
}

pub fn todo_start_editing(world:Rc<World>, id:EntityId) {
    world.run_with_data(actions::editing_todo, (id, true));
    run_update(&world);
}

pub fn todo_finish_editing(world:Rc<World>, id:EntityId) {
    world.run_with_data(actions::editing_todo, (id, false));
    run_update(&world);
}
pub fn all_toggled(world:Rc<World>) {
    world.run(actions::toggle_all);
    run_update(&world);
}
