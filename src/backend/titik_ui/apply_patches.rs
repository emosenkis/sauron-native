use crate::AttribKey;
use crate::Attribute;
use crate::Patch;
use crate::Widget;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use titik::TextArea;

pub fn apply_patches<MSG, DSP>(
    program: &DSP,
    root_node: &mut Box<dyn titik::Widget<MSG>>,
    patches: &[Patch<MSG>],
) where
    MSG: Debug,
{
    for patch in patches {
        let patch_node_idx = patch.node_idx();
        let widget: &mut dyn titik::Widget<MSG> =
            titik::find_widget_mut(root_node.as_mut(), patch_node_idx)
                .expect("must have a node to patch");
        match patch {
            Patch::AddAttributes(tag, _node_idx, attrs) => {
                eprintln!("setting attributes...");
                set_widget_attributes::<MSG>(tag, widget, attrs);
            }
            // todo for other patches here.
            _ => println!("todo for: {:?}", patch),
        }
    }
}

fn set_widget_attributes<MSG: 'static>(
    tag: &crate::Widget,
    widget: &mut dyn titik::Widget<MSG>,
    attrs: &[Attribute<MSG>],
) {
    match tag {
        Widget::TextArea => {
            let text_area: &mut TextArea<MSG> = widget
                .as_any_mut()
                .downcast_mut()
                .expect("must be a textarea");
            for att in attrs {
                if let Some(value) = att.get_value() {
                    match att.name {
                        AttribKey::Value => {
                            text_area.set_value(&value.to_string());
                        }
                        _ => (),
                    }
                }
            }
        }
        //TODO for buttons here
        _ => (),
    }
}
