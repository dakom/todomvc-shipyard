use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use derive_more::IntoIterator;
use gloo_events::EventListener;
use web_sys::HtmlInputElement;
use std::collections::VecDeque;
use shipyard::*;
use crate::templates::TemplateManager;

//Uniques
pub struct MainInput {
    pub elem: HtmlInputElement,
    pub on_keypress: EventListener
}

pub struct DomRoot(pub web_sys::Element);

#[derive(IntoIterator)]
pub struct Order (pub VecDeque<EntityId>);

//Multi components
#[derive(Serialize, Deserialize)]
pub struct Todo{
    pub label: String,
    pub editing: bool,
    pub status: Status,
}

impl Todo {
    pub fn new(label:String) -> Self {
        Self {
            label,
            editing: false,
            status: Status::Active,
        }
    }
}

pub struct EventListeners(pub Vec<EventListener>);

#[derive(Serialize, Deserialize)]
pub struct StatusFilter {
    pub status: Option<Status>
}

//Helpers for view access
pub type DocumentView<'a> = NonSendSync<UniqueView<'a, web_sys::Document>>;
pub type DomRootView<'a> = NonSendSync<UniqueView<'a, DomRoot>>;

pub type TemplateManagerView<'a> = NonSendSync<UniqueView<'a, TemplateManager>>;

pub type OrderView<'a> = UniqueView<'a, Order>;
pub type OrderViewMut<'a> = UniqueViewMut<'a, Order>;

pub type EventListenersView<'a> = NonSendSync<View<'a, EventListeners>>;
pub type EventListenersViewMut<'a> = NonSendSync<ViewMut<'a, EventListeners>>;
//Component dependencies
#[derive(Serialize, Deserialize, PartialEq)]
pub enum Status {
    Active,
    Completed
}
