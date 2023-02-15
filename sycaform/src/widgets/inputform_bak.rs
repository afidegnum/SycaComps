use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use sycamore::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::FormData;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Array {
    items: Vec<Item>,
}

#[component]
pub fn InputForm<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
    let form_name: String = s.0.clone();
    let form_label: String = s.0.clone();

    let form_title =
        s.1.clone()
            .get("title")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();

    let field_type =
        s.1.clone()
            .get("type")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();

    //     let min_length =
    //         s.1.clone()
    //             .get("minLength")
    //             .unwrap()
    //             .as_str()
    //             .is_some()
    //             .to_owned();

    //     if let Some(mut x) = v_iter.next() {
    //     buf.append(&mut x);
    // }
    // let binding = s.1.clone();
    // let min_length = binding.get("minLength");

    // // let min_length = match min_length {
    // //     Some(x) => match x {
    // //         Value::Number(n) => n.to_string(),
    // //         _ => "".to_owned(),
    // //     },
    // //     None => "".to_owned(),
    // // };

    // let min_length = match min_length {
    //     Some(x) => match x {
    //         Value::Number(n) => n.as_i64().unwrap_or(0),
    //         _ => 0,
    //     },
    //     None => 0,
    // };

    // // let min_length = if let Some(x) = min_length {
    // //     serde_json::Value::String(x.to_owned()) /*x.to_owned()*/
    // // };
    // // let s: Option<serde_json::Value> = Some("test".to_owned());

    // let form_type = match field_type.as_ref() {
    //     r#"string"# => "text",
    //     _ => "nothing",
    // };

    // let input_ref = create_node_ref(cx);

    // let sample_data = create_signal(cx, "".to_string());

    // let data_context = use_context::<FormData>(cx);

    // let handle_blur = move || {
    //     let datum = sample_data.get().as_ref().clone();

    //     let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();

    //     let f_name: String = s.0.clone();
    //     let mut this_data = HashMap::new();
    //     this_data.insert(f_name.clone(), val.to_owned());

    //     let mut dt = data_context.data.get().as_ref().clone();
    //     dt.extend(this_data.clone());
    //     data_context.data.set(dt);
    // };

    // let handle_change = move || {
    //     let min_length = 0;
    //     let max_length = 0;
    // };

    // let input_type = match data_type.as_ref() {
    //     "text" =>
    //     input(type="text", value="", name="", placeholder="", required=false, pattern="", minlength="", maxlength="", size="", readonly=false, disabled=false, autofocus=false, autocomplete=""),
    //     input(type="password", value="", name="", placeholder="", required=false, pattern="", minlength="", maxlength="", size="", readonly=false, disabled=false, autofocus=false),
    //     "radio" => input(type="radio", value="", name="", checked=false, required=false, readonly=false, disabled=false, autofocus=false),
    //     "checkbox" => input(type="checkbox", value="", name="", checked=false, required=false, readonly=false, disabled=false, autofocus=false),
    //     "submit" => input(type="submit", value="", name="", formnovalidate=false, formaction="", formtarget="", disabled=false),
    //     "file" => input(type="file", name="", multiple=false, accept="", required=false, disabled=false, value=""),
    //     "image" => input(type="image", name="", src="", alt="", width="", height="", formaction="", formtarget="", formnovalidate=false, disabled=false),
    //     "range" => input(type="range", name="", value="", min="", max="", step="", required=false, readonly=false, disabled=false, autofocus=false),
    //     "color" => input(type="color", value="", name="", required=false, disabled=false),
    //     "date" => input(type="date", value="", name="", required=false, disabled=false),
    //     "time" => input(type="time", value="", name="", required=false, disabled=false),
    //     "datetime" => input(type="datetime", value="", name="", required=false, disabled=false),
    //     "datetime-local" => input(type="datetime-local", value="", name="", required=false, disabled=false),
    //     "week" => input(type="week", value="", name="", required=false, disabled=false),
    //     "month" => input(type="month", value="", name="", required=false, disabled=false),
    //     "search" => input(type="search", value="", name="", required=false, disabled=false),
    //     "tel" => input(type="tel", value="", name="", required=false, disabled=false),
    //     "email" => input(type="email", value="", name="", required=false, disabled=false),
    //     "url" => input(type="url", value="", name="", required=false, disabled=false),
    //     "number" => input(type="number", value="", name="", required=false, disabled=false),
    //     // "hidden" => input(type="hidden", value="", name=""),
    //     //"textarea" => input(type="textarea", value="", name="", rows="", cols="", required=false, readonly=false, disabled=false, autofocus=false)

    let input_type = match field_type.as_ref() {
        "string" => {
            let binding = s.1.clone();
            let min_length = binding.get("minLength");

            // let min_length = match min_length {
            //     Some(x) => match x {
            //         Value::Number(n) => n.to_string(),
            //         _ => "".to_owned(),
            //     },
            //     None => "".to_owned(),
            // };

            let min_length = match min_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let max_length = binding.get("maxLength");
            let max_length = match max_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let default_value = binding.get("default");
            let default_value = match default_value {
                Some(x) => match x {
                    Value::String(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let is_required = binding.get("required");
            let is_required = match is_required {
                Some(x) => match x {
                    Value::Bool(n) => *n,
                    _ => false,
                },
                None => false,
            };

            let validation_message = create_signal(cx, ("valid-feedback", ""));
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
                    validation_message
                        .set(("invalid-feedback", "character exceeded stay within range"));
                } else if length >= max_l.parse::<usize>().unwrap()
                    || length <= max_l.parse::<usize>().unwrap()
                {
                    validation_message.set(("valid-feedback", "looks good"));
                }
            };

            let handle_blur = move || {
                let datum = sample_data.get().as_ref().clone();

                check_length(sample_data.get().as_ref().clone());

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
                             class="form-control", type="text", placeholder=default_value.clone(), required=is_required, pattern="", minlength=min_length, maxlength=max_length, size="",
                             readonly=false, disabled=false, autofocus=false, autocomplete=""){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }
        "textarea" => {
            let binding = s.1.clone();

            let max_length = binding.get("maxLength");
            let max_length = match max_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let default_value = binding.get("default");
            let default_value = match default_value {
                Some(x) => match x {
                    Value::String(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let validation_message = create_signal(cx, ("valid-feedback", ""));
            // validation lists
            // let vm = validation_message.clone();

            // let validate_alpha_numeric = "";
            // let validate_password = "";

            let sample_data = create_signal(cx, default_value.clone());

            let data_context = use_context::<FormData>(cx);

            let max_l = max_length.clone();
            let check_length = move |s: String| {
                let length = s.len();
                if length > (max_l.parse::<usize>().unwrap() + 1) {
                    println!("It's too low");
                    validation_message.set(("invalid-feedback", "Character too low"));
                }
            };
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

                check_length(sample_data.get().as_ref().clone());

                let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();

                // let f_name: String = s.0.clone();
                let mut this_data = HashMap::new();
                this_data.insert(form_name.clone(), val.to_owned());

                let mut dt = data_context.data.get().as_ref().clone();
                dt.extend(this_data.clone());
                data_context.data.set(dt);
            };

            view! { cx,
                    div (class="mb-3 form-loating needs-validation") {
                      label (class="form-label", for=form_label) {  (form_title) }

                      // input( on:blur=move|_| handle_blur(), bind:value=sample_data,
                      //        class="form-control", type="text", placeholder=default_value.clone(), required=false, pattern="", minlength=min_length, maxlength=max_length, size="",
                      //        readonly=false, disabled=false, autofocus=false, autocomplete=""){}

                       input( on:blur=move|_| handle_blur(), bind:value=sample_data, class="form-control", placeholder=default_value.clone(),  type="textarea", rows="", maxlength=max_length, cols="", required=is_required, readonly=false, disabled=false, autofocus=false)

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "password" => {
            let binding = s.1.clone();
            let min_length = binding.get("minLength");

            let min_length = match min_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let max_length = binding.get("maxLength");
            let max_length = match max_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let validation_message = create_signal(cx, ("valid-feedback", "looks good"));
            // validation lists
            // let vm = validation_message.clone();

            // let validate_alpha_numeric = "";
            // let validate_password = "";

            let password = create_signal(cx, "".to_string());
            let password_repeat = create_signal(cx, "".to_string());

            let data_context = use_context::<FormData>(cx);

            let min_l = min_length.clone();
            let max_l = max_length.clone();
            let check_length = move |s: String, s1: String| {
                let length = s.len();
                if length < (min_l.parse::<usize>().unwrap() - 1) {
                    validation_message.set(("invalid-feedback", "Character too low"));
                } else if length > (max_l.parse::<usize>().unwrap() + 2) {
                    validation_message
                        .set(("invalid-feedback", "character exceeded stay within range"));
                } else if s != s1 {
                    validation_message.set(("invalid-feedback", "password do not match"))
                }
            };
            let is_required = binding.get("required");
            let is_required = match is_required {
                Some(x) => match x {
                    Value::Bool(n) => *n,
                    _ => false,
                },
                None => false,
            };

            let handle_blur = move || {
                let datum = password.get().as_ref().clone();

                check_length(
                    password.get().as_ref().clone(),
                    password_repeat.get().as_ref().clone(),
                );

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

                      input( on:blur=move|_| handle_blur(), bind:value=password,
                             class="form-control", type="password",
                             required=is_required, pattern="", minlength=min_length, maxlength=max_length, size="", readonly=false, disabled=false, autofocus=false ){}

                       input(  class="form-control", bind:value=password_repeat, type="password",
                             required=false, pattern="", size="", readonly=false, disabled=false, autofocus=false ){}


                         div(class=validation_message.get().0){(validation_message.get().1)}

                    }
            }
        }

        "integer" => {
            let binding = s.1.clone();

            // let min_length = match min_length {
            //     Some(x) => match x {
            //         Value::Number(n) => n.to_string(),
            //         _ => "".to_owned(),
            //     },
            //     None => "".to_owned(),
            // };
            let minimal = binding.get("minimal");
            let minimal = match minimal {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            let maximal = binding.get("maximal");
            let maximal = match maximal {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            let steps = binding.get("multipleOf");
            let steps = match steps {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            // let default_value = binding.get("default");
            // let default = match default_value {
            //     Some(x) => match x {
            //         Value::Number(n) => n.as_u64(),
            //         _ => Some(0),
            //     },
            //     None => Some(0),
            // };

            let current_value = binding
                .get("default")
                .and_then(|v| v.as_u64())
                .unwrap_or_default();

            let is_required = binding.get("required");
            let is_required = match is_required {
                Some(x) => match x {
                    Value::Bool(n) => *n,
                    _ => false,
                },
                None => false,
            };

            let validation_message = create_signal(cx, ("valid-feedback", "looks good"));
            // validation lists
            // let vm = validation_message.clone();

            // let validate_alpha_numeric = "";
            // let validate_password = "";

            let sample_data = create_signal(cx, current_value.clone().to_string());

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

                let x = match datum.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                // check_length(sample_data.get().as_ref().clone());

                // let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();
                let val = Value::Number(serde_json::Number::from(x));

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

                      input( on:blur=move|_| handle_blur(),  bind:value=sample_data,
                             class="form-control", type="number", placeholder=current_value.clone(), required=is_required, pattern="", min=minimal.is_some(), max=maximal.is_some(), step=steps.unwrap(), size="",
                             readonly=false, disabled=false, autofocus=false){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "range" => {
            let binding = s.1.clone();
            let min_length = binding.get("minLength");

            // let min_length = match min_length {
            //     Some(x) => match x {
            //         Value::Number(n) => n.to_string(),
            //         _ => "".to_owned(),
            //     },
            //     None => "".to_owned(),
            // };
            let minimal = binding.get("minimal");
            let minimal = match minimal {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            let maximal = binding.get("maximal");
            let maximal = match maximal {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            let steps = binding.get("multipleOf");
            let steps = match steps {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            // let default_value = binding.get("default");
            // let default = match default_value {
            //     Some(x) => match x {
            //         Value::Number(n) => n.as_u64(),
            //         _ => Some(0),
            //     },
            //     None => Some(0),
            // };

            let current_value = binding
                .get("default")
                .and_then(|v| v.as_u64())
                .unwrap_or_default();

            let is_required = binding.get("required");
            let is_required = match is_required {
                Some(x) => match x {
                    Value::Bool(n) => *n,
                    _ => false,
                },
                None => false,
            };

            let validation_message = create_signal(cx, ("valid-feedback", "looks good"));
            // validation lists
            // let vm = validation_message.clone();

            // let validate_alpha_numeric = "";
            // let validate_password = "";

            let sample_data = create_signal(cx, current_value.clone().to_string());

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

                let x = match datum.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                // check_length(sample_data.get().as_ref().clone());

                // let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();
                let val = Value::Number(serde_json::Number::from(x));

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

                      input( on:blur=move|_| handle_blur(),  bind:value=sample_data,
                             class="form-range", type="range", placeholder=current_value.clone(), required=is_required, pattern="", /*min=minimal.is_some(), max=maximal.is_some(), */ step="", size="",
                             readonly=false, disabled=false, autofocus=false){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "color" => {
            let binding = s.1.clone();

            let is_required = binding.get("required");
            let is_required = match is_required {
                Some(x) => match x {
                    Value::Bool(n) => *n,
                    _ => false,
                },
                None => false,
            };

            let validation_message = create_signal(cx, ("valid-feedback", "looks good"));
            // validation lists
            // let vm = validation_message.clone();

            // let validate_alpha_numeric = "";
            // let validate_password = "";

            let sample_data = create_signal(cx, "".to_string());

            let data_context = use_context::<FormData>(cx);

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
                             class="form-control", type="color",  disabled=false,  required=is_required,    readonly=false, disabled=false, autofocus=false ){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "email" => {
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

        "tel" => {
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
                             class="form-control", type="tel", placeholder=default_value.clone(), required=is_required, pattern="",       readonly=false, disabled=false, autofocus=false, autocomplete=""){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "date" => {
            let binding = s.1.clone();

            // let min_length = match min_length {
            //     Some(x) => match x {
            //         Value::Number(n) => n.to_string(),
            //         _ => "".to_owned(),
            //     },
            //     None => "".to_owned(),
            // };
            let minimal = binding.get("minimal");
            let minimal = match minimal {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            let maximal = binding.get("maximal");
            let maximal = match maximal {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            let steps = binding.get("multipleOf");
            let steps = match steps {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            // let default_value = binding.get("default");
            // let default = match default_value {
            //     Some(x) => match x {
            //         Value::Number(n) => n.as_u64(),
            //         _ => Some(0),
            //     },
            //     None => Some(0),
            // };

            let current_value = binding
                .get("default")
                .and_then(|v| v.as_u64())
                .unwrap_or_default();

            let is_required = binding.get("required");
            let is_required = match is_required {
                Some(x) => match x {
                    Value::Bool(n) => *n,
                    _ => false,
                },
                None => false,
            };

            let validation_message = create_signal(cx, ("valid-feedback", "looks good"));
            // validation lists
            // let vm = validation_message.clone();

            // let validate_alpha_numeric = "";
            // let validate_password = "";

            let sample_data = create_signal(cx, current_value.clone().to_string());

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

                let x = match datum.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                // check_length(sample_data.get().as_ref().clone());

                // let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();
                let val = Value::Number(serde_json::Number::from(x));

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

                      input( on:blur=move|_| handle_blur(),  bind:value=sample_data,
                             class="form-control", type="date", placeholder=current_value.clone(), required=is_required, pattern="", min=minimal.unwrap(), max=maximal.unwrap(), step=steps.unwrap(), size="",
                             readonly=false, disabled=false, autofocus=false){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "datetime" => {
            let binding = s.1.clone();
            let min_length = binding.get("minLength");

            // let min_length = match min_length {
            //     Some(x) => match x {
            //         Value::Number(n) => n.to_string(),
            //         _ => "".to_owned(),
            //     },
            //     None => "".to_owned(),
            // };
            let minimal = binding.get("minimal");
            let minimal = match minimal {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            let maximal = binding.get("maximal");
            let maximal = match maximal {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            let steps = binding.get("multipleOf");
            let steps = match steps {
                Some(x) => match x {
                    Value::Number(n) => n.as_u64(),
                    _ => Some(0),
                },
                None => Some(0),
            };

            // let default_value = binding.get("default");
            // let default = match default_value {
            //     Some(x) => match x {
            //         Value::Number(n) => n.as_u64(),
            //         _ => Some(0),
            //     },
            //     None => Some(0),
            // };

            let current_value = binding
                .get("default")
                .and_then(|v| v.as_u64())
                .unwrap_or_default();

            let is_required = binding.get("required");
            let is_required = match is_required {
                Some(x) => match x {
                    Value::Bool(n) => *n,
                    _ => false,
                },
                None => false,
            };

            let validation_message = create_signal(cx, ("valid-feedback", "looks good"));
            // validation lists
            // let vm = validation_message.clone();

            // let validate_alpha_numeric = "";
            // let validate_password = "";

            let sample_data = create_signal(cx, current_value.clone().to_string());

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

                let x = match datum.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                // check_length(sample_data.get().as_ref().clone());

                // let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();
                let val = Value::Number(serde_json::Number::from(x));

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

                      input( on:blur=move|_| handle_blur(),  bind:value=sample_data,
                             class="form-control", type="number", placeholder=current_value.clone(), required=is_required, pattern="", min=minimal.is_some(), max=maximal.is_some(), step="", size="",
                             readonly=false, disabled=false, autofocus=false){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "file" => {
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
                             class="form-control", type="url", placeholder=default_value.clone(), required=is_required, pattern="", size="", readonly=false, disabled=false, autofocus=false, autocomplete=""){}
                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "image" => {
            let binding = s.1.clone();
            let min_length = binding.get("minLength");

            // let min_length = match min_length {
            //     Some(x) => match x {
            //         Value::Number(n) => n.to_string(),
            //         _ => "".to_owned(),
            //     },
            //     None => "".to_owned(),
            // };

            let min_length = match min_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let max_length = binding.get("maxLength");
            let max_length = match max_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

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

            let min_l = min_length.clone();
            let max_l = max_length.clone();
            let check_length = move |s: String| {
                let length = s.len();
                if length < (min_l.parse::<usize>().unwrap() - 1) {
                    println!("It's too low");
                    validation_message.set(("invalid-feedback", "Character too low"));
                } else if length > (max_l.parse::<usize>().unwrap() + 2) {
                    println!("you've exceeded your limit");
                    validation_message
                        .set(("invalid-feedback", "character exceeded stay within range"));
                }
            };

            let handle_blur = move || {
                let datum = sample_data.get().as_ref().clone();

                check_length(sample_data.get().as_ref().clone());

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
                             class="form-control", type="text", placeholder=default_value.clone(), required=false, pattern="", minlength=min_length, maxlength=max_length, size="",
                             readonly=false, disabled=false, autofocus=false, autocomplete=""){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "url" => {
            let binding = s.1.clone();
            let min_length = binding.get("minLength");

            // let min_length = match min_length {
            //     Some(x) => match x {
            //         Value::Number(n) => n.to_string(),
            //         _ => "".to_owned(),
            //     },
            //     None => "".to_owned(),
            // };

            let min_length = match min_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let max_length = binding.get("maxLength");
            let max_length = match max_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

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

            let min_l = min_length.clone();
            let max_l = max_length.clone();
            let check_length = move |s: String| {
                let length = s.len();
                if length < (min_l.parse::<usize>().unwrap() - 1) {
                    println!("It's too low");
                    validation_message.set(("invalid-feedback", "Character too low"));
                } else if length > (max_l.parse::<usize>().unwrap() + 2) {
                    println!("you've exceeded your limit");
                    validation_message
                        .set(("invalid-feedback", "character exceeded stay within range"));
                }
            };

            let handle_blur = move || {
                let datum = sample_data.get().as_ref().clone();

                check_length(sample_data.get().as_ref().clone());

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
                             class="form-control", type="text", placeholder=default_value.clone(), required=false, pattern="", minlength=min_length, maxlength=max_length, size="",
                             readonly=false, disabled=false, autofocus=false, autocomplete=""){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        "array" => {
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
            let handle_blur = move || {
                let fname = &form_name;
                let datum = sample_data.get().as_ref().clone();

                let val: Value = serde_json::from_str(&format!("\"{}\"", datum)).unwrap();

                // let f_name: String = s.0.clone();
                let mut this_data = HashMap::new();
                this_data.insert(fname, val.to_owned());

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

                                div(class=""){
                                   // ul(class="list-group list-group-flush"){
                                Keyed(
                                   iterable=items_signal,
                                   view= move |cx, x|
                                    {
                                        let fname = form_name.clone();
                                        let key = x.get("key").unwrap().as_str().unwrap().to_owned();
                                        let value = x.get("name").unwrap().as_str().unwrap().to_owned();
                                            view! { cx,
                                        // input (class="form-check-input", type="radio", name="inlineRadioOptions", id="inlineRadio1", value="option1") {"---"}
                                        // li (class="list-group-item") { input (class="form-check-input me-1", type="radio") {} (y)}
                                         div(class="form-check"){
                                         input (class="form-check-input", id=key.clone(), name=fname, type="radio") {}
                                         label (class="form-check-label", for= x.get("key").unwrap().as_str().unwrap().to_owned()) {(value)}
                                         }
                                   }
                                    }
                                    ,
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
                        // div{(format!("{:#?}", items_signal.get()))}
                    }
            }
        }

        "checkbox" => {
            let binding = s.1.clone();
            let min_length = binding.get("minLength");

            // let min_length = match min_length {
            //     Some(x) => match x {
            //         Value::Number(n) => n.to_string(),
            //         _ => "".to_owned(),
            //     },
            //     None => "".to_owned(),
            // };

            let min_length = match min_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

            let max_length = binding.get("maxLength");
            let max_length = match max_length {
                Some(x) => match x {
                    Value::Number(n) => n.to_string(),
                    _ => "".to_owned(),
                },
                None => "".to_owned(),
            };

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

            let min_l = min_length.clone();
            let max_l = max_length.clone();
            let check_length = move |s: String| {
                let length = s.len();
                if length < (min_l.parse::<usize>().unwrap() - 1) {
                    println!("It's too low");
                    validation_message.set(("invalid-feedback", "Character too low"));
                } else if length > (max_l.parse::<usize>().unwrap() + 2) {
                    println!("you've exceeded your limit");
                    validation_message
                        .set(("invalid-feedback", "character exceeded stay within range"));
                }
            };

            let handle_blur = move || {
                let datum = sample_data.get().as_ref().clone();

                check_length(sample_data.get().as_ref().clone());

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
                             class="form-control", type="text", placeholder=default_value.clone(), required=false, pattern="", minlength=min_length, maxlength=max_length, size="",
                             readonly=false, disabled=false, autofocus=false, autocomplete=""){}

                         div(class=validation_message.get().0){(validation_message.get().1)}
                         div{(sample_data.get())}
                    }
            }
        }

        _ => view! {cx, div{}},
    };
    input_type
}
