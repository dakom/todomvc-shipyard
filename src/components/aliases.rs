use std::rc::Rc;
use shipyard::*;
use crate::templates::TemplateManager;
use super::uniques::{DomRoot, Order};

pub type WorldView<'a> = NonSendSync<UniqueView<'a, Rc<World>>>;
pub type DocumentView<'a> = NonSendSync<UniqueView<'a, web_sys::Document>>;
pub type DomRootView<'a> = NonSendSync<UniqueView<'a, DomRoot>>;

pub type TemplateManagerView<'a> = NonSendSync<UniqueView<'a, TemplateManager>>;

pub type OrderView<'a> = UniqueView<'a, Order>;
pub type OrderViewMut<'a> = UniqueViewMut<'a, Order>;


pub type LocalViewMut<'a, T> = NonSendSync<ViewMut<'a, T>>;
