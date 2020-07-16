use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlElement, Document};
use gloo_events::EventListener;
use std::rc::Rc;
use crate::{
    components::*,
    dom,
    dom::selector,
    events,
    templates::TemplateManager
};
use super::actions::append_todo;

pub fn load(
    world:WorldView,
    mut entities:EntitiesViewMut, 
    mut order:OrderViewMut,
    mut todos:ViewMut<Todo>,
    mut event_listeners:LocalViewMut<EventListeners>,
    tm:TemplateManagerView,
    doc:DocumentView,
) {
    append_todo(
        "hello world".to_string(),
        entities,
        order,
        todos,
        event_listeners,
        world,
        tm,
        doc,
    );

}
pub fn toggle_all(storages:AllStoragesViewMut) {
    let world:WorldView = storages.borrow();
    let doc:DocumentView = storages.borrow();
    let elem:HtmlElement = selector::toggle_all(&doc);
    let on_click = EventListener::new(&elem, "click", {
        let world = world.clone();
        move |_| {
            events::all_toggled(world.clone());
        }
    });

    storages.add_unique_non_send_sync(ToggleAll{
        on_click,
    });
}

pub fn main_input(storages:AllStoragesViewMut) {
    let world:WorldView = storages.borrow();
    let doc:DocumentView = storages.borrow();
    let elem:HtmlInputElement = dom::get_element_by_id(&doc, "main-input");
    let on_keydown = EventListener::new(&elem, "keydown", {
        let world = world.clone();
        let elem = elem.clone();
        move |event| {
            events::main_input_keydown(world.clone(), &elem, event.dyn_ref().unwrap_throw())
        }
    });

    storages.add_unique_non_send_sync(MainInput {
        elem,
        on_keydown 
    });
}

pub fn clear_completed(storages:AllStoragesViewMut) {
    let world:WorldView = storages.borrow();
    let doc:DocumentView = storages.borrow();

    let elem:HtmlElement = selector::clear_completed(&doc);
    let on_click = EventListener::new(&elem, "click", {
        let world = world.clone();
        move |event| {
            events::clear_completed(world.clone());
        }
    });

    storages.add_unique_non_send_sync(ClearCompleted {
        on_click
    });
}

pub fn router(storages:AllStoragesViewMut) {
    let world:WorldView = storages.borrow();
    let window = web_sys::window().unwrap_throw();

    let on_location= EventListener::new(&window, "hashchange", {
        let world = world.clone();
        move |_| {
            events::location_change(world.clone());
        }
    });

    storages.add_unique_non_send_sync(Router {
        on_location
    });
}
