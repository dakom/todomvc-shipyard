use shipyard::*;

const CORE:&'static str = "core";
const DOM_TREE:&'static str = "dom-tree";
const DOM_PROPS:&'static str = "dom-props";
const DOM_EVENTS:&'static str = "dom-events";
const CLEANUP:&'static str = "cleanup";
const SAVE:&'static str = "save";

pub fn register(world:&World) {

    world
        .add_workload(CORE)
        .with_system(system!(super::todos::list))
        .with_system(system!(super::maintenance::delete_pending))
        .build();

    world
        .add_workload(DOM_TREE)
        .with_system(system!(super::dom::tree::list))
        .build();

    world
        .add_workload(DOM_EVENTS)
        .with_system(system!(super::dom::bind_events::list))
        .with_system(system!(super::dom::bind_events::editing_todo))
        .build();

    world
        .add_workload(DOM_PROPS)
        .with_system(system!(super::dom::props::main_visible))
        .with_system(system!(super::dom::props::toggles))
        .with_system(system!(super::dom::props::editing))
        .with_system(system!(super::dom::props::labels))
        .with_system(system!(super::dom::props::count))
        .with_system(system!(super::dom::props::toggle_all))
        .with_system(system!(super::dom::props::clear_completed))
        .with_system(system!(super::dom::props::filter))
        .with_system(system!(super::dom::props::filter_selection))
        .build();


    world
        .add_workload(CLEANUP)
        .with_system(system!(super::cleanup::clear_dirty))
        .with_system(system!(super::cleanup::clear_list_changes))
        .build();

    world
        .add_workload(SAVE)
        .with_system(system!(super::save::save_todos))
        .build();
}

pub fn run_update(world:&World) {
    world.run_workload(CORE);
    world.run_workload(DOM_TREE);
    world.run_workload(DOM_PROPS);
    world.run_workload(DOM_EVENTS);
    world.run_workload(CLEANUP);
   
    //not necessary to do every update, but this is easy :P
    save(world);
    //world.run(super::debug::log_todos);
}

pub fn save(world:&World) {
    world.run_workload(SAVE);
}
