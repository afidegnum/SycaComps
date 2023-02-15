use crate::FormData;
use serde_json::Value;
use std::collections::HashMap;
use sycamore::prelude::*;

#[component]
pub fn DataList<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
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
    let validation_message = create_signal(cx, ("valid-feedback", "looks good"));
    // validation lists
    // let vm = validation_message.clone();

    // let validate_alpha_numeric = "";
    // let validate_password = "";

    let sample_data = create_signal(cx, default_value.clone());

    let data_context = use_context::<FormData>(cx);

    // let items_list = binding.get("enum").cloned();
    // let get_key =
    //     |value: Value| -> Option<str> { value.get("key").and_then(|key| key.as_str()) };

    let items_signal = create_signal(cx, items.clone());
    // let it = items.clone();
    let fname = form_name.clone();
    let handle_blur = move || {
        let datum = sample_data.get().as_ref().clone();
        // check_empty(datum.clone());
        //check_length(sample_data.get().as_ref().clone());

        let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();

        // let f_name: String = s.0.clone();
        let mut this_data = HashMap::new();
        this_data.insert(fname.clone(), val.to_owned());

        let mut dt = data_context.data.get().as_ref().clone();
        dt.extend(this_data.clone());
        data_context.data.set(dt);
    };

    view! { cx,
            div (class="mb-3 needs-validation") {
                    div(class="card"){
                        div(class="card-body"){
                            div (class="form-label" ) {  (form_title) }
                      // input (ref=input_ref, on:blur=move|_| handle_blur(), bind:value=sample_data, class="form-control", id=form_name, type=form_type, disabled=false, readable=false, required=true)

                            input( on:blur=move|_| handle_blur(), bind:value=sample_data, list="items-hint",
                             class="form-control", type="text", placeholder=default_value.clone(), /*required=is_required,*/ pattern="", size="",
                             readonly=false, disabled=false, autofocus=false, autocomplete=""){}


                        datalist(id="items-hint"){
                           // ul(class="list-group list-group-flush"){
                        Keyed(
                           iterable=items_signal,
                           view= move |cx, x|
                            {
                                // let fname = form_name.clone();
                                // let key = x.get("key").unwrap().as_str().unwrap().to_owned();
                                let value = x.get("name").unwrap().as_str().unwrap().to_owned();
                                    view! { cx,
                                // input (class="form-check-input", type="radio", name="inlineRadioOptions", id="inlineRadio1", value="option1") {"---"}
                                // li (class="list-group-item") { input (class="form-check-input me-1", type="radio") {} (y)}
                                 // div(class="form-check"){
                                 // input (class="form-check-input", id=key.clone(), name=fname, type="radio") {}
                                 // label (class="form-check-label", for= x.get("key").unwrap().as_str().unwrap().to_owned()) {(value)}
                                 // }
                                 option(value=value){}
                           }
                            }
                            ,
                           key= move |x| {let y = x.as_object().unwrap(); y.get("key").unwrap().as_str().unwrap().to_owned()},
                       )
                    //}
                        }
                  }
                }




               div(class=validation_message.get().0){(validation_message.get().1)}
               div{(sample_data.get())}
                // div{(format!("{:#?}", items_signal.get()))}
            }
    }
}
