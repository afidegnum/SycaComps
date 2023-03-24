use std::collections::HashMap;

use serde_json::Value;
use sycamore::prelude::*;

use crate::FormData;

#[component]
pub fn SearchInput<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
    let form_name: String = s.0.clone();
    let form_label: String = s.0.clone();

    let form_title = s.1.get("title").unwrap().as_str().unwrap().to_owned();

    let binding = s.1;

    // let min_length = binding.get("minLength");
    // let min_length = match min_length {
    //     Some(x) => match x {
    //         Value::Number(n) => n.to_string(),
    //         _ => "".to_owned(),
    //     },
    //     None => "".to_owned(),
    // };

    // let max_length = binding.get("maxLength");
    // let max_length = match max_length {
    //     Some(x) => match x {
    //         Value::Number(n) => n.to_string(),
    //         _ => "".to_owned(),
    //     },
    //     None => "".to_owned(),
    // };

    // let default_value = binding.get("default");
    // let default_value = match default_value {
    //     Some(x) => match x {
    //         Value::String(n) => n.to_string(),
    //         _ => "".to_owned(),
    //     },
    //     None => "".to_owned(),
    // };

    // let is_required = binding.get("required");
    // let required_mark = create_signal(cx, "");
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

    let min_length = binding.get("minLength");
    let min_length = match min_length {
        Some(Value::Number(n)) => n.to_string(),
        _ => "".to_owned(),
    };

    let max_length = binding.get("maxLength");
    let max_length = match max_length {
        Some(Value::Number(n)) => n.to_string(),
        _ => "".to_owned(),
    };

    let default_value = binding.get("default");
    let default_value = match default_value {
        Some(Value::String(n)) => n.to_string(),
        _ => "".to_owned(),
    };

    let is_required = binding.get("required");
    let required_mark = create_signal(cx, "");
    let is_required = match is_required {
        Some(Value::Bool(n)) => {
            required_mark.set("*");
            *n
        }
        _ => false,
    };

    let validation_message = create_signal(cx, ("valid-feedback", ""));
    validation_message.track();
    // validation lists
    // let vm = validation_message.clone();

    // let validate_alpha_numeric = "";
    // let validate_password = "";

    let sample_data = create_signal(cx, default_value.clone());

    let data_context = use_context::<FormData>(cx);

    let min_l = min_length.clone();
    let max_l = max_length.clone();
    let check_length = move |s: String| {
        let length = s.len();
        if length < (min_l.parse::<usize>().unwrap() - 1) {
            validation_message.set(("invalid-feedback", "Characters too low"));
        } else if length > (max_l.parse::<usize>().unwrap() + 2) {
            validation_message.set(("invalid-feedback", "character exceeded stay within range"));
        } else if length >= max_l.parse::<usize>().unwrap()
            || length <= max_l.parse::<usize>().unwrap()
        {
            validation_message.set(("valid-feedback", "looks good"));
        } else {
            validation_message.set(("valid-feedback", "Ok"));
        }
    };

    let check_empty = move |s: String| {
        if s.is_empty() {
            validation_message.set(("invalid-feedback", "this must not be empty"));
        }
    };

    let handle_blur = move || {
        let datum = sample_data.get().as_ref().clone();
        check_empty(datum.clone());
        // check_length(sample_data.get().as_ref().clone());

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
              label (class="form-label", for=form_label) {  (form_title) (required_mark.get()) }
              // input (ref=input_ref, on:blur=move|_| handle_blur(), bind:value=sample_data, class="form-control", id=form_name, type=form_type, disabled=false, readable=false, required=true)

              input( on:blur=move|_| handle_blur(), bind:value=sample_data,
                     class="form-control", type="search", placeholder=default_value.clone(), required=is_required, pattern="", minlength=min_length, maxlength=max_length, size="",
                     readonly=false, disabled=false, autofocus=false, autocomplete=""){}

                 div(class=validation_message.get().0){(validation_message.get().1)}
                 div{(sample_data.get())}
            }
    }
}
