pub mod actions;
pub mod render;
pub mod setup;
pub mod events;

use shipyard::*;

pub fn register_workloads(world:&World) {
    world
        .add_workload(render::TREE)
        .with_system(system!(render::tree::list))
        .build();

    world
        .add_workload(events::BIND)
        .with_system(system!(events::bind::list))
        .with_system(system!(events::bind::editing_todo))
        .build();

    world
        .add_workload(render::PROPS)
        .with_system(system!(render::props::main_visible))
        .with_system(system!(render::props::toggles))
        .with_system(system!(render::props::editing))
        .with_system(system!(render::props::labels))
        .with_system(system!(render::props::count))
        .with_system(system!(render::props::toggle_all))
        .with_system(system!(render::props::clear_completed))
        .with_system(system!(render::props::filter))
        .with_system(system!(render::props::filter_selection))
        .build();

    world
        .add_workload(actions::CLEANUP)
        .with_system(system!(actions::cleanup::clear_dirty))
        .with_system(system!(actions::cleanup::clear_list_change))
        .build();

}
pub fn update_dom(world:&World) {
    world.run_workload(render::TREE);
    world.run_workload(render::PROPS);
    world.run_workload(events::BIND);
    world.run_workload(actions::CLEANUP);
}
