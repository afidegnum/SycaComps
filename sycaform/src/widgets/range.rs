use crate::FormData;
use serde_json::Value;
use std::collections::HashMap;
use sycamore::prelude::*;

#[component]
pub fn RangeInput<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
    let form_name: String = s.0.clone();
    let form_label: String = s.0.clone();

    let form_title = s.1.get("title").unwrap().as_str().unwrap().to_owned();

    let binding = s.1;

    // let minimal = binding.get("minimal");
    // let minimal = match minimal {
    //     Some(x) => match x {
    //         Value::Number(n) => n.as_u64(),
    //         _ => Some(0),
    //     },
    //     None => Some(0),
    // };
    let minimal = match binding.get("minimal") {
        Some(Value::Number(n)) => n.as_u64(),
        _ => Some(0),
    };

    // let maximal = binding.get("maximal");
    // let maximal = match maximal {
    //     Some(x) => match x {
    //         Value::Number(n) => n.as_u64(),
    //         _ => Some(0),
    //     },
    //     None => Some(0),
    // };
    let maximal = match binding.get("maximal") {
        Some(Value::Number(n)) => n.as_u64(),
        _ => Some(0),
    };

    // let steps = binding.get("multipleOf");
    // let steps = match steps {
    //     Some(x) => match x {
    //         Value::Number(n) => n.as_u64(),
    //         _ => Some(0),
    //     },
    //     None => Some(0),
    // };
    let steps = match binding.get("multipleOf") {
        Some(Value::Number(n)) => n.as_u64(),
        _ => Some(0),
    };

    let current_value = binding
        .get("default")
        .and_then(|v| v.as_u64())
        .unwrap_or_default();

    // let is_required = binding.get("required");
    // let is_required = match is_required {
    //     Some(x) => match x {
    //         Value::Bool(n) => *n,
    //         _ => false,
    //     },
    //     None => false,
    // };

    // let is_required = binding.get("required");
    //     let is_required = match is_required {
    //         Some(x) => match x {
    //             Value::Bool(n) => *n,
    //             _ => false,
    //         },
    //         None => false,
    //     };

    let is_required = match binding.get("required") {
        Some(Value::Bool(n)) => *n,
        _ => false,
    };

    let validation_message = create_signal(cx, ("valid-feedback", ""));
    let sample_data = create_signal(cx, current_value.clone().to_string());

    let data_context = use_context::<FormData>(cx);

    let handle_blur = move || {
        let datum = sample_data.get().as_ref().clone();

        let x = datum.parse::<u64>().unwrap_or(0);
        // check_length(sample_data.get().as_ref().clone());

        // let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();
        let val = Value::Number(serde_json::Number::from(x));

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
                     class="form-range", type="range", placeholder=current_value.clone(), required=is_required, pattern="", min=minimal.unwrap(), max=maximal.unwrap(),  step=steps.unwrap(), size="",
                     readonly=false, disabled=false, autofocus=false){}

                 div(class=validation_message.get().0){(validation_message.get().1)}
                 div{(sample_data.get())}
            }
    }
}
