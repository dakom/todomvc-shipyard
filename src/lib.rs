//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod templates;
mod systems;
mod components;
mod dom;
mod events;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::VecDeque;
use std::rc::Rc;
use shipyard::*;
use gloo_events::EventListener;
use web_sys::{window, Element, Document, HtmlInputElement};
use crate::{
    components::*,
    templates::TemplateManager,
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

    world.add_unique_non_send_sync(world.clone());
    world.add_unique_non_send_sync(template_manager);
    world.add_unique(Order{ list: VecDeque::new(), pending_render: None});
    world.add_unique(BottomFilter::new());
    world.add_unique_non_send_sync(document);
    world.add_unique_non_send_sync(DomRoot(body));

    world
        .add_workload(systems::RENDER)
        .with_system(system!(systems::render::main_visible))
        .with_system(system!(systems::render::toggles))
        .with_system(system!(systems::render::editing))
        .with_system(system!(systems::render::labels))
        .with_system(system!(systems::render::list))
        .with_system(system!(systems::render::count))
        .with_system(system!(systems::render::toggle_all))
        .with_system(system!(systems::render::clear_completed))
        .with_system(system!(systems::render::filter))
        .with_system(system!(systems::render::filter_selection))
        .with_system(system!(systems::render::clear_dirty))
        .build();

    world
        .add_workload(systems::SETUP)
        .with_system(system!(systems::setup::main_input))
        .with_system(system!(systems::setup::toggle_all))
        .with_system(system!(systems::setup::router))
        .with_system(system!(systems::setup::load))
        .with_system(system!(systems::setup::clear_completed))
        .build();


    world.run_workload(systems::SETUP);
    world.run_workload(systems::RENDER);

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
