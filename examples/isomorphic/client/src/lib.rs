//#![deny(warnings)]
#![deny(clippy::all)]
use browser::html::attributes::*;
use browser::html::events::*;
use browser::html::*;
use browser::*;
use console_error_panic_hook;
use js_sys::{Array, Date};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Mutex;
use vdom::builder::*;
use vdom::Event;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::console;
use web_sys::{Document, Element, Window};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use lazy_static::lazy_static;

#[wasm_bindgen]
pub struct Client {
    app: App,
    dom_updater: DomUpdater,
}

pub struct State {
    click_count: u32,
    listeners: Vec<Box<(Fn() -> () + 'static)>>,
}

pub struct App {
    pub store: Rc<RefCell<State>>,
}

#[derive(Debug)]
pub enum Msg {
    Click,
}

// Expose globals from JS for things such as request animation frame
// that web sys doesn't seem to have yet
//
// TODO: Remove this and use RAF from Rust
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.request_animation_frame
#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;

    pub static global_js: GlobalJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();
        console::log_1(&format!("What to do with this initial state: {}", initial_state).into());

        let root_node = document()
            .get_element_by_id("isomorphic-rust-web-app")
            .unwrap();

        let app = App::new(1);

        app.store.borrow_mut().subscribe(Box::new(|| {
            web_sys::console::log_1(&"Updating state".into());
            global_js.update();
        }));

        setup_clock();
        let dom_updater = DomUpdater::new_replace_mount(app.view(), root_node);
        let client = Client { app, dom_updater };
        client
    }

    pub fn render(&mut self) {
        console::log_1(&"in render function".into());
        let vdom = self.app.view();
        self.dom_updater.update(vdom);
    }
}

impl State {
    pub fn new(count: u32) -> State {
        State {
            click_count: count,
            listeners: vec![],
        }
    }

    pub fn subscribe(&mut self, callback: Box<Fn() -> ()>) {
        self.listeners.push(callback)
    }

    pub fn msg(&mut self, msg: &Msg) {
        console::log_1(&format!("Msg got here: {:?}", msg).into());
        match msg {
            Msg::Click => self.increment_click(),
        };

        // Whenever we update state we'll let all of our state listeners know that state was
        // updated
        for callback in self.listeners.iter() {
            console::log_1(&"Calling callback...".into());
            callback();
        }
    }

    pub fn click_count(&self) -> u32 {
        self.click_count
    }

    fn increment_click(&mut self) {
        self.click_count += 1;
    }
}

impl App {
    fn new(count: u32) -> App {
        let mut state = State::new(count);
        state.subscribe(Box::new(|| {
            web_sys::console::log_1(&"Updating state".into());
        }));
        let store = Rc::new(RefCell::new(state));

        App { store }
    }

    fn view(&self) -> vdom::Node {
        let store_clone = Rc::clone(&self.store);
        let count: u32 = self.store.borrow().click_count();
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                div([], [text(format!("Hello world! {}", count))]),
                div([id("current-time")], []),
                div(
                    [],
                    [button(
                        [onclick(move |v: Event| {
                            console::log_1(
                                &format!("I've been clicked and the value is: {:#?}", v).into(),
                            );
                            store_clone.borrow_mut().msg(&Msg::Click);
                        })],
                        [text("Click me!")],
                    )],
                ),
                div(
                    [],
                    [
                        text("Using oninput"),
                        input(
                            [
                                r#type("text"),
                                oninput(|v: Event| {
                                    console::log_1(&format!("input has input: {:#?}", v).into());
                                }),
                                placeholder("Type here..."),
                            ],
                            [],
                        ),
                    ],
                ),
                div(
                    [],
                    [
                        text("using oninput on a textarea"),
                        textarea(
                            [
                                oninput(|v: Event| {
                                    console::log_1(
                                        &format!("textarea has changed: {:#?}", v).into(),
                                    );
                                }),
                                placeholder("Description here..."),
                            ],
                            [],
                        ),
                    ],
                ),
                div(
                    [],
                    [
                        text("Using onchange"),
                        input(
                            [
                                r#type("text"),
                                onchange(|v: Event| {
                                    console::log_1(&format!("input has changed: {:#?}", v).into());
                                }),
                                placeholder("Description here..."),
                            ],
                            [],
                        ),
                    ],
                ),
            ],
        )
    }
}

// Set up a clock on our page and update it each second to ensure it's got
// an accurate date.
//
// Note the usage of `Closure` here because the closure is "long lived",
// basically meaning it has to persist beyond the call to this one function.
// Also of note here is the `.as_ref().unchecked_ref()` chain, which is who
// you can extract `&Function`, what `web-sys` expects, from a `Closure`
// which only hands you `&JsValue` via `AsRef`.
fn setup_clock() -> Result<(), JsValue> {
    //update_time(&current_time);
    let a = Closure::wrap(Box::new(move || update_time()) as Box<dyn Fn()>);
    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
    fn update_time() {
        let current_time = document()
            .get_element_by_id("current-time")
            .expect("should have #current-time on the page");
        current_time.set_inner_html(&String::from(
            Date::new_0().to_locale_string("en-GB", &JsValue::undefined()),
        ));
    }

    // The instances of `Closure` that we created will invalidate their
    // corresponding JS callback whenever they're dropped, so if we were to
    // normally return from `run` then both of our registered closures will
    // raise exceptions when invoked.
    //
    // Normally we'd store these handles to later get dropped at an appropriate
    // time but for now we want these to be global handlers so we use the
    // `forget` method to drop them without invalidating the closure. Note that
    // this is leaking memory in Rust, so this should be done judiciously!
    a.forget();

    Ok(())
}
