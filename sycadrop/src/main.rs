// https://htmldom.dev/make-a-draggable-element/

use gloo::{console::log, utils::document};
use sycamore::prelude::*;
use wasm_bindgen::*;
use web_sys::{Event, HtmlElement, MouseEvent};

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            p { "Hello, World!" }
                ContainerWidget()
        }
    });
}

#[component]
fn ContainerWidget<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="container") {
             DropZone{}

        }
    }
}

#[component]
fn DraggableItem<G: Html>(cx: Scope) -> View<G> {
    let node_ref = create_node_ref(cx);

    let handle_dragstart = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        // dom.add_class("hide");
        dom.set_attribute("style", "opacity: 0.2");
    };

    let handle_dragend = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.set_attribute("style", "opacity: 1");
    };

    view! { cx,
        div(ref=node_ref, draggable=true, class="item", on:dragstart=handle_dragstart, on:dragend=handle_dragend) {
             //ItemWidget{}

        }
    }
}

// basic test

#[component]
fn DropZone<G: Html>(cx: Scope) -> View<G> {
    let node_ref = create_node_ref(cx);

    let handle_dragstart = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.add_class("hide");
    };

    let handle_dragend = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.set_attribute("style", "opacity: 0.2");
    };

    view! { cx,
        div(ref=node_ref, class="box") {
             DraggableItem{}

        }
    }
}

#[component]
fn ItemWidget<G: Html>(cx: Scope) -> View<G> {
    let x = create_signal(cx, 0);
    let y = create_signal(cx, 0);
    let node_ref = create_node_ref(cx);

    let handle_dragstart = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.set_attribute("style", "opacity: 0.4");
    };

    let handle_dragend = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.set_attribute("style", "opacity: 1");
    };

    let mousemove_handler = |e: Event| {
        log!("mouse_move");
        let e: MouseEvent = e.unchecked_into();

        // How far the mouse has been moved
        let dx = e.client_x() - *x.get();
        let dy = e.client_y() - *x.get();

        // Set the position of element
        // let dom = node_ref.get::<DomNode>();
        let dom: HtmlElement = document()
            .get_element_by_id("dragMe")
            .unwrap()
            .unchecked_into();

        let top = dom.offset_top() + dy;
        let left = dom.offset_left() + dx;
        dom.set_attribute(
            "style",
            format!("top: {}px; left: {}px", top, left).as_str(),
        );

        // Reassign the position of mouse
        x.set(e.client_x());
        y.set(e.client_y());
    };

    let mouseup_handler = |e: Event| {
        // let dom = node_ref.get::<DomNode>();
        // How to remove event ?

        // let dom: HtmlElement = document()
        //     .get_element_by_id("dragMe")
        //     .unwrap()
        //     .unchecked_into();
        // dom.remove_event_listener_with_callback("mousemove", mousemove_handler);
        // dom.remove_event_listener_with_callback("mouseup", mouseup_handler);

        //dom.remove_event(cx, "mousemove", mousemove_handler);
        //dom.remove_event(cx, "mouseup", mouseup_handler);
        let dom = node_ref.get::<DomNode>();
    };

    let mousedown_handler = move |e: Event| {
        let e: MouseEvent = e.unchecked_into();
        x.set(e.client_x());
        y.set(e.client_y());

        let dom = node_ref.get::<DomNode>();
        dom.event(cx, "mousemove", mousemove_handler);
        dom.event(cx, "mouseup", mouseup_handler);
    };

    view! {cx,
        div(ref=node_ref, draggable=true, id="dragMe", class="box",
            on:mousedown=mousedown_handler, on:dragstart=handle_dragstart,
        ){
            "Drag   me"
        }
    }
}
