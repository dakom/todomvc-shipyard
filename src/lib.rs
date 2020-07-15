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
    let world = Rc::new(World::default());
    let template_manager = TemplateManager::new(); 
    let (document, body) = init_dom(&template_manager);
    let main_input = init_main_input(world.clone(), &document);

    init_world(
        world.clone(), 
        template_manager, 
        document, 
        body,
        main_input
    );

    world.run_workload(systems::RENDER_VISIBLE);

    //We need to keep the world in memory for the duration of the app
    //In other scenarios, like a game loop, this wouldn't be necessary
    std::mem::forget(Box::new(world));
    log::info!("let's rock and roll!");
}

fn init_dom(tm:&TemplateManager) -> (Document, Element) {
    let document = window().unwrap_throw().document().unwrap_throw();
    let body:Element = document.body().unwrap_throw().into();

    body.append_child(&tm.body()).unwrap_throw();
    body.append_child(&tm.footer()).unwrap_throw();

    (document, body) 
}

fn init_main_input(world:Rc<World>, doc:&Document) -> MainInput {
    let elem:HtmlInputElement = dom::get_element_by_id(&doc, "main-input").unwrap_throw();
    let on_keypress = EventListener::new(&elem, "keypress", {
        let elem = elem.clone();
        move |event| {
            events::main_input_keypress(world.clone(), &elem, event.dyn_ref().unwrap_throw())
        }
    });
    MainInput {
        elem,
        on_keypress 
    }
}

fn init_world(world:Rc<World>, tm:TemplateManager, document:Document, body:Element, main_input:MainInput) {
    world.add_unique_non_send_sync(tm);
    //world.add_unique_non_send_sync(web_sys::window().unwrap_throw().document().unwrap_throw());
    world.add_unique(Order(VecDeque::new()));
    world.add_unique(StatusFilter {status: None} );
    world.add_unique(StatusFilter {status: None} );
    world.add_unique_non_send_sync(main_input);

    world.add_unique_non_send_sync(document);
    world.add_unique_non_send_sync(DomRoot(body));

    world
        .add_workload(systems::RENDER_VISIBLE)
        .with_system(system!(systems::toggle_main_visible))
        .build();

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
