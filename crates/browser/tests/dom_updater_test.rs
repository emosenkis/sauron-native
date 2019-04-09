use browser::dom::CreatedNode;
use browser::html::attributes::*;
use browser::html::events::*;
use browser::html::*;
use browser::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use web_sys::{console, Element, Event, EventTarget, InputEvent, MouseEvent, Node};

wasm_bindgen_test_configure!(run_in_browser);

// Verify that our DomUpdater's patch method works.
// We test a simple case here, since diff_patch.rs is responsible for testing more complex
// diffing and patching.
#[wasm_bindgen_test]
fn patches_dom() {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let vdom = div([], []);

    let mut dom_updater = DomUpdater::new(vdom);

    let new_vdom = div([id("patched")], []); //html! { <div id="patched"></div> };
    dom_updater.update(new_vdom);

    document
        .body()
        .unwrap()
        .append_child(&dom_updater.root_node());
    assert_eq!(document.query_selector("#patched").unwrap().is_some(), true);
}

// When you replace a DOM node with another DOM node we need to make sure that the closures
// from the new DOM node are stored by the DomUpdater otherwise they'll get dropped and
// won't work.
#[wasm_bindgen_test]
fn updates_active_closure_on_replace() {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let old = div([], []);
    let mut dom_updater = DomUpdater::new_append_to_mount(old, &body);

    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "update-active-closures-on-replace";

    let replace_node = input(
        [
            id(elem_id),
            oninput(move |event: vdom::Event| match event {
                vdom::Event::InputEvent(input) => {
                    *text_clone.borrow_mut() = input.value;
                }
                _ => unimplemented!(),
            }),
            value("End Text"),
        ],
        [],
    );

    // New node replaces old node.
    // We are testing that we've stored this new node's closures even though `new` will be dropped
    // at the end of this block.
    dom_updater.update(replace_node);

    let input_event = InputEvent::new("input").unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the oninput event our `text` should have a value of the input elements value.
    let input = document.get_element_by_id(&elem_id).unwrap();
    web_sys::EventTarget::from(input)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}