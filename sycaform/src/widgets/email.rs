use std::collections::HashMap;
use sycamore::prelude::*;

use crate::FormData;
use serde_json::Value;

#[component]
pub fn EmailInput<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
    let form_name: String = s.0.clone();
    let form_label: String = s.0.clone();

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

    let validation_message = create_signal(cx, ("valid-feedback", "looks good"));
    // validation lists
    // let vm = validation_message.clone();

    // let validate_alpha_numeric = "";
    // let validate_password = "";

    let sample_data = create_signal(cx, default_value.clone());

    let data_context = use_context::<FormData>(cx);

    let is_required = binding.get("required");
    let is_required = match is_required {
        Some(x) => match x {
            Value::Bool(n) => *n,
            _ => false,
        },
        None => false,
    };

    let handle_blur = move || {
        let datum = sample_data.get().as_ref().clone();

        let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();

        // let f_name: String = s.0.clone();
        let mut this_data = HashMap::new();
        this_data.insert(form_name.clone(), val.to_owned());

        let mut dt = data_context.data.get().as_ref().clone();
        dt.extend(this_data.clone());
        data_context.data.set(dt);
    };

    view! { cx,
            div (class="mb-3 needs-validation") {
              label (class="form-label", for=form_label) {  (form_title) }
              // input (ref=input_ref, on:blur=move|_| handle_blur(), bind:value=sample_data, class="form-control", id=form_name, type=form_type, disabled=false, readable=false, required=true)

              input( on:blur=move|_| handle_blur(), bind:value=sample_data,
                     class="form-control", type="email", placeholder=default_value.clone(), required=is_required,   readonly=false, disabled=false, autofocus=false, autocomplete=""){}

                 div(class=validation_message.get().0){(validation_message.get().1)}
                 div{(sample_data.get())}
            }
    }
}
