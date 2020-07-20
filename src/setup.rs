use shipyard::*;
use web_sys::{Element, Document};
use std::collections::VecDeque;
use std::rc::Rc;
use crate::{
    components::*,
    templates::TemplateManager,
    storage,
    events::initial::InitialEvents,
    actions,
};

pub fn global_uniques(
    (tm, document, body, initial_events, world): (TemplateManager, Document, Element, InitialEvents, Rc<World>),
    storages:AllStoragesViewMut
) {


    storages.add_unique_non_send_sync(world);
    storages.add_unique_non_send_sync(tm);
    storages.add_unique(Order(VecDeque::new()));
    storages.add_unique(ListChanges(Vec::new()));
    storages.add_unique(BottomFilter::new());
    storages.add_unique_non_send_sync(document);
    storages.add_unique_non_send_sync(DomRoot(body));
    storages.add_unique_non_send_sync(storage::get_local_storage());


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

pub fn load(world:&World) {
    if let Some(data) = storage::load_data() { 
        //add in reverse order because it's ascending by recent added
        for (label, completed) in data.items.into_iter().rev() {
            world.run_with_data(actions::append_todo, (label, completed));
        }
    }
}
