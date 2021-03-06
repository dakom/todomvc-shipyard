use serde::{Serialize, Deserialize};
use gloo_events::EventListener;
use derive_more::{Deref, DerefMut};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo{
    pub label: String,
    pub editing: bool,
    pub completed: bool,
}

impl Todo {
    pub fn new(label:String, completed: bool) -> Self {
        Self {
            label,
            editing: false,
            completed,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct EventListeners(pub Vec<EventListener>);

pub struct Editing{
    pub on_blur: EventListener,
    pub on_keydown: EventListener,
}

