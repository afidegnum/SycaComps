use std::collections::HashMap;
pub mod formlayout;
pub mod formresult;
pub mod inputform;
pub mod widgets;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Value};
use sycamore::prelude::*;
// use widgets::formlayout::FormLayout;
// use widgets::formresult::FormResult;
use formlayout::FormLayout;
use formresult::FormResult;

// macro_rules! node_ref {
//     ($($id:expr)*) => {
//         paste::paste! {
//             $(
//                 let [< $id >] = create_node_ref(cx);
//             )*
//         }
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Form {
//     pub title: String,
//     pub description: String,
//     required: Vec<String>, // point Vec to properties field.
//     properties: IndexMap<String, Value>,
//     // ui_schema: IndexMap<String, Value>,
// }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Form {
    schema: IndexMap<String, Value>,
    ui_schema: IndexMap<String, Value>,
    merged_schema: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormExportData {
    pub field_data: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormData {
    pub data: RcSignal<IndexMap<String, Value>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FormState {
    pub formstate: RcSignal<Form>,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    // endof json_data udpate
    let schema = json!({
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
          "type": "string",
          "title": "Enter a Date"
        },
          "datetime": {
          "type": "string",
          "title": "Enter a Date and Time"
        },
          "file": {
          "type": "string",
          "title": "Upload a File",
          "accept": "",
          "multiple": false
        },
          "url": {
          "type": "string",
          "title": "Enter link"
        },
          "search": {
          "type": "string",
          "title": "Search"
        },
         "datalist": {
          "type": "array",
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
          "type": "array",
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
          "type": "string",
          "title": "Choose A Color"
        },
         "interval": {
          "type": "integer",
          "title": "Interval",
           "minimal": 0,
           "maximal": 100,
           "multipleOf": 2

        },
        "bio": {
          "type": "string",
          "title": "Bio",
          "minLength": 5,
          "maxLength": 10,
        },
        "password": {
          "type": "string",
          "title": "Password",
          "minLength": 3
        },
        "telephone": {
          "type": "string",
          "title": "Telephone",
          "minLength": 5
        },
         "countries": {
         "type": "array",
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
        "type": "HorizontalLayout",
        "elements": [
        {
            "firstName": {
                "ui:autofocus": true,
                "ui:emptyValue": "",
                "ui:placeholder": "ui:emptyValue causes this field to always be valid despite being required",
                "ui:autocomplete": "family-name",
                "ui:widget": "textinput"

            },
            "lastName": {
                "ui:autocomplete": "given-name",
                 "ui:widget": "textinput"

            },
            "age": {
                "ui:widget": "updown",
                "ui:title": "Age of person",
                "ui:description": "(earth year)",
                 "ui:widget": "integer"

            },
            "date" : {"ui:widget" : "date"},
            "datetime" : {"ui:widget" : "datetime"},
            "file" : {"ui:widget" : "file"},
            "url" : {"ui:widget" : "url"},
             "search" : {"ui:widget" : "search"},
            "datalist" : {"ui:widget" : "datalist"},
            "checkboxes" : {"ui:widget" : "checkbox"},
            "color" : {"ui:widget" : "color"},
            "interval" : {"ui:widget" : "range"},
            "bio": {
                "ui:widget": "textarea"
            },
            "password": {
                "ui:widget": "password",
                "ui:help": "Hint: Make it strong!"
            },
            "telephone": {
                 "ui:widget": "textinput",
                "ui:options": {
                    "inputType": "tel"
                }
            },
             "countries" : {"ui:widget" : "radio"},

        }
        ]
    }
    );

    let required_properties = schema["required"]
        .as_array()
        .map(|a| a.iter().map(|v| v.as_str()).collect::<Option<Vec<&str>>>())
        .flatten()
        .unwrap_or_default();

    let modified_properties = schema["properties"]
        .as_object()
        .map(|o| {
            o.iter()
                .map(|(name, prop)| {
                    let mut prop = prop.clone();
                    if required_properties.contains(&name.as_str()) {
                        if let serde_json::Value::Object(ref mut obj) = prop {
                            obj.insert("required".to_owned(), serde_json::Value::Bool(true));
                        }
                    }
                    (name.to_owned(), prop)
                })
                .collect()
        })
        .unwrap_or_else(|| IndexMap::new());

    let modified_data = json!({
        "title": schema["title"],
        "description": schema["description"],
        "type": schema["type"],
        "required": schema["required"],
        "properties": modified_properties
    });

    let mut merged_schema = IndexMap::new();

    if let Some(properties) = modified_data.get("properties") {
        if let Some(properties_obj) = properties.as_object() {
            for (prop_name, prop_val) in properties_obj.iter() {
                if let Some(element) = ui_schema.get("elements") {
                    if let Some(element_arr) = element.as_array() {
                        for obj in element_arr.iter() {
                            if let Some(inner_obj) = obj.as_object() {
                                if let Some(field_val) = inner_obj.get(prop_name) {
                                    let mut merged_field = prop_val.as_object().unwrap().clone();
                                    let mut element_field = field_val.as_object().unwrap().clone();
                                    merged_field.append(&mut element_field);
                                    merged_schema
                                        .insert(prop_name.to_string(), Value::Object(merged_field));
                                } else {
                                    merged_schema.insert(prop_name.to_string(), prop_val.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let full_schema = Form {
        schema: serde_json::from_value(modified_data).unwrap(),
        ui_schema: serde_json::from_value(ui_schema).unwrap(),
        merged_schema,
    };

    // let required = &json_form.required;
    // let mut properties = json_form.properties.clone();
    // for (key, value) in properties.iter_mut() {
    //     if required.contains(key) {
    //         value["required"] = json!(true);
    //     }
    // }

    // let mut json_form = json_form;
    // json_form.properties = properties;

    // let ui_schema_map: IndexMap<String, Value> = serde_json::from_value(ui_schema).unwrap();
    // json_form.ui_schema = ui_schema_map;

    let form_state = FormState {
        formstate: create_rc_signal(full_schema),
    };

    let form_data = FormData {
        data: create_rc_signal(IndexMap::new()),
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
                    {(format!("{:#?}", new_state.formstate.get().merged_schema)) }

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
