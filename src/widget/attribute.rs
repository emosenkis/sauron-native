//! Provides functions for attributes of sauron native widgets
//!
use crate::Attribute;
pub use event::Event;
use mt_dom::{attr, on};
use std::fmt;
pub use util::{find_callback, find_value};
pub use value::Value;

pub mod event;
pub mod util;
mod value;

/// declare an attribute to be used as a function call
macro_rules! declare_attr {
    (
        $(
            $(#[$attr:meta])*
            $fname:ident => $att_key:tt;
        )*
    ) => {

        $(
            /// creates an attribute with the function name as the attribute key
            $(#[$attr])*
            pub fn $fname<V,MSG>(v: V) -> Attribute<MSG>
                where V:Into<Value>,
            {
                attr(AttribKey::$att_key, v.into())
            }
        )*
    }
}

/// These are attribute keys used in sauron-native, which will be translated to their
/// corresponding backends
#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum AttribKey {
    /// String, used in text_input
    Value,
    /// String, used in button, label, checkbox, radio
    Label,
    /// bool, used in checkbox, radio
    Checked,
    /// Alignment Enum, used in hbox and vbox
    Alignment,
    /// If the key differ in the diff, all of the subtree will be discarded
    Key,
    /// whether or not the control is editable, used in text_view
    Editable,
    /// data, used in image blobs and svg
    Data,
    /// svg image data attribute used in button
    SvgImage,
    /// The style attribute
    Style,
    /// Events
    ClickEvent,
    /// Mouse down event
    MouseDown,
    /// Mouse up event
    MouseUp,
    /// Mouse move event
    MouseMove,
    /// Input event
    InputEvent,
    /// whether or not a widget is scrollable, such as image, text_area
    Scrollable,
}

declare_attr! {
    /// value attribute, used in text_input, textarea
    value => Value;
    /// data attribute, used in image, svg
    data => Data;
    /// label attribute, used in button, checkbox and radio
    label => Label;
    /// height attribute, used in most widgets
    /// svg_image attribute, used in buttons
    svg_image => SvgImage;
    /// editable attribute
    editable => Editable;
    /// scrollable attribute
    scrollable => Scrollable;
}

impl fmt::Display for AttribKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
