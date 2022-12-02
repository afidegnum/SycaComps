// https://htmldom.dev/make-a-draggable-element/

use gloo::{console::log, utils::document};
use sycamore::generic_node::SycamoreElement;
use sycamore::prelude::*;
use wasm_bindgen::*;
use web_sys::{DataTransfer, DragEvent, Element, Event, HtmlElement, MouseEvent};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(|cx| {
        view! { cx,
            p { "Hello, World!" }
                ContainerWidget()
        }
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cat {
    id: &'static str,
    name: &'static str,
}

#[component]
fn ContainerWidget<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="container") {
             DropZone{}

        }
    }
}

// #[component]
#[component(inline_props)]
fn DraggableItem<G: Html>(cx: Scope, b: String) -> View<G> {
    let node_ref = create_node_ref(cx);
    let bb = create_signal(cx, b);
    let handle_dragstart = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        // dom.add_class("hide");
        let target = e.target().unwrap();
        log!(format!("Target --- {:?}", &target));
        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        // log!(format!("{:?}", &dom.inner_element()));
        if e.type_().contains("dragstart") {
            data_transf.set_effect_allowed("move");
            data_transf
                .set_data(
                    "text/html",
                    &dom.inner_element().unchecked_into::<Element>().inner_html(),
                )
                .unwrap();
            //let dt = data_transf.set_data("text/html", dom.inner_element().to_string());
            //log!(format!("{:?}", &dom.inner_element().as_string().unwrap()));
        }
        dom.set_attribute("style", "opacity: 0.2");

        log!(format!("{:?}", e.type_()));
    };

    let handle_dragend = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.set_attribute("style", "opacity: 1");
        log!(format!("{:?}", e.type_()));
    };
    let handle_dragenter = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.add_class("drag-over");
        log!(format!("{:?}", e.type_()));

        // dom.add_class("drag-over");
        //dom.set_attribute("style", "opacity: 0.2");
    };

    let handle_dragover = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        e.prevent_default();
        dom.add_class("drag-over");
        //dom.set_attribute("style", "opacity: 0.2");
        log!(format!("{:?}", e.type_()));
    };

    let handle_dragleave = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.remove_class("drag-over");
        log!(format!("{:?}", e.type_()));

        //dom.set_attribute("style", "opacity: 0.2");
    };

    let handle_drop = |e: Event| {
        let dom = node_ref.get::<DomNode>();

        // data_transfer_ref.get_data(format);
        //

        let data_transfer_ref: &web_sys::DataTransfer = e.unchecked_ref();

        // dom.dangerously_set_inner_html(data_transfer_ref.get_data("text/html").unwrap().as_str());
        // let drop_event = data_transfer_ref.clone();
        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        let data = data_transf.get_data("text/html").unwrap();
        log!(format!("{:?}", data.clone()));
        // dom.append_child(&G::te)
        log!(format!("{:?}", dom));
        let nd = &dom.inner_element().unchecked_into::<Element>();
        let elem = dom.inner_element();
        log!(format!("... {:?}", elem.));
        // dom.append_child(child)
        dom.dangerously_set_inner_html(data_transf.get_data("text/html").unwrap().as_str());
        let node = dom
            .inner_element()
            .unchecked_into::<Element>()
            .set_inner_html(&data);

        dom.append_child(node);
        // let node =
        //     dom.dangerously_set_inner_html(data_transf.get_data("text/html").unwrap().as_str());
        // log!(format!("{:?}", &node));

        // let drag_event = drag_event_ref.clone();
        // let data_transf: DataTransfer = drag_event.data_transfer().unwrap();

        // e.stop_propagation();
        // log!(format!("{:?}", e.type_()));
        // if e.type_().contains("dragend") {
        //     let data_transfer_ref: &web_sys::DataTransfer = e.unchecked_ref();

        //     dom.dangerously_set_inner_html(
        //         data_transfer_ref.get_data("text/html").unwrap().as_str(),
        //     );
        // }

        // dom.add_class("hide");
        // dom.set_attribute("style", "opacity: 0.2");
    };

    view! { cx,
        div(ref=node_ref, draggable=true, class="item", on:dragstart=handle_dragstart, on:dragend=handle_dragend, on:dragenter=handle_dragenter, on:dragover=handle_dragover, on:dragleave=handle_dragleave, on:drop=handle_drop) {
             //ItemWidget{}
            (bb.get())
        }
    }
}

// basic test

#[component]
fn DropZone<G: Html>(cx: Scope) -> View<G> {
    let node_ref = create_node_ref(cx);
    let count = create_signal(cx, vec![1, 2, 3, 4, 5]);
    view! { cx,
        div(ref=node_ref, class="box") {
             // DraggableItem{}
            Keyed(
                iterable=count,
                    view=|cx, x|
                        view! { cx,
                                // li { (x) }
                                DraggableItem(b=x.to_string())
                },
                key=|x| *x,
            )
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
