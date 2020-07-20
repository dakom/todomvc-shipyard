use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use web_sys::{HtmlElement, Document, HtmlInputElement};
use gloo_events::EventListener;
use awsm_web::dom;
use crate::{
    components::*,
    dom::selector,
};
use crate::events::handlers;

pub fn list(
    mut entities:EntitiesViewMut, 
    mut event_listeners:LocalViewMut<EventListeners>,
    world: WorldView,
    list_changes: UniqueView<ListChanges>, 
    doc:DocumentView, 
    todos:View<Todo>, 
) {
    for change in list_changes.iter() {
        match change {
            //No need to handle Remove since the entire entity is deleted
            //which will also drop the EventListeners

            ListChange::Append(id) => {
                let id = *id;
                if let Ok(todo) = (&todos).get(id) {

                    entities.add_component(
                        &mut *event_listeners, 
                        EventListeners(vec![
                            EventListener::new(&selector::todo_toggle(&doc, id), "click", {
                                let world = world.clone();
                                move |event| {
                                    handlers::todo_toggled(world.clone(), event.dyn_ref().unwrap_throw(), id)
                                }
                            }),
                            EventListener::new(&selector::todo(&doc, id), "dblclick", {
                                let world = world.clone();
                                move |event| {
                                    handlers::todo_start_editing(world.clone(), id)
                                }
                            }),
                            EventListener::new(&selector::todo_delete(&doc, id), "click", {
                                let world = world.clone();
                                move |event| {
                                    handlers::todo_delete(world.clone(), id)
                                }
                            }),
                        ]),
                        id
                    );
                }
            }
            _ => {}
        }
    }
}


pub fn editing_todo(
    todos:View<Todo>, 
    mut todo_editing:LocalViewMut<Editing>, 
    dirty_editing:View<DirtyEditing>,
    mut entities:EntitiesViewMut, 
    doc:DocumentView,
    world:WorldView,
) {
    for (id, (todo, _)) in (&todos, &dirty_editing).iter().with_id() {
        if todo.editing {
            let elem:HtmlInputElement = selector::todo_edit(&doc, id);
            entities.add_component(
                &mut *todo_editing,
                Editing {
                    on_blur: EventListener::new(&elem, "blur", {
                        let world = world.clone();
                        move |event| {
                            handlers::todo_finish_editing(world.clone(), id)
                        }
                    }),
                    on_keydown: EventListener::new(&elem, "keydown", {
                        let world = world.clone();
                        let elem = elem.clone();
                        move |event| {
                            handlers::todo_edit_keydown(world.clone(), id, &elem, event.dyn_ref().unwrap_throw());
                        }
                    }),
                }, 
                id
            );
        } else {
            todo_editing.delete(id);
        }
    }
}

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
