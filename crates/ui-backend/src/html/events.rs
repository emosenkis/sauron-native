//! https://developer.mozilla.org/en-US/docs/Web/Events
use virtual_dom::builder::on;
use virtual_dom::builder::Attribute;
use virtual_dom::{Callback, Value};

macro_rules! declare_events {
    ( $(
         $(#[$attr:meta])*
         $name:ident => $event:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<'a, F>(f: F) -> Attribute<'a>
                where F: Into<Callback<Value>>
                {
                    on(stringify!($event), f)
                }
         )*
    }
}

// Mouse events
declare_events! {
    onauxclick => auxclick;
    onclick  => click;
    oncontextmenu =>contextmenu;
    ondblclick  => dblclick;
    onmousedown => mousedown;
    onmouseenter => mouseenter;
    onmouseleave => mouseleave;
    onmousemove => mousemove;
    onmouseover => mouseover;
    onmouseout => mouseout;
    onmouseup => mouseup;
    onpointerlockchange => pointerlockchange;
    onpointerlockerror => pointerlockerror;
    onselect => select;
    onwheel => wheel;
}

// keyboard events
declare_events! {
    onkeydown => keydown;
    onkeypress => keypress;
    onkeyup => keyup;
}

// focus events
declare_events! {
    onfocus => focus;
    onblur => blur;
}

// form events
declare_events! {
    onreset => reset;
    onsubmit => submit;
}

declare_events! {
    onbroadcast => broadcast;
    //CheckboxStateChange
    onhashchange => hashchange;
    oninput => input;
    //RadioStateChange
    onreadystatechange => readystatechange;
    //ValueChange
}
