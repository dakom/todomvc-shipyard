use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use derive_more::IntoIterator;
use gloo_events::EventListener;
use web_sys::HtmlInputElement;
use std::collections::VecDeque;
use std::rc::Rc;
use shipyard::*;
use crate::templates::TemplateManager;

//Uniques
pub struct MainInput {
    pub elem: HtmlInputElement,
    pub on_keydown: EventListener
}

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

pub struct Order {
    pub list: VecDeque<EntityId>,
    pub pending_render: Option<ListChange>,
}

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

//Multi components
#[derive(Serialize, Deserialize)]
pub struct Todo{
    pub label: String,
    pub editing: bool,
    pub completed: bool,
}

impl Todo {
    pub fn new(label:String) -> Self {
        Self {
            label,
            editing: false,
            completed: false,
        }
    }
}

pub struct EventListeners(pub Vec<EventListener>);

pub struct Editing{
    pub on_blur: EventListener,
    pub on_keydown: EventListener,
}


//Multi components

pub struct DirtyToggle {}
pub struct DirtyEditing{}
pub struct DirtyLabel{}

//Helpers for view access
pub type WorldView<'a> = NonSendSync<UniqueView<'a, Rc<World>>>;
pub type DocumentView<'a> = NonSendSync<UniqueView<'a, web_sys::Document>>;
pub type DomRootView<'a> = NonSendSync<UniqueView<'a, DomRoot>>;

pub type TemplateManagerView<'a> = NonSendSync<UniqueView<'a, TemplateManager>>;

pub type OrderView<'a> = UniqueView<'a, Order>;
pub type OrderViewMut<'a> = UniqueViewMut<'a, Order>;


pub type LocalViewMut<'a, T> = NonSendSync<ViewMut<'a, T>>;

//Dependencies
pub enum ListChange{
    Remove(EntityId),
    Swap(EntityId, EntityId),
    RemoveMulti(Vec<EntityId>),
}
