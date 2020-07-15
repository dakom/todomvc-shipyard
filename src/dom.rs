pub mod selector;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::components::DocumentView;
use web_sys::{HtmlElement, Element, DocumentFragment, HtmlCollection, Document};

pub fn entity_id(id:shipyard::EntityId) -> String {
    //gotta start with a letter and avoid special characters
    format!("e{}-{}", id.index(), id.gen())
}

pub fn select(doc:&Document, query:&str) -> Option<Element> {
    doc.query_selector(query).unwrap_throw()
}

// gets the parent element via parent id
pub fn append_to_id(doc:&Document, parent_id:&str, fragment:DocumentFragment) {
    let parent:Element = get_element_by_id(&doc, parent_id).unwrap_throw();
    parent.append_child(&fragment).unwrap_throw();
}
// gets the parent element via parent id
pub fn prepend_to_id(doc:&Document, parent_id:&str, fragment:DocumentFragment) {
    let parent:Element = get_element_by_id(&doc, parent_id).unwrap_throw();
    parent.prepend_with_node_1(&fragment).unwrap_throw();
}

pub fn set_styles_by_id(doc:&Document, id:&str, styles:&[(&str, &str)]) {
    if let Some(elem) = get_element_by_id::<HtmlElement>(doc, id) {
        for (style, value) in styles.iter() {
            elem.style()
                .set_property(style, value)
                .unwrap_throw();
        }
    }
}

pub fn set_style_by_id(doc:&Document, id:&str, style:&str, value:&str) {
    if let Some(elem) = get_element_by_id::<HtmlElement>(doc, id) {
        elem.style()
            .set_property(style, value)
            .unwrap_throw();
    }
}

pub fn set_styles_by_class(parent:&Element, class_names:&str, styles:&[(&str, &str)]) {
    for elem in get_elements_by_class::<HtmlElement>(parent, class_names) {
        for (style, value) in styles.iter() {
            elem.style()
                .set_property(style, value)
                .unwrap_throw();
        }
    }
}

pub fn set_style_by_class(parent:&Element, class_names:&str, style:&str, value:&str) {
    for elem in get_elements_by_class::<HtmlElement>(parent, class_names) {
        elem.style()
            .set_property(style, value)
            .unwrap_throw();
    }
}

pub fn get_elements_by_class<T: JsCast>(parent:&Element, class_names:&str) -> Vec<T> {
    let col:HtmlCollection = parent.get_elements_by_class_name(class_names);
    let res = html_collection_to_vec(&col);
    res
}

pub fn get_element_by_id<T: JsCast>(doc:&Document, id:&str) -> Option<T> {
    doc.get_element_by_id(id)
        .map(|elem| elem.dyn_into().unwrap_throw())
}

pub fn with_element_id<T: JsCast, F: FnOnce(T)> (doc:&Document, id:&str, f:F) {
    if let Some(elem) = doc.get_element_by_id(id) {
        f(elem.dyn_into().unwrap_throw());
    }
}

pub fn html_collection_to_vec<T: JsCast>(col:&HtmlCollection) -> Vec<T> {
    let mut res:Vec<T> = Vec::with_capacity(col.length() as usize);

    for i in 0..col.length() {
        if let Some(elem) = col.item(i) {
            res.push(elem.dyn_into().unwrap_throw());
        } 
    }

    res
}
