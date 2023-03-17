use std::collections::HashMap;

use serde_json::Value;
use sycamore::prelude::*;
pub mod widgets;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use serde_json::Map;
use widgets::formlayout::FormLayout;
use widgets::formresult::FormResult;

// macro_rules! node_ref {
//     ($($id:expr)*) => {
//         paste::paste! {
//             $(
//                 let [< $id >] = create_node_ref(cx);
//             )*
//         }
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Form {
    pub title: String,
    pub description: String,
    required: Vec<String>, // point Vec to properties field.
    properties: HashMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormExportData {
    pub field_data: HashMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormData {
    pub data: RcSignal<HashMap<String, Value>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FormState {
    pub formstate: RcSignal<Form>,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    // json_data update
    // let mut properties = Map::new();
    // properties.insert(
    //     "firstName".to_owned(),
    //     json!({
    //         "type": "string",
    //         "title": "First name",
    //         "default": "Chuck"
    //     }),
    // );

    // properties.insert(
    //     "age".to_owned(),
    //     json!({
    //         "type": "integer",
    //         "title": "Ente a number"
    //     }),
    // );

    // properties.insert(
    //     "datetime".to_owned(),
    //     json!({
    //         "type": "string",
    //         "title": "Enter a Date and Time"
    //     }),
    // );

    // properties.insert(
    //     "url".to_owned(),
    //     json!({
    //         "type": "string",
    //         "title": "Enter link"
    //     }),
    // );

    // properties.insert(
    //     "datalist".to_owned(),
    //     json!({
    //         "type": "datalist",
    //         "title": "Datalist: Search and/or select",
    //         "enum": [
    //             {
    //                 "name": "Option One",
    //                 "key": "o1",
    //             },
    //             {
    //                 "name": "Option 4",
    //                 "key": "o4"
    //             }
    //         ]
    //     }),
    // );

    // properties.insert(
    //     "checkboxes".to_owned(),
    //     json!({
    //         "type": "checkbox",
    //         "title": "select as many as possible",
    //         "enum": [
    //             {
    //                 "name": "Item One",
    //                 "key": "io",
    //                 "checked": false
    //             },
    //             {
    //                 "name": "Item 4",
    //                 "key": "ity",
    //                 "checked": false
    //             }
    //         ]
    //     }),
    // );

    // properties.insert(
    //     "color".to_owned(),
    //     json!({
    //         "type": "color",
    //         "title": "Choose A Color"
    //     }),
    // );

    // properties.insert(
    //     "interval".to_owned(),
    //     json!({
    //         "type": "range",
    //         "title": "Interval",
    //         "minimal": 0,
    //         "maximal": 100,
    //         "multipleOf": 2
    //     }),
    // );

    // properties.insert(
    //     "bio".to_owned(),
    //     json!({
    //         "type": "textarea",
    //         "title": "Bio",
    //         "minLength": 5,
    //         "maxLength": 10
    //     }),
    // );

    // properties.insert(
    //     "countries".to_owned(),
    //     json!({
    //         "type": "radio",
    //         "title": "Number enum",
    //         "enum": [
    //             {
    //                 "name": "New York",
    //                 "key": "ny"
    //             },
    //             {
    //                 "name": "Hong Kong",
    //                 "key": "hongkong"
    //             }
    //         ]
    //     }),
    // );

    // let json_data = json!({
    //     "title": "A registration form",
    //     "description": "A simple form example.",
    //     "type": "object",
    //     "required": ["firstName", "lastName"],
    //     "properties": properties
    // });

    // endof json_data udpate
    let json_data = json!({
      "title": "A registration form",
      "description": "A simple form example.",
      "type": "object",
      "required": [
        "firstName",
        "lastName"
      ],
      "properties": {
        "firstName": {
          "type": "string",
          "title": "First name",
          "default": "Chuck"
        },
        "lastName": {
          "type": "string",
          "title": "Last name"
        },
        "age": {
          "type": "integer",
          "title": "Ente a number"
        },
          "date": {
          "type": "date",
          "title": "Enter a Date"
        },
          "datetime": {
          "type": "datetime",
          "title": "Enter a Date and Time"
        },
          "file": {
          "type": "file",
          "title": "Upload a File",
          "accept": "",
          "multiple": false
        },
          "url": {
          "type": "url",
          "title": "Enter link"
        },
          "search": {
          "type": "search",
          "title": "Search"
        },
         "datalist": {
          "type": "datalist",
          "title": "Datalist: Search and/or select",
            "enum": [
            {
              "name": "Option One",
              "key": "o1",
            },
            {
              "name": "Option Two",
              "key": "o2",
            },
            {
              "name": "Option Tree",
              "key": "o3"
            },
            {
              "name": "Option 4",
              "key": "o4"
            }
            ]
        },
          "checkboxes": {
          "type": "checkbox",
          "title": "select as many as possible",
           "enum": [
            {
              "name": "Item One",
              "key": "io",
              "checked": false
            },
            {
              "name": "Item two",
              "key": "it",
              "checked": true

            },
            {
              "name": "Item 3",
              "key": "itx",
              "checked": false

            },
            {
              "name": "Item 4",
              "key": "ity",
              "checked": false

            }
            ]
        },
         "color": {
          "type": "color",
          "title": "Choose A Color"
        },
         "interval": {
          "type": "range",
          "title": "Interval",
           "minimal": 0,
           "maximal": 100,
           "multipleOf": 2

        },
        "bio": {
          "type": "textarea",
          "title": "Bio",
          "minLength": 5,
          "maxLength": 10,
        },
        "password": {
          "type": "password",
          "title": "Password",
          "minLength": 3
        },
        "telephone": {
          "type": "string",
          "title": "Telephone",
          "minLength": 5
        },
         "countries": {
         "type": "radio",
            "title": "Number enum",
            "enum": [
            {
              "name": "New York",
              "key": "ny",
            },
            {
              "name": "Amsterdam",
              "key": "amst",
            },
            {
              "name": "Hong Kong",
              "key": "hongkong"
            }

            ]
         },
      }
    }
    );

    let ui_schema = json!(
            {
      "firstName": {
        "ui:autofocus": true,
        "ui:emptyValue": "",
        "ui:placeholder": "ui:emptyValue causes this field to always be valid despite being required",
        "ui:autocomplete": "family-name"
      },
      "lastName": {
        "ui:autocomplete": "given-name"
      },
      "age": {
        "ui:widget": "updown",
        "ui:title": "Age of person",
        "ui:description": "(earth year)"
      },
      "bio": {
        "ui:widget": "textarea"
      },
      "password": {
        "ui:widget": "password",
        "ui:help": "Hint: Make it strong!"
      },
      "telephone": {
        "ui:options": {
          "inputType": "tel"
        }
      }
    }
        );

    let json_form: Form = serde_json::from_value(json_data).unwrap();
    println!("JSON form: {:?}", json_form);

    let required = &json_form.required;
    let mut properties = json_form.properties.clone();
    for (key, value) in properties.iter_mut() {
        if required.contains(key) {
            value["required"] = json!(true);
        }
    }

    let mut json_form = json_form;
    json_form.properties = properties;

    let form_state = FormState {
        formstate: create_rc_signal(json_form),
    };

    let form_data = FormData {
        data: create_rc_signal(HashMap::new()),
    };

    let new_state = form_state.clone();

    provide_context(cx, form_data);
    view! { cx,
            // Body Start
            nav (class="navbar navbar-expand-lg navbar-light bg-light") {div (class="container px-4 px-lg-5") {
                    a (class="navbar-brand", href="#!") {"Sycamore Data Form"}

                    button (class="navbar-toggler", type="button", data-bs-toggle="collapse", data-bs-target="#navbarSupportedContent", aria-controls="navbarSupportedContent", aria-expanded="false", aria-label="Toggle navigation") {span (class="navbar-toggler-icon") {}
    }

                    div (class="collapse navbar-collapse", id="navbarSupportedContent") {
                        ul (class="navbar-nav me-auto mb-2 mb-lg-0 ms-lg-4") {li (class="nav-item") {a (class="nav-link active", aria-current="page", href="#!") {"Home"} }

                            li (class="nav-item") {a (class="nav-link active", aria-current="page", href="#!") {"Nested"} }

                            li (class="nav-item") {a (class="nav-link active", aria-current="page", href="#!") {"Arrays"} }


                            li (class="nav-item") {
                                a (class="nav-link active", aria-current="page", href="#!") {"File"}
                            }

                        }
    }

                }


            }
    //
            section (class="py-5") {div (class="container px-4 px-lg-5 my-5") {
            h1 (class="display-3 text-center py-5") {"Dynamic Forms"}

            div (class="row gx-2 gx-lg-2 align-items-center") {
                div (class="col-md-4") {
                    div (class="card border-info") {
                        div (class="card-header") {"Data"}

                        div (class="card-body") {
                            h5 (class="card-title") {"Initial Form Data"}

                            pre (class="card-text")
                    {(format!("{:#?}", new_state.formstate.get())) }

                        }

                    }

                }

                //Insert Form

                FormLayout(form_state)

                //End of Form


                FormResult()

            }

        }

    }


                                                                         //



            // Body End
        }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
