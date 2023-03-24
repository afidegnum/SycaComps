use crate::FormData;
use serde_json::Value;
use std::collections::HashMap;
use sycamore::prelude::*;

#[component]
pub fn FileInput<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
    let form_name: String = s.0.clone();
    let form_label: String = s.0.clone();

    let form_title = s.1.get("title").unwrap().as_str().unwrap().to_owned();
    let binding = s.1;

    // let default_value = binding.get("default");
    // let default_value = match default_value {
    //     Some(x) => match x {
    //         Value::String(n) => n.to_string(),
    //         _ => "".to_owned(),
    //     },
    //     None => "".to_owned(),
    // };

    // let accept_filetype = binding.get("accept");
    // let accept_filetype = match accept_filetype {
    //     Some(x) => match x {
    //         Value::String(n) => n.to_string(),
    //         _ => "".to_owned(),
    //     },
    //     None => "".to_owned(),
    // };

    // let is_multiple = binding.get("multiple");
    // let is_multiple = match is_multiple {
    //     Some(x) => match x {
    //         Value::Bool(n) => *n,
    //         _ => false,
    //     },
    //     None => false,
    // };

    let default_value = binding
        .get("default")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();

    let accept_filetype = binding
        .get("accept")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();

    let is_multiple = binding
        .get("multiple")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let validation_message = create_signal(cx, ("", ""));
    // validation lists
    // let vm = validation_message.clone();

    // let validate_alpha_numeric = "";
    // let validate_password = "";

    let sample_data = create_signal(cx, default_value.clone());

    let data_context = use_context::<FormData>(cx);

    let is_required = binding.get("required");
    let required_mark = create_signal(cx, "");
    // let is_required = match is_required {
    //     Some(x) => match x {
    //         Value::Bool(n) => {
    //             required_mark.set("*");
    //             *n
    //         }
    //         _ => false,
    //     },
    //     None => false,
    // };
    let is_required = match binding.get("required") {
        Some(Value::Bool(n)) => *n,
        _ => false,
    };

    let handle_blur = move || {
        let datum = sample_data.get().as_ref().clone();

        let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();

        // let f_name: String = s.0.clone();
        let mut this_data = HashMap::new();
        this_data.insert(form_name.clone(), val);

        let mut dt = data_context.data.get().as_ref().clone();
        dt.extend(this_data.clone());
        data_context.data.set(dt);
    };

    view! { cx,
            div (class="mb-3 needs-validation") {
              label (class="form-label", for=form_label) {  (form_title) }
              // input (ref=input_ref, on:blur=move|_| handle_blur(), bind:value=sample_data, class="form-control", id=form_name, type=form_type, disabled=false, readable=false, required=true)

              input( on:blur=move|_| handle_blur(), bind:value=sample_data,
                     class="form-control", type="file",accept=accept_filetype, placeholder=default_value.clone(), required=is_required, size="", readonly=false, disabled=false, autofocus=false, multiple=is_multiple){}
                 div(class=validation_message.get().0){(validation_message.get().1)}
                 div{(sample_data.get())}
            }
    }
}
