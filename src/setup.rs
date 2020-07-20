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
    templates::TemplateManager,
    events::handlers
};
pub struct InitialEvents {
    pub toggle_all_click: EventListener,
    pub main_input_keydown: EventListener,
    pub clear_completed_click: EventListener,
    pub router_location: EventListener
}

impl InitialEvents {
    pub fn bind(doc:&Document, world:Rc<World>) -> Self {
        let elem:HtmlElement = selector::toggle_all(&doc);
        let toggle_all_click = EventListener::new(&elem, "click", {
            let world = world.clone();
            move |_| {
                handlers::all_toggled(world.clone());
            }
        });

        let elem:HtmlInputElement = dom::get_element_by_id(&doc, "main-input");
        let main_input_keydown = EventListener::new(&elem, "keydown", {
            let world = world.clone();
            move |event| {
                handlers::main_input_keydown(world.clone(), event.dyn_ref().unwrap_throw())
            }
        });


        let elem:HtmlElement = selector::clear_completed(&doc);
        let clear_completed_click = EventListener::new(&elem, "click", {
            let world = world.clone();
            move |event| {
                handlers::clear_completed(world.clone());
            }
        });

        let window = web_sys::window().unwrap_throw();

        let router_location= EventListener::new(&window, "hashchange", {
            let world = world.clone();
            move |_| {
                handlers::location_change(world.clone());
            }
        });

        Self {
            toggle_all_click,
            main_input_keydown,
            clear_completed_click,
            router_location
        }
    }
}

pub fn global_uniques(
    (tm, document, body, world): (TemplateManager, Document, Element, Rc<World>),
    storages:AllStoragesViewMut
) {

    let initial_events = InitialEvents::bind(&document, world.clone());

    storages.add_unique_non_send_sync(world);
    storages.add_unique_non_send_sync(tm);
    storages.add_unique(Order(VecDeque::new()));
    storages.add_unique(ListChanges(Vec::new()));
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
