use wasm_bindgen::prelude::*;
use gloo_events::EventListener;
use web_sys::HtmlInputElement;
use std::collections::VecDeque;
use shipyard::*;
use derive_more::{Deref, DerefMut};

pub struct MainInput {
    pub on_keydown: EventListener
}

#[derive(Deref, DerefMut)]
pub struct DomRoot(pub web_sys::Element);

pub struct ToggleAll {
    pub on_click: EventListener,
}

pub struct ClearCompleted {
    pub on_click: EventListener,
}
pub struct Router {
    pub on_location: EventListener,
}

#[derive(Deref, DerefMut)]
pub struct Order(pub VecDeque<EntityId>);

#[derive(Deref, DerefMut)]
pub struct TodoListChange(pub Option<ListChange>);

pub struct BottomFilter {
    pub completed: Option<bool>,
}
impl BottomFilter {
    pub fn new() -> Self {
        let mut filter = Self {completed: None};
        filter.reset();
        filter
    }

    pub fn reset(&mut self) {
        let window = web_sys::window().unwrap_throw();
        match window.location().hash() {
            Err(_) => self.completed = None, 
            Ok(hash) => {
                if hash.len() > 2 {
                    let hash = &hash[2..];

                    match hash {
                        "active" => { self.completed = Some(false); },
                        "completed" => { self.completed = Some(true); },
                        _ => { self.completed =  None; }
                    }
                } else {
                    self.completed = None;
                }
            }
        }
    }
}


//Dependencies
pub enum ListChange{
    Append(EntityId),
    Remove(EntityId),
    Swap(EntityId, EntityId),
    RemoveMulti(Vec<EntityId>),
}
