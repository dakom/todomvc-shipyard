use simple_html_template::{TemplateCache, html_map};
use shipyard::EntityId;
use wasm_bindgen::prelude::*;
use web_sys::DocumentFragment;
use crate::dom::entity_id;

pub const BODY:&'static str = "body";
pub const FOOTER:&'static str = "footer";
pub const TODO_ITEM:&'static str = "todo-item";

pub struct TemplateManager {
    pub cache: TemplateCache
}

impl TemplateManager {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (BODY, include_str!("../templates/body.html")),
            (FOOTER, include_str!("../templates/footer.html")),
            (TODO_ITEM, include_str!("../templates/todo-item.html")),
        ]);

        Self { cache }
    }

    pub fn body(&self) -> DocumentFragment {
        self.cache.render_dom_plain(BODY)
    }

    pub fn footer(&self) -> DocumentFragment {
        self.cache.render_dom_plain(FOOTER)
    }

    pub fn todo_item(&self, label:&str, id:EntityId) -> DocumentFragment {
        let data = html_map! {
            "label" => label,
            "classnames" => "",
            "checked" => "",
            "id" => &entity_id(id), 
        };

        self.cache.render_dom(TODO_ITEM, &data).unwrap_throw()
    }
}
