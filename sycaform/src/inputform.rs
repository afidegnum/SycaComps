use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use sycamore::prelude::*;

use crate::widgets::checkbox::CheckBoxInput;
use crate::widgets::color::ColorInput;
use crate::widgets::datalist::DataList;
use crate::widgets::date::DateInput;
use crate::widgets::datetime::DateTimeInput;
use crate::widgets::email::EmailInput;
use crate::widgets::fileinput::FileInput;
use crate::widgets::integer::IntegerInput;
use crate::widgets::password::PassWordInput;
use crate::widgets::radio::RadioInput;
use crate::widgets::range::RangeInput;
use crate::widgets::search::SearchInput;
use crate::widgets::tel::TelelphoneInput;
use crate::widgets::textarea::TextArea;
use crate::widgets::textinput::TextInput;
use crate::widgets::url::UrlInput;

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
    let field_type =
        s.1.clone()
            .get("ui:widget")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();

    let y = s.clone();

    // log!(format!("{:?}", &field_type));
    let input_type = match field_type.as_ref() {
        "textinput" => TextInput(cx, y),
        "textarea" => TextArea(cx, y),
        "password" => PassWordInput(cx, y),
        "integer" => IntegerInput(cx, y),
        "range" => RangeInput(cx, y),
        "color" => ColorInput(cx, s),
        "email" => EmailInput(cx, y),
        "tel" => TelelphoneInput(cx, y),
        "date" => DateInput(cx, y),
        "datetime" => DateTimeInput(cx, y),
        "file" => FileInput(cx, y),
        "url" => UrlInput(cx, y),
        "radio" => RadioInput(cx, y),
        "checkbox" => CheckBoxInput(cx, y),
        "search" => SearchInput(cx, y),
        "datalist" => DataList(cx, y),
        _ => view! {cx, div{}},
    };
    input_type
}
