use crate::Callback;
use crate::{Element, Event, Node, Text, Value};
use std::convert::AsRef;

pub struct Attribute<'a> {
    name: &'a str,
    value: AttribValue,
}

pub enum AttribValue {
    Value(Value),
    Callback(Callback<Event>),
}

impl<V: Into<Value>> From<V> for AttribValue {
    fn from(v: V) -> Self {
        AttribValue::Value(v.into())
    }
}

impl From<Callback<Event>> for AttribValue {
    fn from(c: Callback<Event>) -> Self {
        AttribValue::Callback(c)
    }
}

impl Element {
    /// add the attribute values or events callback
    /// into this element
    pub fn add_attributes<'a, A>(mut self, attrs: A) -> Self
    where
        A: AsRef<[Attribute<'a>]>,
    {
        for a in attrs.as_ref() {
            match a.value {
                AttribValue::Value(ref v) => {
                    self.attrs.insert(a.name.to_string(), v.clone());
                }
                AttribValue::Callback(ref v) => {
                    self.events.insert(a.name.to_string(), v.clone());
                }
            }
        }
        self
    }

    pub fn add_children<C>(mut self, children: C) -> Self
    where
        C: AsRef<[Node]>,
    {
        for c in children.as_ref() {
            self.children.push(c.clone());
        }
        self
    }

    pub fn add_event_listener(mut self, event: &str, cb: Callback<Event>) -> Self {
        self.events.insert(event.to_string(), cb);
        self
    }
}

/// Create an element
///
///```
/// use vdom::builder::*;
/// fn main(){
///    let old = element(
///        "div",
///        [
///            attr("class", "some-class"),
///            attr("id", "some-id"),
///            on("click", |_| {
///                println!("clicked");
///            }),
///            attr("data-id", 1111),
///            on("mouseover", |_| {
///                println!("i've been clicked");
///            }),
///        ],
///        [element("div", [], [text("Hello world!")])],
///    );
/// }
///```
#[inline]
pub fn element<'a, A, C>(tag: &str, attrs: A, children: C) -> Node
where
    C: AsRef<[Node]>,
    A: AsRef<[Attribute<'a>]>,
{
    Node::Element(
        Element::new(tag)
            .add_children(children)
            .add_attributes(attrs),
    )
}

/// Create a textnode element
#[inline]
pub fn text<V>(v: V) -> Node
where
    V: Into<String>,
{
    Node::Text(Text { text: v.into() })
}

/// Create an attribute
#[inline]
pub fn attr<'a, V>(name: &'a str, v: V) -> Attribute<'a>
where
    V: Into<Value>,
{
    Attribute {
        name: name,
        value: v.into().into(),
    }
}

/// Attach a callback to an event
#[inline]
pub fn on<'a, C>(name: &'a str, c: C) -> Attribute<'a>
where
    C: Into<Callback<Event>>,
{
    Attribute {
        name: name,
        value: c.into().into(),
    }
}