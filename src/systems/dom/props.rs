use shipyard::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use crate::{
    components::*,
    dom,
    dom::selector,
};


pub fn main_visible(root:DomRootView, order: UniqueView<Order>) {

    let display = if order.is_empty() { "none" } else { "block" };

    dom::set_children_with_class_style(&root.0, "main", "display", display);
    dom::set_children_with_class_style(&root.0, "footer", "display", display);
}

pub fn labels (
    todos:View<Todo>,
    dirty_labels:View<DirtyLabel>,
    doc:DocumentView,
) {
    for (id, (todo, _)) in (&todos, &dirty_labels).iter().with_id() {
        let elem:HtmlElement = selector::todo_label(&doc, id);
        elem.set_inner_text(&todo.label);
    }
}

pub fn toggles(
    todos:View<Todo>,
    dirty_toggles:View<DirtyToggle>,
    doc:DocumentView,
) {
    for (id, (todo, _)) in (&todos, &dirty_toggles).iter().with_id() {
        let elem:HtmlElement = selector::todo(&doc, id);
        let toggle_elem:HtmlInputElement = selector::todo_toggle(&doc, id);

        let class_list = elem.class_list();
        if todo.completed {
            class_list.add_1("completed").unwrap_throw();
            toggle_elem.set_checked(true);
        } else {
            class_list.remove_1("completed").unwrap_throw();
            toggle_elem.set_checked(false);
        }
    }
}

pub fn editing (
    todos:View<Todo>,
    dirty_editing:View<DirtyEditing>,
    doc:DocumentView,
) {
    for (id, (todo, _)) in (&todos, &dirty_editing).iter().with_id() {
        let elem:HtmlElement = selector::todo(&doc, id);
        let edit_elem:HtmlInputElement = selector::todo_edit(&doc, id);
        
        dom::toggle_class(&elem, "editing", todo.editing);
        if todo.editing {
            edit_elem.set_value(&todo.label);
            edit_elem.focus();
        }
    }
}

pub fn toggle_all(todos:View<Todo>, doc:DocumentView) {
    let all_completed = todos.iter().into_iter().all(|todo| todo.completed);
    
    let elem:HtmlInputElement = selector::toggle_all(&doc);

    elem.set_checked(all_completed);
}

pub fn clear_completed(todos:View<Todo>,doc:DocumentView) {
    let any_completed = todos.iter().into_iter().any(|todo| todo.completed);
    let elem:HtmlElement = selector::clear_completed(&doc);
    dom::set_style(&elem, "display", {
        if any_completed {
            "block"
        } else {
            "none"
        }
    });
}

pub fn count(todos:View<Todo>, doc:DocumentView) {
    let n_active = todos.iter().fold(0, |acc, todo| {
        if todo.completed {
            acc
        } else {
            acc+1
        }
    });

    let num_elem:HtmlElement = selector::count_num(&doc);
    let label_elem:HtmlElement = selector::count_label(&doc);

    num_elem.set_inner_text(&format!("{}", n_active));
    if n_active == 1 {
        label_elem.set_inner_text("item left")
    } else {
        label_elem.set_inner_text("items left")
    }
}

pub fn filter(todos:View<Todo>, bottom_filter:UniqueView<BottomFilter>, doc:DocumentView) {

    for (id, todo) in todos.iter().with_id() {
        let elem:HtmlElement = selector::todo(&doc, id);
        let status = bottom_filter.completed;

        dom::set_style(&elem, "display", {
            if status.is_none() || status == Some(todo.completed) {
                "block"
            } else {
                "none"
            }
        });
    }
}

pub fn filter_selection(bottom_filter:UniqueView<BottomFilter>, doc:DocumentView) {
    let all_elem = selector::bottom_filter(&doc, "all");
    let active_elem = selector::bottom_filter(&doc, "active");
    let completed_elem = selector::bottom_filter(&doc, "completed");

    let (all, active, completed) = match bottom_filter.completed {
        None => (true, false, false),
        Some(flag) => {
            if !flag {
                (false, true, false)
            } else {
                (false, false, true)
            }
        }
    };
    dom::toggle_class(&all_elem, "selected", all);
    dom::toggle_class(&active_elem, "selected", active);
    dom::toggle_class(&completed_elem, "selected", completed);
    //  
}
