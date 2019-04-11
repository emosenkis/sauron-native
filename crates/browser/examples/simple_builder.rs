#![deny(warnings)]
use browser::html::attributes::*;
use browser::html::events::*;
use browser::html::*;
use vdom::diff;

fn main() {
    let old = div(
        [
            class("some-class"),
            id("some-id"),
            onclick(|_| {
                println!("clicked");
            }),
            attr("data-id", 1),
            on("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        [input([class("client"), r#type("checkbox")], [])],
    );
    let new = div(
        [
            class("some-class2"),
            id("some-id2"),
            onclick(|_| {
                println!("clicked2");
            }),
            attr("data-id", 2),
            on("mouseover", |_| {
                println!("i've been clicked2");
            }),
        ],
        [input([class("client"), r#type("checkbox")], [])],
    );
    println!("{}", old);
    println!("{}", new);
    let patches = diff(&old, &new);
    println!("patches: {:#?}", patches);
}
