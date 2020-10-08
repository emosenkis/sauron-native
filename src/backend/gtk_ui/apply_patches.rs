use super::convert_widget;
use super::Dispatch;
use crate::{
    widget::attribute::util::is_scrollable, AttribKey, Attribute, Node, Patch,
};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gtk::{
    prelude::*, Button, Container, ContainerExt, EventBox, Image, Label,
    MenuItem, Overlay, TextView, Widget,
};
use mt_dom::patch::{AddAttributes, AppendChildren, RemoveNode, ReplaceNode};
use std::{collections::HashMap, fmt::Debug};

pub fn apply_patches<MSG, DSP>(
    program: &DSP,
    node: &Node<MSG>,
    root_container: &Container,
    patches: &Vec<Patch<MSG>>,
) where
    MSG: Debug + 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    let nodes_to_patch = find_nodes(node, root_container, patches);

    for patch in patches {
        let patch_node_idx = patch.node_idx();
        let widget = nodes_to_patch
            .get(&patch_node_idx)
            .expect("must have a node to patch");
        match patch {
            Patch::AddAttributes(AddAttributes {
                tag,
                node_idx: _,
                new_node_idx: _,
                attrs,
            }) => {
                set_widget_attributes(tag, widget, attrs);
            }
            Patch::AppendChildren(AppendChildren {
                tag,
                node_idx: _,
                children,
            }) => {
                match tag {
                    crate::Widget::Overlay => {
                        let overlay = widget
                            .downcast_ref::<Overlay>()
                            .expect("must be an overlay");
                        for (child_idx, child) in children {
                            if let Some(element) = child.as_element_ref() {
                                let child =
                                    convert_widget::from_node(program, element);
                                let widget = child
                                    .as_widget()
                                    .expect("must be a widget");
                                //Note: overlay have different behavior when adding child widget
                                overlay.add_overlay(widget);
                                overlay.set_child_index(widget, -1);
                                widget.show();
                                overlay.show_all();
                            }
                        }
                    }

                    _ => {
                        let container = widget
                            .downcast_ref::<Container>()
                            .expect("must be a container");
                        for (child_idx, child) in children {
                            if let Some(element) = child.as_element_ref() {
                                let child = convert_widget::from_node(
                                    program, &element,
                                );
                                let widget = child
                                    .as_widget()
                                    .expect("must be a widget");
                                //Note: overlay have different behavior when adding child widget
                                container.add(widget);
                                widget.show();
                            }
                        }
                    }
                }
            }
            Patch::RemoveNode(RemoveNode {
                tag: _,
                node_idx: _,
            }) => {
                let parent = widget.get_parent().expect("must have a parent");
                if let Some(container) = parent.downcast_ref::<Container>() {
                    container.remove(widget);
                }
            }
            Patch::ReplaceNode(ReplaceNode {
                tag: _,
                node_idx: _,
                new_node_idx: _,
                replacement,
            }) => {
                root_container.remove(widget);
                if let Some(new_element) = replacement.as_element_ref() {
                    let new_widget =
                        convert_widget::from_node(program, new_element);
                    let new_widget =
                        new_widget.as_widget().expect("must be a widget");
                    root_container.add(new_widget);
                    new_widget.show();
                }
            }
            _ => {
                println!("container: {:?}", root_container);
                println!("todo for: {:?}", patch);
            }
        }
    }
}

fn set_widget_attributes<MSG: 'static>(
    tag: &crate::Widget,
    widget: &Widget,
    attrs: &[&Attribute<MSG>],
) {
    match tag {
        crate::Widget::Button => {
            let button =
                widget.downcast_ref::<Button>().expect("must be a button");
            for att in attrs {
                for value in att.get_plain() {
                    match att.name() {
                        AttribKey::Label => {
                            button.set_label(&value.to_string())
                        }
                        _ => (),
                    }
                }
            }
        }
        crate::Widget::TextArea => {
            let text_view =
                widget.downcast_ref::<TextView>().unwrap_or_else(|| {
                    panic!("must be a text_view, found: {:?}", widget)
                });
            for att in attrs {
                for value in att.get_plain() {
                    match att.name() {
                        AttribKey::Value => {
                            if let Some(buffer) = text_view.get_buffer() {
                                buffer.set_text(&value.to_string());
                            }
                        }
                        AttribKey::Editable => {
                            let editable = value.as_bool();
                            text_view.set_editable(editable);
                        }
                        _ => (),
                    }
                }
            }
        }
        crate::Widget::Svg => {
            let image = widget
                .downcast_ref::<Image>()
                .unwrap_or_else(|| panic!("must be an image {:?}", widget));
            for att in attrs {
                for value in att.get_plain() {
                    match att.name() {
                        AttribKey::Data => {
                            if let Some(bytes) = value.as_bytes() {
                                let pixbuf_loader =
                                    PixbufLoader::new_with_mime_type(
                                        "image/svg+xml",
                                    )
                                    .expect("error loader");
                                pixbuf_loader
                                    .write(bytes)
                                    .expect("Unable to write svg data into pixbuf_loader");
                                pixbuf_loader
                                    .close()
                                    .expect("error creating pixbuf");
                                let pixbuf = pixbuf_loader.get_pixbuf();
                                image.set_from_pixbuf(Some(
                                    &pixbuf.expect("error in pixbuf_loader"),
                                ));
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        crate::Widget::Label => {
            let event_box =
                widget.downcast_ref::<EventBox>().unwrap_or_else(|| {
                    panic!("must be an eventbox, found: {:?}", widget)
                });
            let event_box_children = event_box.get_children();
            let child1 =
                event_box_children.get(0).expect("must have one child");
            let label = child1.downcast_ref::<Label>().unwrap_or_else(|| {
                panic!("must be a label, found: {:?}", widget)
            });
            for att in attrs {
                for value in att.get_plain() {
                    match att.name() {
                        AttribKey::Value => label.set_text(&value.to_string()),
                        _ => (),
                    }
                }
            }
        }
        _ => {
            println!("todo for other widgets");
        }
    }
}

fn find_nodes<MSG>(
    node: &Node<MSG>,
    container: &Container,
    patches: &[Patch<MSG>],
) -> HashMap<usize, Widget>
where
    MSG: 'static,
{
    let mut nodes_to_find: HashMap<usize, &crate::Widget> = HashMap::new();
    let mut cur_node_idx = 0;

    for patch in patches {
        let tag = patch.tag().expect("must have a tag");
        nodes_to_find.insert(patch.node_idx(), tag);
    }
    find_nodes_recursive(node, container, &mut cur_node_idx, &nodes_to_find)
}

fn find_nodes_recursive<MSG>(
    node: &Node<MSG>,
    container: &Container,
    cur_node_idx: &mut usize,
    nodes_to_find: &HashMap<usize, &crate::Widget>,
) -> HashMap<usize, Widget>
where
    MSG: 'static,
{
    let tag = node.tag().expect("must have a tag");
    let mut nodes_to_patch: HashMap<usize, Widget> = HashMap::new();

    if let Some(_) = nodes_to_find.get(cur_node_idx) {
        let container_widget: Widget = container.clone().upcast();
        nodes_to_patch.insert(*cur_node_idx, container_widget);
    }

    if tag.is_container() {
        let node_children = node.get_children().expect("must have children");

        let attrs = node.get_attributes().expect("must have have attributes");
        let widget_children = get_widget_children(tag, container, &attrs);

        assert_eq!(
            node_children.len(),
            widget_children.len(),
            "must have the same children len, in widget: {:?}",
            tag
        );
        for (child_node, widget_child) in
            node_children.iter().zip(widget_children.iter())
        {
            *cur_node_idx += 1;
            let child_tag = child_node.tag().expect("must have a child tag");
            if let Some(_patch_tag) = nodes_to_find.get(&cur_node_idx) {
                let child_attrs =
                    child_node.get_attributes().expect("must have attributes");
                let widget: Widget = get_actual_node_to_patch(
                    child_tag,
                    widget_child,
                    &child_attrs,
                );
                nodes_to_patch.insert(*cur_node_idx, widget);
            }
            if child_tag.is_container() {
                if let Some(container) =
                    widget_child.downcast_ref::<Container>()
                {
                    let child_nodes_to_patch = find_nodes_recursive(
                        child_node,
                        container,
                        cur_node_idx,
                        nodes_to_find,
                    );
                    nodes_to_patch.extend(child_nodes_to_patch);
                }
            }
        }
    }
    nodes_to_patch
}

/// return the actual node to be patched
/// dealing with widgets that is wrapped with scrolled window
fn get_actual_node_to_patch<MSG>(
    child_tag: &crate::Widget,
    widget_child: &Widget,
    attrs: &[crate::Attribute<MSG>],
) -> Widget
where
    MSG: 'static,
{
    match child_tag {
        crate::Widget::TextArea => {
            if is_scrollable(&attrs) {
                // ScrolledWindow -> TextArea
                let scrolled_window = widget_child
                    .downcast_ref::<gtk::ScrolledWindow>()
                    .expect("must be a scrolled window container");
                let scrolled_window_children = scrolled_window.get_children();
                let text_area =
                    scrolled_window_children.get(0).expect("must have a child");
                let text_area: Widget = text_area.clone().upcast();
                text_area
            } else {
                let text_area: Widget = widget_child.clone().upcast();
                text_area
            }
        }
        crate::Widget::Svg => {
            if is_scrollable(&attrs) {
                // ScrolledWindow -> ViewPort -> Image
                let scrolled_window = widget_child
                    .downcast_ref::<gtk::ScrolledWindow>()
                    .expect("must be a scrolled window container");

                let scrolled_window_children = scrolled_window.get_children();
                let view_port = scrolled_window_children
                    .get(0)
                    .expect("scrolled window must have a child");
                let view_port = view_port
                    .downcast_ref::<gtk::Viewport>()
                    .expect("must be a viewport container");

                let view_port_children = view_port.get_children();
                let svg_image = view_port_children
                    .get(0)
                    .expect("view port must have svg image as child");
                let svg_image: Widget = svg_image.clone().upcast();
                svg_image
            } else {
                let svg_image: Widget = widget_child.clone().upcast();
                svg_image
            }
        }
        _ => {
            let widget: Widget = widget_child.clone().upcast();
            widget
        }
    }
}

/// return the children of this widget
/// dealing with special case where widgets are wrapped with scrolled window
fn get_widget_children<MSG>(
    tag: &crate::Widget,
    container: &Container,
    attrs: &[crate::Attribute<MSG>],
) -> Vec<Widget>
where
    MSG: 'static,
{
    match *tag {
        // special case for GroupBox since GroupBox have a frame wrapper
        // GroupBox(Frame(Box))
        crate::Widget::GroupBox => {
            let frame_children = container.get_children();
            let gbox_widget =
                frame_children.get(0).expect("must have one child");
            let gbox = gbox_widget
                .downcast_ref::<gtk::Box>()
                .expect("must be a container");
            gbox.get_children()
        }
        crate::Widget::Vbox | crate::Widget::Hbox => {
            if is_scrollable(attrs) {
                println!("VBOX is SCROLLABLE..");
                container.downcast_ref::<gtk::ScrolledWindow>()
                            .unwrap_or_else(|| {
                                panic!(
                                    "container must be a ScrolledWindow, but found: {:?}",
                                    container
                                )
                            });

                // ScrolledWindow -> VBox
                let scrolled_children = container.get_children();
                assert_eq!(
                    scrolled_children.len(),
                    1,
                    "There should only be one children.. that is the real vbox"
                );
                let scrolled_widget =
                    scrolled_children.get(0).expect("must have one child");
                let view_port = scrolled_widget
                    .downcast_ref::<gtk::Viewport>()
                    .unwrap_or_else(|| {
                        panic!(
                            "must be a view port, but found: {:?}",
                            scrolled_widget
                        )
                    });
                let viewport_children = view_port.get_children();
                let box_widget =
                    viewport_children.get(0).expect("must have 1 child");

                let box_container = box_widget
                    .downcast_ref::<gtk::Box>()
                    .unwrap_or_else(|| {
                        panic!(
                            "must be a gtk::Box.. but found: {:?}",
                            box_widget,
                        )
                    });
                box_container.get_children()
            } else {
                container.get_children()
            }
        }
        // special case for SubMenu in MenuItem since
        // sub_menu is not returned as children for the menu_item
        // instead, we get the sub menu as part of it's children
        //
        // WARNING: The submenu should be at the last part of the menu_item
        // so as to match the NodeIdx arrangement when applying patches
        crate::Widget::MenuItem => {
            let mut widgets = container.get_children();
            let menu_item = container
                .downcast_ref::<MenuItem>()
                .expect("must be castable to menu item");
            if let Some(sub_menu) = menu_item.get_submenu() {
                widgets.push(sub_menu);
            }
            widgets
        }
        _ => container.get_children(),
    }
}
