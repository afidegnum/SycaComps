use std::collections::HashMap;

use serde_json::Value;
use sycamore::prelude::*;

use crate::FormData;

#[component]
pub fn PassWordInput<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
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

    let min_length = binding
        .get("minLength")
        .and_then(|v| v.as_u64())
        .map(|n| n.to_string())
        .unwrap_or("".to_owned());

    let max_length = binding
        .get("maxLength")
        .and_then(|v| v.as_u64())
        .map(|n| n.to_string())
        .unwrap_or("".to_owned());

    let validation_message = create_signal(cx, ("valid-feedback", ""));
    // validation lists
    // let vm = validation_message.clone();

    // let validate_alpha_numeric = "";
    // let validate_password = "";

    let password = create_signal(cx, "".to_string());
    let password_repeat = create_signal(cx, "".to_string());

    let data_context = use_context::<FormData>(cx);

    let min_l = min_length.clone();
    let max_l = max_length.clone();
    let check_length = move |s: String| {
        let length = s.len();
        if length < (min_l.parse::<usize>().unwrap() - 1) {
            validation_message.set(("invalid-feedback", "Character too low"));
        } else if length > (max_l.parse::<usize>().unwrap() + 2) {
            validation_message.set(("invalid-feedback", "character exceeded stay within range"));
        }
    };
    // let is_required = binding.get("required");
    // let is_required = match is_required {
    //     Some(x) => match x {
    //         Value::Bool(n) => *n,
    //         _ => false,
    //     },
    //     None => false,
    // };
    let is_required = match binding.get("required") {
        Some(Value::Bool(n)) => *n,
        _ => false,
    };

    let handle_blur = move || {
        let datum = password.get().as_ref().clone();

        check_length(password.get().as_ref().clone());
        let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();

        // let f_name: String = s.0.clone();
        let mut this_data = HashMap::new();
        this_data.insert(form_name.clone(), val);

        let mut dt = data_context.data.get().as_ref().clone();
        dt.extend(this_data.clone());
        data_context.data.set(dt);
    };

    let handle_repeat = move || {
        if password.get().as_ref().clone() != password_repeat.get().as_ref().clone() {
            validation_message.set(("invalid-feedback", "password do not match"))
        }
    };

    view! { cx,
            div (class="mb-3 needs-validation") {
              label (class="form-label", for=form_label) {  (form_title) }
              // input (ref=input_ref, on:blur=move|_| handle_blur(), bind:value=sample_data, class="form-control", id=form_name, type=form_type, disabled=false, readable=false, required=true)

              input( on:blur=move|_| handle_blur(), bind:value=password,
                     class="form-control", type="password",
                     required=is_required, pattern="", minlength=min_length, maxlength=max_length, size="", readonly=false, disabled=false, autofocus=false ){}

              label (class="form-label", for="pwrepeat") {  "Password Repeat" }
               input(on:blur=move|_| handle_repeat(), id="pwrepeat", class="form-control", bind:value=password_repeat, type="password",
                     required=false, pattern="", size="", readonly=false, disabled=false, autofocus=false ){}


                 div(class=validation_message.get().0){(validation_message.get().1)}
                div{(password.get())}

            }
    }
}
