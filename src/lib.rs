//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate derive_more;

mod templates;
mod systems;
mod components;
mod dom;
mod setup;
mod actions;
mod events;
mod storage;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use shipyard::*;
use gloo_events::EventListener;
use web_sys::{window, Element, Document, HtmlInputElement};
use crate::{
    components::*,
    templates::TemplateManager,
    events::initial::InitialEvents,
};

#[wasm_bindgen(start)]
pub async fn main_js() {
    init_logger();

    //init dom stuff
    let template_manager = TemplateManager::new(); 

    let document = window().unwrap_throw().document().unwrap_throw();
    let body:Element = document.body().unwrap_throw().into();

    body.append_child(&template_manager.body()).unwrap_throw();
    body.append_child(&template_manager.footer()).unwrap_throw();

    //init world
    let world = Rc::new(World::default());
    let initial_events = InitialEvents::bind(&document, world.clone());
    world.run_with_data(setup::global_uniques, (
            template_manager, 
            document, 
            body, 
            initial_events,
            world.clone()
    ));
    systems::workloads::register(&world);

    setup::load(&world);

    systems::workloads::run_update(&world);
}

// enable logging and panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
        fn init_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn init_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}
