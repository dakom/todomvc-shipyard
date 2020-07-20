use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Element, HtmlElement, Document};
use gloo_events::EventListener;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::{
    components::*,
    dom,
    dom::selector,
    templates::TemplateManager
};
use super::actions::todos::append_todo;
use super::events::bind::InitialEvents;

pub const SETUP:&'static str = "setup";

pub fn global_uniques(
    (tm, document, body, world): (TemplateManager, Document, Element, Rc<World>),
    storages:AllStoragesViewMut
) {

    let initial_events = InitialEvents::bind(&document, world.clone());

    storages.add_unique_non_send_sync(world);
    storages.add_unique_non_send_sync(tm);
    storages.add_unique(Order(VecDeque::new()));
    storages.add_unique(TodoListChange(None));
    storages.add_unique(BottomFilter::new());
    storages.add_unique_non_send_sync(document);
    storages.add_unique_non_send_sync(DomRoot(body));


    storages.add_unique_non_send_sync(ToggleAll{
        on_click: initial_events.toggle_all_click
    });
    storages.add_unique_non_send_sync(MainInput {
        on_keydown: initial_events.main_input_keydown
    });
    storages.add_unique_non_send_sync(ClearCompleted {
        on_click: initial_events.clear_completed_click
    });
    storages.add_unique_non_send_sync(Router {
        on_location: initial_events.router_location
    });
}

pub fn load(
    world:WorldView,
    mut entities:EntitiesViewMut, 
    mut order:UniqueViewMut<Order>,
    mut todos:ViewMut<Todo>,
    mut event_listeners:LocalViewMut<EventListeners>,
    tm:TemplateManagerView,
    doc:DocumentView,
) {
    /*
    append_todo(
        "loading and saving".to_string(),
        entities,
        order,
        todos,
        event_listeners,
        world,
        tm,
        doc,
    );
    */
}
