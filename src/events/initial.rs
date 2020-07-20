use shipyard::*;
use gloo_events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlElement, Document};
use std::rc::Rc;
use awsm_web::dom;
use crate::{
    dom::selector,
    events::handlers,
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

