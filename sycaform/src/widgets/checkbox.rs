use crate::FormData;
// use console_log::log;
use serde_json::{json, Value};
use std::collections::HashMap;
use sycamore::prelude::*;

#[component]
pub fn CheckBoxInput<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
    let form_name: String = s.0.clone();
    let _form_label: String = s.0.clone();

    let form_title =
        s.1.clone()
            .get("title")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();
    let binding = s.1.clone();

    let default_value = binding.get("default");
    let default_value = match default_value {
        Some(x) => match x {
            Value::String(n) => n.to_string(),
            _ => "".to_owned(),
        },
        None => "".to_owned(),
    };

    // let items_list = binding
    //     .get("default")
    //     .and_then(|v| v.as_array())
    //     .unwrap_or(&vec![]);

    let empty = vec![];

    let item_list = binding
        .get("enum")
        .and_then(|v| v.as_array())
        .unwrap_or(&empty);

    let items = item_list;

    // let array: Array = serde_json::from_str(items).unwrap();
    // let keys: Vec<&str> = items_list
    //     .iter()
    //     .map(|item| {
    //         item.get("key")
    //             .unwrap_or(&Value::String("".to_string()))
    //             .as_str()
    //     })
    //     .collect();

    // let optional_items = binding.get("default").and_then(|v| v.as_array());
    // let items = match optional_items {
    //     Some(v) => v,
    //     None => Value::Array(vec![]),
    // };
    let validation_message = create_signal(cx, ("", ""));
    // validation lists
    // let vm = validation_message.clone();

    // let validate_alpha_numeric = "";
    // let validate_password = "";
    let checked_value = create_signal(cx, String::new());

    // let checked = create_signal(cx, false);
    let sample_data = create_signal(cx, default_value.clone());

    let data_context = use_context::<FormData>(cx);

    // let items_list = binding.get("enum").cloned();
    // let get_key =
    //     |value: Value| -> Option<str> { value.get("key").and_then(|key| key.as_str()) };
    // let toggle_check = move |x: &str| checked_value.set(x);

    let items_signal = create_signal(cx, items.clone());

    // let selected_items = items_signal
    //     .get()
    //     // .as_ref()
    //     // .clone()
    //     .iter()
    //     .filter(|item| item["checked"].as_bool().unwrap_or(false))
    //     .cloned()
    //     .collect::<Vec<Value>>();

    // log::info!("{:#?}", selected_items);

    // let it = items.clone();
    // let fname = form_name.clone();
    /*
        let _handle_blur = move || {
            let fname = fname;
            let datum = sample_data.get().as_ref().clone();

            let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();

            // let f_name: String = s.0.clone();
            let mut this_data = HashMap::new();
            this_data.insert(fname, val.to_owned());

            let mut dt = data_context.data.get().as_ref().clone();
            dt.extend(this_data.clone());
            data_context.data.set(dt);
        };
    */
    view! { cx,
            div (class="mb-3 needs-validation") {
                    div(class="card"){
                        div(class="card-body"){
                            div (class="form-label" ) {  (form_title) }
                      // input (ref=input_ref, on:blur=move|_| handle_blur(), bind:value=sample_data, class="form-control", id=form_name, type=form_type, disabled=false, readable=false, required=true)

                        div(class=""){
                           // ul(class="list-group list-group-flush"){
                        Keyed(
                           iterable=items_signal,
                           view= move |cx, x|
                            {
                                // let y = x.clone();
                                let fname = form_name.clone();
                                let fnx = fname.clone();
                                let key = x.get("key").unwrap().as_str().unwrap().to_owned();
                                let kx = key.clone();
                                let value = x.get("name").unwrap().as_str().unwrap().to_owned();
                                let check_it = x.get("checked").unwrap().as_bool().unwrap().to_owned();


                                // let val: Value = serde_json::from_str(&format!("\"{}\"", checked_value.get())).unwrap();

                                // let mut this_data = HashMap::new();
                                // this_data.insert(fname.clone(), val.to_owned());
                                // let mut dt = data_context.data.get().as_ref().clone();
                                // dt.extend(this_data.clone());
                                // data_context.data.set(dt);


                                // let checked_set = move |_| checked_value.set(key.clone());


                                 let checked_set = move |_|
                                    {
                                        let fname = fname.clone();
                                        // let datum = sample_data.get().as_ref().clone();
                                        // log::info!("{:#?}", y["name"]);

                                        // checked_value.set(key.clone());

                                        let new_items = items_signal.get().as_ref().clone().iter().cloned().collect::<Vec<Value>>();
                                        // let mut its = new_items.to_vec();

                                        // updated_item(&mut its, &key);

                                        let new_obj = new_items.iter().map(|item| {
                                            if item["key"] == key.clone() {
                                                let checked = item["checked"].clone();
                                                let mut new_item = item.clone();
                                                if let serde_json::Value::Bool(checked_value) = checked {
                                                    new_item["checked"] = serde_json::Value::Bool(!checked_value);
                                                }
                                                new_item
                                            } else {
                                                item.clone()
                                            }
                                        }).collect::<Vec<serde_json::Value>>();
                                        items_signal.set(new_obj);
                                        items_signal.track();
                                        log::info!("{:#?}", items_signal.get());


                                        let selected_items = items_signal
                                                .get()
                                                // .as_ref()
                                                // .clone()
                                                .iter()
                                                .filter(|item| item["checked"].as_bool().unwrap_or(false))
                                                .cloned()
                                                .collect::<Vec<Value>>();

                                        // let val: Value = serde_json::from_str(&format!("\"{}\"", checked_value.get().as_ref().clone())).unwrap();
                                        let val: Value = json!(selected_items.clone());

                                        // let f_name: String = s.0.clone();
                                        let mut this_data = HashMap::new();
                                        this_data.insert(fname, val.to_owned());

                                        let mut dt = data_context.data.get().as_ref().clone();
                                        dt.extend(this_data.clone());
                                        data_context.data.set(dt);

                                    };
                                   view! { cx,
                                // input (class="form-check-input", type="radio", name="inlineRadioOptions", id="inlineRadio1", value="option1") {"---"}
                                // li (class="list-group-item") { input (class="form-check-input me-1", type="radio") {} (y)}
                                 div(class="form-check"){
                                 input (on:click=checked_set, checked=check_it.clone(), class="form-check-input", id=kx.clone(), name=fnx.clone(), type="checkbox") {}
                                 label (class="form-check-label", for= x.get("key").unwrap().as_str().unwrap().to_owned()) {(value)}
                                 }
                           }
                            },
                            key= move |x| {let y = x.as_object().unwrap(); y.get("key").unwrap().as_str().unwrap().to_owned()},
                       )
                    //}
                        }
                  }
                }

                // ul (class="list-group") {
                //     li (class="list-group-item") {
                //         input (class="form-check-input me-1", type="checkbox", value="", aria-label="...") {}
                //         " First checkbox"
                //     }

                // }

               div(class=validation_message.get().0){(validation_message.get().1)}
               div{(sample_data.get())}
               div{(checked_value.get())}
                // div{(format!("{:#?}", items_signal.get()))}
            }
    }
}
