use serde::{Serialize, Deserialize};
use gloo_events::EventListener;

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

