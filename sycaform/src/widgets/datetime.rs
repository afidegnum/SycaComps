use crate::FormData;
use serde_json::Value;
use std::collections::HashMap;
use sycamore::prelude::*;

#[component]
pub fn DateTimeInput<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
    let form_name: String = s.0.clone();
    let form_label: String = s.0.clone();

    let form_title = s.1.get("title").unwrap().as_str().unwrap().to_owned();

    let binding = s.1;

    // let min_length = match min_length {
    //     Some(x) => match x {
    //         Value::Number(n) => n.to_string(),
    //         _ => "".to_owned(),
    //     },
    //     None => "".to_owned(),
    // };
    //
    let minimal = match binding.get("minimal") {
        Some(Value::Number(n)) => n.as_u64(),
        _ => Some(0),
    };

    let maximal = match binding.get("maximal") {
        Some(Value::Number(n)) => n.as_u64(),
        _ => Some(0),
    };

    let steps = match binding.get("multipleOf") {
        Some(Value::Number(n)) => n.as_u64(),
        _ => Some(0),
    };

    let default_value = match binding.get("default") {
        Some(Value::String(n)) => n.to_string(),
        _ => "".to_owned(),
    };

    let is_required = match binding.get("required") {
        Some(Value::Bool(n)) => *n,
        _ => false,
    };
    let validation_message = create_signal(cx, ("", ""));
    // validation lists
    // let vm = validation_message.clone();

    // let validate_alpha_numeric = "";
    // let validate_password = "";

    let sample_data = create_signal(cx, default_value.clone());

    let data_context = use_context::<FormData>(cx);

    // let min_l = min_length.clone();
    // let max_l = max_length.clone();
    // let check_length = move |s: String| {
    //     let length = s.len();
    //     if length < (min_l.parse::<usize>().unwrap() - 1) {
    //         println!("It's too low");
    //         validation_message.set(("invalid-feedback", "Character too low"));
    //     } else if length > (max_l.parse::<usize>().unwrap() + 2) {
    //         println!("you've exceeded your limit");
    //         validation_message
    //             .set(("invalid-feedback", "character exceeded stay within range"));
    //     }
    // };

    let handle_blur = move || {
        let datum = sample_data.get().as_ref().clone();

        // check_length(sample_data.get().as_ref().clone());

        let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();
        // let val = Value::String(serde_json::Str::from(x));

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

              input( on:blur=move|_| handle_blur(),  bind:value=sample_data,
                     class="form-control", type="datetime-local", placeholder=default_value.clone(), required=is_required, pattern="", min=minimal.unwrap(), max=maximal.unwrap(), step=steps.unwrap(), size="",
                     readonly=false, disabled=false, autofocus=false){}

                 div(class=validation_message.get().0){(validation_message.get().1)}
                 div{(sample_data.get())}
            }
    }
}
